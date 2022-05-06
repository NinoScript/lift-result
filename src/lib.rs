//! Lifts a fallible function that fails with error E1, into one that fails with error E2.
//!
//! I had my own `Error` type, turning another result into mine was easy and nice
//!
//! ```rust
//! result.map_err(Error::from)
//! ```
//!
//! But when I wanted to apply a fallible function via `.and_then`, I had to do this:
//!
//! ```rust
//! result.and_then(|x| failable(x).map_err(|e| e.into()))
//! ```
//!
//! And I didn't like that:
//! * too verbose
//! * not very readable
//! * the compiler should be able to know how to do that
//!
//! So I wrote this library.
//! Yay, programming! 🎉
//!
//! # Examples
//!
//! ```rust
//! result
//!     .map_err(Error::from)
//!     .and_then(lift(failable))
//! ```
//!
//! # Special thanks
//! Thank you [cargo-readme](https://github.com/livioribeiro/cargo-readme) for generating this README for me.
//!

/// Lifts a fallible function that fails with error E1, into one that fails with error E2.
///
/// # Generic types
///
/// * `F`: The function to lift.
/// * `I`: The input type of the function.
/// * `O`: The output type of the function.
/// * `E1`: The error type that the function can fail with.
/// * `E2`: The error type that the lifted function will fail with.
///
/// # Examples
///
/// ```rust
/// result
///     .map_err(Error::from)
///     .and_then(lift(failable))
/// ```
///
pub fn lift<F, I, O, E1, E2>(f: F) -> impl Fn(I) -> Result<O, E2>
where
    F: Fn(I) -> Result<O, E1>,
    E2: From<E1>,
{
    move |t| f(t).map_err(E2::from)
}

#[cfg(test)]
mod tests {
    use super::lift;
    #[derive(Debug)]
    struct Error1;
    #[derive(Debug)]
    struct Error2(Error1);

    impl From<Error1> for Error2 {
        fn from(error: Error1) -> Self {
            Self(error)
        }
    }

    fn fallible(i: &str) -> Result<i32, Error1> {
        i.parse().map_err(|_| Error1)
    }

    #[test]
    fn it_works() {
        let string_result = Result::<&str, Error2>::Ok("42");
        let i32_result = string_result.and_then(lift(fallible));
        assert_eq!(i32_result.unwrap(), 42);
    }
}
