use super::{expression::*, keywords::*};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    combinator::{map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(PartialEq, Debug)]
pub enum SchemaName {
    SymbolicName(SymbolicName),
    ReservedWord(ReservedWord),
}

pub type SymbolicName = String;

pub type ReservedWord = String;

#[derive(PartialEq, Debug)]
pub struct MapLiteral {
    pub props: Vec<(SchemaName, Expr)>,
}

#[derive(PartialEq, Debug)]
pub enum Parameter {
    SymbolicName(SymbolicName),
    DecimalInteger(DecimalInteger),
}

pub type DecimalInteger = i64;

pub type LiteralInteger = i32;

pub fn parse_schema_name(input: &str) -> IResult<&str, SchemaName> {
    alt((
        map(parse_symbolic_name, |v| SchemaName::SymbolicName(v)),
        map(parse_reserved_word, |v| SchemaName::ReservedWord(v)),
    ))(input)
}

pub fn parse_symbolic_name(input: &str) -> IResult<&str, SymbolicName> {
    map(unescaped_str, |s| s.to_owned())(input)
}

pub fn parse_reserved_word(input: &str) -> IResult<&str, ReservedWord> {
    Ok(("", input.to_owned()))
}

pub fn parse_map_literal(input: &str) -> IResult<&str, MapLiteral> {
    map(
        tuple((
            char('{'),
            opt_white_space,
            opt(map(
                tuple((
                    parse_schema_name,
                    opt_white_space,
                    char(':'),
                    opt_white_space,
                    parse_expr,
                    opt_white_space,
                    many0(map(
                        tuple((
                            char(','),
                            opt_white_space,
                            parse_schema_name,
                            opt_white_space,
                            char(':'),
                            opt_white_space,
                            parse_expr,
                            opt_white_space,
                        )),
                        |v| (v.2, v.6),
                    )),
                )),
                |v| {
                    let mut pairs = vec![(v.0, v.4)];
                    pairs.extend(v.6);
                    pairs
                },
            )),
            char('}'),
        )),
        |v| MapLiteral {
            props: match v.2 {
                Some(p) => p,
                _ => vec![],
            },
        },
    )(input)
}

pub fn parse_parameter(input: &str) -> IResult<&str, Parameter> {
    map(
        tuple((
            tag("$"),
            alt((
                map(parse_symbolic_name, |v| Parameter::SymbolicName(v)),
                map(parse_decimal_integer, |v| Parameter::DecimalInteger(v)),
            )),
        )),
        |v| v.1,
    )(input)
}

pub fn parse_literal_integer(input: &str) -> IResult<&str, LiteralInteger> {
    println!("{}", input);
    unimplemented!()
}

pub fn parse_decimal_integer(input: &str) -> IResult<&str, DecimalInteger> {
    unimplemented!()
}
