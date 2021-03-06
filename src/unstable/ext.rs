use crate::{
    tuple::{flip::FlipTuple, take::TupleTake},
    unstable::{
        chain::{chain, Chain},
        compose::{compose, Compose},
        curry::{curry, Curry},
        flip::{flip, Flip},
        supply::{supply, Supply},
        unit::{unit, Unit},
        untuple::{untuple, Untuple},
    },
};

/// Extensions for Fn* types.
///
/// Provide shortcuts for
/// - [`chain`]
/// - [`chain`] + [`untuple`]
/// - [`compose`]
/// - [`compose`] + [`untuple`]
/// - [`supply`]
/// - [`flip`]
/// - [`curry`]
///
/// [`chain`]: crate::unstable::chain
/// [`untuple`]: crate::unstable::untuple
/// [`compose`]: crate::unstable::compose
/// [`supply`]: crate::unstable::supply
/// [`flip`]: crate::unstable::flip
/// [`curry`]: crate::unstable::curry
pub trait FnExt<Args>: Sized {
    /// Chain two functions (`g ∘ self`)
    ///
    /// # Examples:
    /// ```
    /// use fntools::unstable::FnExt;
    ///
    /// let add_two = |a: i32| a + 2;
    /// let add_three = |a: i32| a + 3;
    /// #[rustfmt::skip]
    /// let add_eight = add_two
    ///     .chain(add_three)
    ///     .chain(add_three);
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    ///
    /// For more info see [`chain`]
    ///
    /// [`chain`]: crate::unstable::chain
    #[inline]
    fn chain<G>(self, g: G) -> Chain<Self, G>
    where
        Self: FnOnce<Args>,
        G: FnOnce<(Self::Output,)>,
    {
        chain(self, g)
    }

    /// Chain two functions (`g ∘ self`) **u**n**t**upling result of the first
    /// (`self`).
    ///
    /// # Examples:
    /// ```
    /// use fntools::unstable::FnExt;
    ///
    /// let tuple = |a| (a, 8);
    /// let add_eight = tuple.chain_ut(|a, b| a + b);
    /// //                              ^^^^---- note: no destructing (`|(a, b)|`)
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    #[inline]
    fn chain_ut<G>(self, g: G) -> Chain<Self, Untuple<G>>
    where
        Self: FnOnce<Args>,
        G: FnOnce<Self::Output>,
    {
        self.chain(untuple(g))
    }

    /// Compose two functions (`self ∘ g`)
    ///
    /// # Examples:
    /// ```
    /// use fntools::unstable::{compose, FnExt};
    ///
    /// let add_two = |a: i32| a + 2;
    /// let add_three = |a: i32| a + 3;
    /// #[rustfmt::skip]
    /// let add_eight = add_two
    ///     .compose(add_three)
    ///     .compose(add_three);
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    ///
    /// For more info see [`compose`]
    ///
    /// [`compose`]: crate::unstable::compose
    #[inline]
    fn compose<A, G>(self, g: G) -> Compose<Self, G>
    where
        Self: FnOnce<(G::Output,)>,
        G: FnOnce<A>,
    {
        compose(self, g)
    }

    /// Compose two functions (`self ∘ g`) **u**n**t**upling result of the first
    /// (`g`)
    ///
    /// # Examples:
    /// ```
    /// use fntools::unstable::FnExt;
    ///
    /// let tuple = |a| (a, 8);
    /// let add = |a, b| a + b;
    /// //         ^^^^---- note: no destructing (`|(a, b)|`)
    /// let add_eight = add.compose_ut(tuple);
    ///
    /// assert_eq!(add_eight(4), 12);
    /// ```
    ///
    /// ```
    /// use fntools::unstable::FnExt;
    ///
    /// // very bad impl of `checked_add`
    /// let my_checked_add =  (|res, over| if over { None } else { Some(res) }).compose_ut(i32::overflowing_add);
    /// //                      ^^^^^^^^^`\                           return `(i32, bool)` ---- ^^^^^^^^^^^^^^^
    /// //                                 `---- note: no destructing needed
    ///
    /// assert_eq!(my_checked_add(8, 16), Some(24));
    /// //                        ^^^^^--- takes 2 parameters as `i32::overflowing_add`
    /// assert_eq!(my_checked_add(std::i32::MAX, 1), None);
    /// ```
    ///
    /// For more info see [`compose`]
    ///
    /// [`compose`]: crate::unstable::compose
    #[inline]
    fn compose_ut<A, G>(self, g: G) -> Compose<Untuple<Self>, G>
    where
        Self: FnOnce<Args>,
        G: FnOnce<A, Output = Args>,
    {
        compose(untuple(self), g)
    }

    /// Supply argument to function.
    ///
    /// ## Example
    /// ```
    /// use fntools::unstable::{supply, FnExt};
    ///
    /// let fun = |a: i32, b: usize, c: String| format!("a: {}, b: {}, c: {:?}", a, b, c);
    /// #[rustfmt::skip]
    /// let fun = fun
    ///             .supply(8)
    ///             .supply(16)
    ///             .supply(String::from("AAA"));
    ///
    /// assert_eq!(fun(), "a: 8, b: 16, c: \"AAA\"")
    /// ```
    #[inline]
    fn supply(self, argument: Args::Take) -> Supply<Args::Take, Self, Args>
    where
        Self: FnOnce<Args>,
        Args: TupleTake,
    {
        supply(self, argument)
    }

    /// Flips argument order of `self`.
    ///
    /// # Example
    /// ```
    /// use fntools::unstable::FnExt;
    ///
    /// let fun = |a: &str, b: i32, c: char| format!("{}{}{}", a, b, c);
    /// let fun = fun.flip();
    ///
    /// assert_eq!(fun('c', 17, "hello, "), "hello, 17c")
    /// ```
    #[inline]
    fn flip(self) -> Flip<Self>
    where
        Self: FnOnce<Args>,
        Args: FlipTuple,
    {
        flip(self)
    }

    /// Curring.
    ///
    /// ## Examples
    /// ```
    /// use fntools::unstable::FnExt;
    /// use std::ops::Add;
    ///
    /// let fun = i32::add.curry();
    /// let res = fun(2)(2);
    /// assert_eq!(res, 4);
    /// ```
    #[inline]
    fn curry(self) -> Curry<(), Self, Args>
    where
        Self: FnOnce<Args>,
    {
        curry(self)
    }

    /// Unit function output
    ///
    /// ## Examples
    ///
    /// ```
    /// use fntools::unstable::FnExt;
    /// use std::ops::Sub;
    ///
    /// let fun = i32::sub.unit();
    /// assert_eq!(fun(2, 1), ());
    /// ```
    #[inline]
    fn unit(self) -> Unit<Self>
    where
        Self: FnOnce<Args>,
    {
        unit(self)
    }
}

impl<A, F> FnExt<A> for F
where
    F: FnOnce<A>,
{
    /* use default implementations */
}
