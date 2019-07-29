//! `nom-both` is an extension of [nom](https://docs.rs/nom) to provide special `both_` parser.
//!
//! ## Examples
//!
//! The following example show a quick example.
//!
//! ```
//! use nom::branch::*;
//! use nom::bytes::complete::*;
//! use nom::IResult;
//! use nom_both::both_parser;
//!
//! #[both_parser]
//! pub fn both_opt_parser(s: &str) -> IResult<&str, Option<&str>> {
//!     let (s, _) = tag("1")(s)?;
//!     let (s, x) = both_opt(tag("2"))(s)?;
//!     let (s, _) = tag("2")(s)?;
//!     let (s, _) = tag("3")(s)?;
//!     Ok((s, x))
//! }
//!
//! #[both_parser]
//! pub fn both_alt_parser(s: &str) -> IResult<&str, &str> {
//!     let (s, _) = tag("1")(s)?;
//!     let (s, x) = both_alt(tag("22"), tag("2"))(s)?;
//!     let (s, _) = tag("2")(s)?;
//!     let (s, _) = tag("3")(s)?;
//!     Ok((s, x))
//! }
//!
//! fn main() {
//!     let ret = both_opt_parser("123");
//!     assert_eq!("None", format!("{:?}", ret.unwrap().1));
//!
//!     let ret = both_alt_parser("1223");
//!     assert_eq!("\"2\"", format!("{:?}", ret.unwrap().1));
//! }
//! ```

use nom::IResult;
pub use nom_both_macros::both_parser;

pub fn some<'a, I, O, F>(f: F) -> impl Fn(I) -> IResult<I, Option<O>>
where
    F: Fn(I) -> IResult<I, O>,
{
    move |i: I| {
        let (i, x) = f(i)?;
        Ok((i, Some(x)))
    }
}

pub fn none<'a, I, O, F>(_f: F) -> impl Fn(I) -> IResult<I, Option<O>>
where
    F: Fn(I) -> IResult<I, O>,
{
    move |i: I| Ok((i, None))
}

pub fn alt_left<'a, I, O, F, G>(f: F, _g: G) -> impl Fn(I) -> IResult<I, O>
where
    F: Fn(I) -> IResult<I, O>,
    G: Fn(I) -> IResult<I, O>,
{
    move |i: I| {
        let (i, x) = f(i)?;
        Ok((i, x))
    }
}

pub fn alt_right<'a, I, O, F, G>(_f: F, g: G) -> impl Fn(I) -> IResult<I, O>
where
    F: Fn(I) -> IResult<I, O>,
    G: Fn(I) -> IResult<I, O>,
{
    move |i: I| {
        let (i, x) = g(i)?;
        Ok((i, x))
    }
}
