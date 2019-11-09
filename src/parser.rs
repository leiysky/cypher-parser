use super::keywords::*;
use super::statement::*;
use nom::{sequence::tuple, Err, IResult};

pub fn parse(input: &str) -> IResult<&str, Statement> {
    let parser = tuple((white_space, parse_statement));
    match parser(input) {
        Ok((_, (i, stmt))) => Ok((i, stmt)),
        Err(e) => Err(e),
    }
}
