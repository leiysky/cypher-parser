use nom::{
    bytes::complete::*,
    combinator::{map, map_res, peek},
    error,
    sequence::tuple,
    Err, IResult,
};
use unicode_xid::UnicodeXID;

pub fn white_space(input: &str) -> IResult<&str, &str> {
    take_while1(|c| vec![' ', '\t', '\n'].contains(&c))(input)
}

pub fn opt_white_space(input: &str) -> IResult<&str, &str> {
    take_while(|c| vec![' ', '\t', '\n'].contains(&c))(input)
}

pub fn unescaped_str(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            peek(map_res(take(1usize), |c: &str| match c.chars().nth(0) {
                Some(v) => {
                    if UnicodeXID::is_xid_start(v) {
                        Ok(c)
                    } else {
                        Err(Err::Error((input, error::ErrorKind::Char)))
                    }
                }
                None => Err(Err::Error((input, error::ErrorKind::Char))),
            })),
            take_while(|c| UnicodeXID::is_xid_continue(c)),
        )),
        |v| v.1,
    )(input)
}

#[cfg(test)]
mod keywords_test {
    use super::*;

    #[test]
    fn test_white_space() {
        assert_eq!(white_space("  "), Ok(("", "  ")))
    }

    #[test]
    fn test_unescaped_str() {
        assert_eq!(unescaped_str("helloworld"), Ok(("", "helloworld")))
    }
}
