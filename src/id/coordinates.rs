use crate::id::{DimensionRange, SpaceTimeId};
use std::f64::consts::PI;

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
#[derive(Debug)]
pub struct Coordinates {
    pub latitude: (f64, f64),
    pub longitude: (f64, f64),
    pub altitude: (f64, f64),
}

#[cfg_attr(
    feature = "serde_support",
    derive(serde::Serialize, serde::Deserialize, schemars::JsonSchema)
)]
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

impl SpaceTimeId {
    /// Converts the `SpaceTimeId` into geographic coordinates (latitude, longitude, and altitude).
    ///
    /// This method interprets the spatial and vertical components of the ID.
    ///
    /// # Returns
    /// A [`Coordinates`] struct containing the latitude, longitude, and altitude ranges
    /// as floating-point tuples `(start, end)`, representing the spatial extent.

    pub fn coordinates(&self) -> Coordinates {
        let n = 2_u64.pow(self.z as u32);

        let longitude = Self::map_range_u64(&self.x, n, Self::longitude);
        let latitude = Self::map_range_u64(&self.y, n, Self::latitude);
        let altitude = Self::map_range_i64(&self.f, n, Self::altitude);

        Coordinates {
            latitude,
            longitude,
            altitude,
        }
    }

    fn longitude(x: u64, n: u64) -> f64 {
        360.0 * (x as f64 / n as f64) - 180.0
    }

    fn latitude(y: u64, n: u64) -> f64 {
        let y_f64 = y as f64;
        let n_f64 = n as f64;
        let exponent = (1.0 - 2.0 * y_f64 / n_f64) * PI;
        let lat_rad = 2.0 * (1.0 - 2.0 / (exponent.exp() + 1.0)).atan();
        lat_rad.to_degrees()
    }

    fn altitude(f: i64, n: u64) -> f64 {
        let f64_val = f as f64;
        let n64_val = n as f64;
        33_554_432.0 * (f64_val / n64_val)
    }

    fn map_range_u64<F>(range: &DimensionRange<u64>, n: u64, func: F) -> (f64, f64)
    where
        F: Fn(u64, u64) -> f64,
    {
        let max_val = n;
        match *range {
            DimensionRange::Single(v) => (func(v, n), func(v + 1, n)),
            DimensionRange::LimitRange(start, end) => (func(start, n), func(end + 1, n)),
            DimensionRange::BeforeUnLimitRange(end) => (func(0, n), func(end + 1, n)),
            DimensionRange::AfterUnLimitRange(start) => (func(start, n), func(max_val, n)),
            DimensionRange::Any => (func(0, n), func(max_val, n)),
        }
    }

    fn map_range_i64<F>(range: &DimensionRange<i64>, n: u64, func: F) -> (f64, f64)
    where
        F: Fn(i64, u64) -> f64,
    {
        let max_val = n as i64;
        match *range {
            DimensionRange::Single(v) => (func(v, n), func(v + 1, n)),
            DimensionRange::LimitRange(start, end) => (func(start, n), func(end + 1, n)),
            DimensionRange::BeforeUnLimitRange(end) => (func(-max_val, n), func(end + 1, n)),
            DimensionRange::AfterUnLimitRange(start) => (func(start, n), func(max_val, n)),
            DimensionRange::Any => (func(-max_val, n), func(max_val, n)),
        }
    }
}
