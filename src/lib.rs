//! Lazily initialized values for Rust
//! 
//! This library basically ports the .NET Lazy<T> class straight
//! into Rust, permitting the use of values that aren't initialized
//! until (if) they are called for by your code.
//! 
//! # Example
//! 
//! ```
//! use lazy::Lazy;
//! 
//! let mut lazy_5 = Lazy::new(|| 5);
//! 
//! assert!(lazy_5.value() == &5);
//! ```

/// Stores a function required to create a lazy value
/// when the value is required.
pub struct Lazy<T, F>
    where F: FnMut() -> T
{
    t: Option<T>,
    f: F,
}

impl<T, F> Lazy<T, F>
    where F: Fn() -> T
{
    /// Creates a new lazy value to be initialized
    /// using the provided function.
    pub fn new(f: F) -> Lazy<T, F> {
        Lazy {
            t: None,
            f: f,
        }
    }

    // Returns a reference to the  lazily-initialized
    // value created by the function used to create
    // provided to the constructor. I think this is neat
    // because it's recursive.
    pub fn value(&mut self) -> &T {
        match self.t {
            Some(ref t) => &t,
            None => {
                self.t = Some((self.f)());
                self.value()
            }
        }
    }
}

#[cfg(test)]
mod lazy_tests {
    use super::Lazy;

    #[test]
    fn lazy_value_works() {
        let mut lazy_5 = Lazy::new(|| 5);
        assert!(lazy_5.value() == &5);
    }

    #[test]
    fn lazy_value_works_multiple_times() {
        let mut lazy_5 = Lazy::new(|| 5);
        assert!(lazy_5.value() == &5);
        assert!(lazy_5.value() == &5);
    }
}
