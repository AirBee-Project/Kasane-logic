use crate::id::{DimensionRange, SpaceTimeId};

impl SpaceTimeId {
    /// Returns the F dimension (vertical/altitude) value.
    ///
    /// # Returns
    ///
    /// The F coordinate as a `DimensionRange<i64>`, representing the vertical position
    /// or range along the F-axis (altitude).
    ///
    /// # Example
    ///
    /// ```
    /// use logic::id::{DimensionRange, SpaceTimeId};
    ///
    /// let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
    ///                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
    /// let f_value = stid.f();
    /// assert_eq!(f_value, DimensionRange::Single(5));
    /// ```
    pub fn f(&self) -> DimensionRange<i64> {
        self.f
    }
    /// Returns the X dimension value.
    ///
    /// # Returns
    ///
    /// The X coordinate as a `DimensionRange<u64>`, representing the spatial position
    /// or range along the X-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use logic::id::{DimensionRange, SpaceTimeId};
    ///
    /// let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
    ///                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
    /// let x_value = stid.x();
    /// assert_eq!(x_value, DimensionRange::Single(3));
    /// ```
    pub fn x(&self) -> DimensionRange<u64> {
        self.x
    }

    /// Returns the Y dimension value.
    ///
    /// # Returns
    ///
    /// The Y coordinate as a `DimensionRange<u64>`, representing the spatial position
    /// or range along the Y-axis.
    ///
    /// # Example
    ///
    /// ```
    /// use logic::id::{DimensionRange, SpaceTimeId};
    ///
    /// let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
    ///                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
    /// let y_value = stid.y();
    /// assert_eq!(y_value, DimensionRange::Single(10));
    /// ```
    pub fn y(&self) -> DimensionRange<u64> {
        self.y
    }

    /// Returns the T dimension (time index) value.
    ///
    /// # Returns
    ///
    /// The T coordinate as a `DimensionRange<u32>`, representing the temporal position
    /// or range along the T-axis (time index).
    ///
    /// # Example
    ///
    /// ```
    /// use logic::id::{DimensionRange, SpaceTimeId};
    ///
    /// let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
    ///                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
    /// let t_value = stid.t();
    /// assert_eq!(t_value, DimensionRange::Single(100));
    /// ```
    pub fn t(&self) -> DimensionRange<u32> {
        self.t
    }

    /// Returns the zoom level.
    ///
    /// # Returns
    ///
    /// The zoom level as a `u16`, which determines the spatial resolution and
    /// coordinate space boundaries.
    ///
    /// # Example
    ///
    /// ```
    /// use logic::id::{DimensionRange, SpaceTimeId};
    ///
    /// let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
    ///                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
    /// let zoom_level = stid.z();
    /// assert_eq!(zoom_level, 4);
    /// ```
    pub fn z(&self) -> u16 {
        self.z
    }

    /// Returns the time interval.
    ///
    /// # Returns
    ///
    /// The time interval as a `u32`, representing the temporal resolution in seconds.
    /// When `i` is 0, this indicates a spatial ID that is valid for all time.
    ///
    /// # Example
    ///
    /// ```
    /// use logic::id::{DimensionRange, SpaceTimeId};
    ///
    /// let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
    ///                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
    /// let time_interval = stid.i();
    /// assert_eq!(time_interval, 60);
    /// ```
    pub fn i(&self) -> u32 {
        self.i
    }
}
