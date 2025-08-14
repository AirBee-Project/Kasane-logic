use crate::id::{DimensionRange, SpaceTimeId};
use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

impl SpaceTimeId {
    /// Converts the spatial and temporal resolution of the current `SpaceTimeId` to a finer zoom level `z`
    /// and/or a finer time interval `i`, while preserving the physical coverage.
    ///
    /// # Arguments
    ///
    /// * `z` - Optional target zoom level. Must be greater than or equal to the current zoom level.
    /// * `i` - Optional target time interval. Must divide the current interval (`self.i`) and be less than or equal to it.
    ///
    /// # Returns
    ///
    /// Returns a new `SpaceTimeId` instance with adjusted spatial and/or temporal resolution.
    /// If both `z` and `i` are `None`, returns the original ID.
    ///
    /// # Notes
    ///
    /// When a new time interval `i` is specified, the scaling of the time dimension is calculated using the
    /// greatest common divisor (GCD) of the current interval (`self.i`) and the new interval. This ensures
    /// alignment of time boundaries while increasing temporal resolution.
    ///
    /// For example, if `self.i` is 60 and `i` is 15, the GCD is 15, and the time range is expanded by a factor of `60 / 15 = 4`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - `z` is less than the current zoom level
    /// - `z >= 32` (to prevent bit overflow)
    /// - `i` is less than the current time level
    /// - Internal coefficient conversion fails

    pub fn change_scale(&self, z: Option<u16>, i: Option<u32>) -> Result<Self, String> {
        if z.is_none() && i.is_none() {
            return Ok(self.clone());
        }

        // Validate zoom level
        if let Some(z) = z {
            if z < self.z {
                return Err(
                    "Target zoom level must be finer than or equal to the current zoom level."
                        .to_string(),
                );
            }
            if z >= 32 {
                return Err(format!(
                    "Zoom level z must be less than 32 to prevent overflow. Received: {}.",
                    z
                ));
            }
        }

        let mut f = self.f.clone();
        let mut x = self.x.clone();
        let mut y = self.y.clone();
        let mut t = self.t.clone();

        let z = match z {
            Some(z_new) => {
                if self.z == z_new {
                    z_new
                } else {
                    let diff = z_new
                        .checked_sub(self.z)
                        .ok_or("Zoom level subtraction error")?;
                    let xyf_coef = 2_u32.pow(diff as u32);
                    x = Self::change_scale_logic(&self.x, &xyf_coef)?;
                    y = Self::change_scale_logic(&self.y, &xyf_coef)?;
                    f = Self::change_scale_logic(&self.f, &xyf_coef)?;
                    z_new
                }
            }
            None => self.z,
        };

        let i = match i {
            Some(other_i) => {
                if self.i == !0 && other_i == 0 {
                    //時空間IDを空間IDに変換しようとしている場合
                    Err("時空間IDを空間IDに変換することはできないよ".to_string());
                } else if self.i == 0 && other_i == !0 {
                    //空間IDを時空間IDに変換しようとしている場合
                    t = DimensionRange::Any;
                    other_i
                } else if self.i == !0 && other_i == !0 {
                    //時空間IDを時空間IDに変換しようとしている場合
                } else {
                    //空間IDを空間IDに変換しようとしている場合
                    t = DimensionRange::Any;
                    other_i
                }
            }
            None => self.i,
        };

        Ok(Self::new(z, f, x, y, i, t)?)
    }

    fn change_scale_logic<T, Y>(
        range: &DimensionRange<T>,
        k: &Y,
    ) -> Result<DimensionRange<T>, String>
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + From<u8>,
        Y: Copy + Debug + TryInto<T>,
        <Y as TryInto<T>>::Error: Debug,
    {
        let one = T::from(1);

        let k_t: T = (*k)
            .try_into()
            .map_err(|e| format!("Failed to convert scale coefficient: {:?}", e))?;

        let scaled = match range {
            DimensionRange::Single(v) => {
                let start = *v * k_t;
                let end = (*v + one) * k_t - one;
                DimensionRange::LimitRange(start, end)
            }
            DimensionRange::LimitRange(s, e) => {
                let start = *s * k_t;
                let end = (*e + one) * k_t - one;
                DimensionRange::LimitRange(start, end)
            }
            DimensionRange::AfterUnLimitRange(v) => {
                let start = *v * k_t;
                DimensionRange::AfterUnLimitRange(start)
            }
            DimensionRange::BeforeUnLimitRange(v) => {
                let end = (*v + one) * k_t - one;
                DimensionRange::BeforeUnLimitRange(end)
            }
            DimensionRange::Any => DimensionRange::Any,
        };

        Ok(scaled)
    }

    pub fn gcd(mut a: u32, mut b: u32) -> u32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}
