mod std_impls;

/// Types that can be incremented or decremented by a unit value or a
/// given value.
pub trait Step: Sized {
    /// Return the next sequential value for the type.
    fn next(&self) -> Option<Self>;
    /// Return the value a given amount after the value.
    fn next_by(&self, by: &Self) -> Option<Self>;
    /// Return the previous sequential value for the type.
    fn prev(&self) -> Option<Self>;
    /// Return the value a given amount before the value.
    fn prev_by(&self, by: &Self) -> Option<Self>;
}
