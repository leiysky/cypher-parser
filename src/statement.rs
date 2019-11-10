use super::{clause::*, expression::*, keywords::*};
use nom::{
    branch::alt, bytes::streaming::tag_no_case, combinator::map, combinator::opt, sequence::tuple,
    IResult,
};

#[derive(PartialEq, Debug)]
pub struct Statement {
    pub query: Query,
}

#[derive(PartialEq, Debug)]
pub enum Query {
    RegularQuery(RegularQuery),
    StandAloneCall(StandAloneCall),
}

#[derive(PartialEq, Debug)]
pub struct RegularQuery {
    pub single_query: SingleQuery,
    pub union: Option<Union>,
}

#[derive(PartialEq, Debug)]
pub struct StandAloneCall {}

#[derive(PartialEq, Debug)]
pub enum SingleQuery {
    SinglePartQuery(SinglePartQuery),
    MultiPartQUery(MultiPartQuery),
}

#[derive(PartialEq, Debug)]
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
    alt((
        map(parse_regular_query, |regular| Query::RegularQuery(regular)),
        map(parse_stand_alone_call, |call| Query::StandAloneCall(call)),
    ))(input)
}

pub fn parse_regular_query(input: &str) -> IResult<&str, RegularQuery> {
    match tuple((parse_single_query, opt(tuple((white_space, parse_union)))))(input) {
        Ok((o, (single, union))) => Ok((
            o,
            RegularQuery {
                single_query: single,
                union: match union {
                    Some((_, u)) => Some(u),
                    None => None,
                },
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn parse_stand_alone_call(input: &str) -> IResult<&str, StandAloneCall> {
    unimplemented!()
}

pub fn parse_single_query(input: &str) -> IResult<&str, SingleQuery> {
    alt((
        map(parse_single_part_query, |part| {
            SingleQuery::SinglePartQuery(part)
        }),
        map(parse_multi_part_query, |part| {
            SingleQuery::MultiPartQUery(part)
        }),
    ))(input)
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
        Ok((o, (_, _, all, _, single))) => Ok((
            o,
            Union {
                all: match all {
                    Some(_) => true,
                    None => false,
                },
                single_query: single,
            },
        )),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod parse_statement_test {
    use super::*;

    // #[test]
    // fn test_parse_statement() {
    //     assert_eq!(
    //         parse_statement("match a return a"),
    //         Ok((
    //             "",
    //             Statement {
    //                 query: Query::RegularQuery(RegularQuery {
    //                     single_query: SingleQuery::SinglePartQuery(SinglePartQuery {
    //                         reading_clauses: vec![ReadingClause::Match(Match {
    //                             optional: false,
    //                             pattern: Pattern {},
    //                             where_clause: None,
    //                         })],
    //                         updating_clauses: vec![],
    //                         ret: Some(Return {
    //                             distinct: false,
    //                             return_body: ReturnBody {}
    //                         }),
    //                     }),
    //                     union: None,
    //                 })
    //             }
    //         ))
    //     );
    // }
}
