// ! #ADD ONE
// ! `ADD ONE` is a test crate

/// Adds one to the number given.
///
/// # Example
/// ```
/// let a = add_one::add_one(2);
/// assert_eq!(a, 3);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
pub mod tests{
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(2), 3);
    }
}