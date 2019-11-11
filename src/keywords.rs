use nom::{bytes::complete::*, combinator::opt, IResult};

pub fn white_space(input: &str) -> IResult<&str, &str> {
    take_while1(|c| vec![' ', '\t', '\n'].contains(&c))(input)
}

pub fn opt_white_space(input: &str) -> IResult<&str, &str> {
    take_while(|c| vec![' ', '\t', '\n'].contains(&c))(input)
}

#[cfg(test)]
mod keywords_test {
    use super::*;

    #[test]
    fn test_white_space() {
        assert_eq!(white_space("  "), Ok(("", "  ")))
    }
}
