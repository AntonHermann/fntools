#![cfg_attr(not(feature = "stable"), feature(unboxed_closures, fn_traits))]

/// Features that uses nightly-only unstable API
#[cfg(not(feature = "stable"))]
pub mod unstable;
/// Extension for all types
pub mod value;
/// Functions that return functions that return constants
pub mod constant;

/// Helper for `unstable` things
pub mod auto_tuple;
/// Helper for `unstable` things
pub mod flip_tuple;

pub mod prelude {
    pub use crate::{value::ValueExt, chain};
}

mod macro_def;

/// Compose two functions.
///
/// Takes functions `f` and `g` and returns `f ∘ g = |a: A| f(g(a))`.
///
/// # Examples
/// ```
/// use fntools::compose;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = compose(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// Note the order:
/// ```
/// use fntools::compose;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = compose(compose(to_64, to_32), to_16);
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// See also:
/// - [`unstable::compose`]
/// - [`fntools::chain`]
///
/// [`unstable::compose`]: crate::unstable::compose::compose
/// [`fntools::chain`]: crate::chain
pub fn compose<A, B, C, F, G>(f: F,  g: G) -> impl Fn(A) -> C
    where
        G: Fn(A) -> B,
        F: Fn(B) -> C,
{
    move |a: A| f(g(a))
}

/// Chain two functions.
///
/// Takes functions `f` and `g` and returns `g ∘ f = |a: A| g(f(a))`.
///
/// # Examples
/// ```
/// use fntools::chain;
///
/// let add_two = |a: i32| a + 2;
/// let add_three = |a: i32| a + 3;
/// let add_five = chain(add_two, add_three);
///
/// assert_eq!(add_five(4), 9);
/// ```
///
/// Note the order:
/// ```
/// use fntools::chain;
///
/// let to_16 = |i: i8| i16::from(i);
/// let to_32 = |i: i16| i32::from(i);
/// let to_64 = |i: i32| i64::from(i);
///
/// // execution order: to_16 -> to_32 -> to_64
/// let i8_to_i64 = chain(to_16, chain(to_32, to_64));
///
/// assert_eq!(i8_to_i64(8i8), 8i64);
/// ```
///
/// See also:
/// - [`unstable::chain`]
/// - [`fntools::compose`]
///
/// [`unstable::chain`]: crate::unstable::chain::chain
/// [`fntools::compose`]: crate::compose
pub fn chain<A, B, C, F, G>(f: F,  g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |a: A| g(f(a))
}

/// Flip function arguments.
/// 
/// # Examples
/// ```
/// use fntools::flip_args;
/// 
/// let fun = |a: &str, b: i32| format!("{}{}", a, b);
/// let fun = flip_args(fun);
/// 
/// assert_eq!(fun(17, "hello, "), "hello, 17")
/// ```
pub fn flip_args<A, B, R, F>(f: F) -> impl FnOnce(B, A) -> R
where
    F: FnOnce(A, B) -> R
{
    move |b: B, a: A| f(a, b)
}

/// Cartesian product of functions.
///
/// Takes functions `f` and `g` and returns `g × f = |a: A, x: X| (f(a), g(x))`.
///
/// ## Example
/// ```
/// use fntools::product;
///
/// // TODO: better example
/// let string = "привет";
/// let (slice, str) = product(<[_]>::len, str::len)(string.as_bytes(), string);
/// assert_eq!(slice, 12);
/// assert_eq!(str, 12);
/// ```
pub fn product<A, B, X, Y, F, G>(f: F, g: G) -> impl Fn(A, X) -> (B, Y)
where
    F: Fn(A) -> B,
    G: Fn(X) -> Y,
{
    move |a: A, x: X| (f(a), g(x))
}
