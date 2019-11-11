use nom::IResult;

#[derive(PartialEq, Debug)]
pub enum SchemaName {
    SymbolicName(SymbolicName),
    ReservedWord(ReservedWord),
}

pub type SymbolicName = String;

pub type ReservedWord = String;

#[derive(PartialEq, Debug)]
pub struct MapLiteral {}

#[derive(PartialEq, Debug)]
pub struct Parameter {}

pub type LiteralInteger = i32;

pub fn parse_schema_name(input: &str) -> IResult<&str, SchemaName> {
    unimplemented!()
}

pub fn parse_symbolic_name(input: &str) -> IResult<&str, SymbolicName> {
    unimplemented!()
}

pub fn parse_map_literal(input: &str) -> IResult<&str, MapLiteral> {
    unimplemented!()
}

pub fn parse_parameter(input: &str) -> IResult<&str, Parameter> {
    unimplemented!()
}

pub fn parse_literal_integer(input: &str) -> IResult<&str, LiteralInteger> {
    unimplemented!()
}