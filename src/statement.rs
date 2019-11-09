use super::keywords::*;
use nom::{branch::alt, bytes::streaming::tag_no_case, combinator::opt, sequence::tuple, IResult};

pub struct Statement {
    pub query: Query,
}

pub struct Query {
    pub regular_query: Option<RegularQuery>,
    pub stand_alone_call: Option<StandAloneCall>,
}

pub struct RegularQuery {
    pub single_query: SingleQuery,
    pub union: Option<Union>,
}

pub struct StandAloneCall {}

pub struct SingleQuery {}

pub struct Union {
    pub all: bool,
    pub single_query: SingleQuery,
}

pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    match parse_query(input) {
        Ok((i, query)) => Ok((i, Statement { query })),
        Err(e) => Err(e),
    }
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    let out;
    let mut query = Query {
        regular_query: None,
        stand_alone_call: None,
    };
    match parse_regular_query(input) {
        Ok((o, regular)) => {
            out = o;
            query.regular_query = Some(regular);
        }
        Err(e) => match parse_stand_alone_call(input) {
            Ok((o, call)) => {
                out = o;
                query.stand_alone_call = Some(call);
            }
            Err(_) => return Err(nom::Err::convert(e)),
        },
    }
    Ok((out, query))
}

pub fn parse_regular_query(input: &str) -> IResult<&str, RegularQuery> {
    match parse_single_query(input) {
        Ok((o, single)) => match tuple((white_space, parse_union))(o) {
            Ok((_, (o, union))) => Ok((
                o,
                RegularQuery {
                    single_query: single,
                    union: Some(union),
                },
            )),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub fn parse_stand_alone_call(input: &str) -> IResult<&str, StandAloneCall> {
    unimplemented!()
}

pub fn parse_single_query(input: &str) -> IResult<&str, SingleQuery> {
    unimplemented!()
}

pub fn parse_union(input: &str) -> IResult<&str, Union> {
    match tuple((
        tag_no_case("UNION"),
        white_space,
        opt(tag_no_case("ALL")),
        white_space,
        parse_single_query,
    ))(input)
    {
        Ok((o, (union, _, all, _, single))) => Ok((
            o,
            Union {
                all: match all {
                    Some(_) => true,
                    None => false,
                },
                single_query: single,
            },
        )),
        Err(e) => Err(nom::Err::convert(e)),
    }
}

#[cfg(test)]
mod parse_statement_test {
    #[test]
    fn test_parse_statement() {
        assert_eq!(1, 1);
    }
}
