//! `Step` is a trait that allows for stepping numeric values and makes
//! writing functions that are generic over numeric types easier.
//!
//! # Example
//!
//! ```
//! use step::Step;
//!
//! let number = 42;
//! println!("The number is: {}", number);
//! println!("The previous number is: {}", number.prev().unwrap());
//! println!("The next number is: {}", number.next().unwrap());
//! ```

mod std_impls;

/// Types that can be incremented or decremented by a unit value or a
/// given value.
pub trait Step: Sized {
    /// Return the next sequential value for the type
    fn next(&self) -> Option<Self>;
    /// Return the value a given amount after the value
    fn next_by(&self, by: &Self) -> Option<Self>;
    /// Return the previous sequential value for the type
    fn prev(&self) -> Option<Self>;
    /// Return the value a given amount before the value
    fn prev_by(&self, by: &Self) -> Option<Self>;
    /// Return the steps to another value
    ///
    /// The return value will be the number of steps between the two
    /// values (i.e. it will always be a positive number).
    fn steps_to(&self, value: &Self) -> Self;
}
