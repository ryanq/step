mod std_impls;

/// Types that can be incremented or decremented by a unit value or a
/// given value.
pub trait Step: Sized {
    /// Increment the value by the unit value for the type.
    fn step(&self) -> Option<Self>;
    /// Increment the value by a given value.
    fn step_by(&self, by: &Self) -> Option<Self>;
    /// Decrement the value by the unit value for the type.
    fn step_back(&self) -> Option<Self>;
    /// Decrement the value by a given value.
    fn step_back_by(&self, by: &Self) -> Option<Self>;
}
