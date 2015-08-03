//! The `testing` crate provides functions that add numbers to other numbers.
//!
//! Note the //! being used for module level comments.
//! The test below will be found and run by cargo test
//! # Examples
//!
//! ```
//! assert_eq!(4, testing::add_two(2));
//! ```

/// This function adds two to its argument.
///
/// Note the /// being used for function level comments
/// The test below will be found and run by cargo test
/// # Examples
///
/// ```
/// use testing::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

//cfg attribute means only compile when doing: cargo test
#[cfg(test)]
mod tests {
	//Common idiom to match all functions in the parent module
	//Could be: use super::add_two;
    use super::*; 

	#[test]
	fn no_panic() {
		assert!(true);
	}
	
	//Expecting a failure, the expected argument is not required but is 
	//a bit safer as it rules out other types of failures
	#[test]
	#[should_panic(expected = "assertion failed")]
	fn should_panic() {
		assert_eq!("Hello", "world");
	}

    #[test]
    fn two_and_two_is_four() {
        assert_eq!(4, add_two(2));
    }
}