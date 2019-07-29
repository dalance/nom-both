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
