use nom::{bytes::complete::*, sequence::tuple, IResult};

pub fn white_space(input: &str) -> IResult<&str, &str> {
    Ok(take_while(|c| vec![' ', '\t', '\n'].contains(&c))(input)?)
}

#[cfg(test)]
mod keywords_test {
    use super::*;

    #[test]
    fn test_white_space() {
        assert_eq!(white_space(" "), Ok(("", " ")))
    }
}
