# nom-both
Extension of [nom](https://github.com/Geal/nom) to provide special `both_` parsers.

[![Build Status](https://dev.azure.com/dalance/nom-both/_apis/build/status/dalance.nom-both?branchName=master)](https://dev.azure.com/dalance/nom-both/_build/latest?definitionId=1&branchName=master)
[![Crates.io](https://img.shields.io/crates/v/nom-both.svg)](https://crates.io/crates/nom-both)
[![Docs.rs](https://docs.rs/nom-both/badge.svg)](https://docs.rs/nom-both)

## Requirement

nom must be 5.0.0 or later.
nom-both can be applied to function-style parser only.

## Usage

```Cargo.toml
[dependencies]
nom-both = "0.1.1"
```

## Example

nom-both provide `both_opt` and `both_alt` parser.
`both_opt` means that the parser tries both argument parser and skip.
`both_alt` means that the parser tries both 1st argument parser and 2nd argument parser.

```rust
use nom::branch::*;
use nom::bytes::complete::*;
use nom::IResult;
use nom_both::both_parser;

#[both_parser]
pub fn both_opt_parser(s: &str) -> IResult<&str, Option<&str>> {
    let (s, _) = tag("1")(s)?;
    let (s, x) = both_opt(tag("2"))(s)?;
    let (s, _) = tag("2")(s)?;
    let (s, _) = tag("3")(s)?;
    Ok((s, x))
}

#[both_parser]
pub fn both_alt_parser(s: &str) -> IResult<&str, &str> {
    let (s, _) = tag("1")(s)?;
    let (s, x) = both_alt(tag("22"), tag("2"))(s)?;
    let (s, _) = tag("2")(s)?;
    let (s, _) = tag("3")(s)?;
    Ok((s, x))
}

#[test]
fn test() {
    let ret = both_opt_parser("123");
    assert_eq!("None", format!("{:?}", ret.unwrap().1));

    let ret = both_alt_parser("1223");
    assert_eq!("\"2\"", format!("{:?}", ret.unwrap().1));
}
```

`both_opt_parser` is expanded as below:

```rust
pub fn both_opt_parser(s: &str) -> IResult<&str, Option<&str>> {
    alt((
        { |s| {
            let (s, _) = tag("1")(s)?;
            let (s, x) = nom_both::some(tag("2"))(s)?;
            let (s, _) = tag("2")(s)?;
            let (s, _) = tag("3")(s)?;
            Ok((s, x))
        } },
        { |s| {
            let (s, _) = tag("1")(s)?;
            let (s, x) = nom_both::none(tag("2"))(s)?;
            let (s, _) = tag("2")(s)?;
            let (s, _) = tag("3")(s)?;
            Ok((s, x))
        } },
    ))(s)
}
```

`both_alt_parser` is expanded as below:

```rust
pub fn both_opt_parser(s: &str) -> IResult<&str, Option<&str>> {
    alt((
        { |s| {
            let (s, _) = tag("1")(s)?;
            let (s, x) = nom_both::alt_left(tag("22"), tag("2"))(s)?;
            let (s, _) = tag("2")(s)?;
            let (s, _) = tag("3")(s)?;
            Ok((s, x))
        } },
        { |s| {
            let (s, _) = tag("1")(s)?;
            let (s, x) = nom_both::alt_right(tag("22"), tag("2"))(s)?;
            let (s, _) = tag("2")(s)?;
            let (s, _) = tag("3")(s)?;
            Ok((s, x))
        } },
    ))(s)
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
