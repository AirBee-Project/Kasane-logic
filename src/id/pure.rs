use crate::id::DimensionRange::{AfterUnLimitRange, Any, BeforeUnLimitRange, LimitRange, Single};
use crate::id::{DimensionRange, SpaceTimeId};

pub struct PureSpaceTimeId {
    pub z: u8,
    pub f: i32,
    pub x: u32,
    pub y: u32,
    pub i: u32,
    pub t: u32,
}

impl SpaceTimeId {
    /// Expands all dimension ranges into individual `SpaceTimeId` instances with single values only.
    ///
    /// This method converts extended notation (Range, Before, After, Any) into a collection of
    /// pure space-time IDs where each dimension uses only `Single` variants. This is useful
    /// for processing operations that require discrete, individual space-time cells.
    ///
    /// # Returns
    ///
    /// A `Vec<SpaceTimeId>` containing all individual IDs that represent the same space-time
    /// region as the original ID, but with each dimension expanded to single values.
    ///
    /// # Note
    ///
    /// The T dimension is preserved as-is and not expanded, maintaining the original temporal range.
    ///
    /// # Japanese Note
    ///
    /// 拡張記法 (Range, Before, After, Any) をすべて展開して
    /// 各次元が Single だけの純粋な ID 群を返す
    /// Expands all dimension ranges into individual `PureSpaceTimeId` instances.
    /// Expands all dimension ranges into individual `PureSpaceTimeId` instances.
    pub fn pure(&self) -> Vec<PureSpaceTimeId> {
        let z = self.z;
        let i = self.i;

        let max_xy = (1u64 << z) - 1;
        let max_f = (1i64 << z) - 1;
        let min_f = -(1i64 << z);

        // u32/u64 の DimensionRange を展開
        let expand_u32 = |range: &DimensionRange<u32>, max: u32| -> Vec<u32> {
            match range {
                Single(v) => vec![*v],
                LimitRange(s, e) => (*s..=*e).collect(),
                BeforeUnLimitRange(e) => (0..=*e).collect(),
                AfterUnLimitRange(s) => (*s..=max).collect(),
                Any => (0..=max).collect(),
            }
        };

        // i32/i64 の DimensionRange を展開
        let expand_i32 = |range: &DimensionRange<i32>, min: i32, max: i32| -> Vec<i32> {
            match range {
                Single(v) => vec![*v],
                LimitRange(s, e) => (*s..=*e).collect(),
                BeforeUnLimitRange(e) => (min..=*e).collect(),
                AfterUnLimitRange(s) => (*s..=max).collect(),
                Any => (min..=max).collect(),
            }
        };

        let x_vals = expand_u32(&self.x, max_xy as u32);
        let y_vals = expand_u32(&self.y, max_xy as u32);
        let f_vals = expand_i32(&self.f, min_f as i32, max_f as i32);

        // 時間 t の単一値展開（i == 0 の場合 Any）
        let t_vals = match &self.t {
            Single(v) => vec![*v],
            LimitRange(s, e) => vec![*s], // 展開せず先頭のみ
            BeforeUnLimitRange(v) => vec![*v],
            AfterUnLimitRange(v) => vec![*v],
            Any => vec![0],
        };

        let mut result =
            Vec::with_capacity(x_vals.len() * y_vals.len() * f_vals.len() * t_vals.len());

        for &x in &x_vals {
            for &y in &y_vals {
                for &f in &f_vals {
                    for &t in &t_vals {
                        result.push(PureSpaceTimeId { z, f, x, y, i, t });
                    }
                }
            }
        }

        result
    }
}
