//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// # Panics
/// The scenarios in which the function being documented could panic.
///
/// # Errors
///
///  kinds of errors that might occur and what conditions might cause those errors to be returned
///
/// # Safety
///
/// why the function is unsafe and covering the invariants that the function expects callers to uphold.
///
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
