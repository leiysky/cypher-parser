use super::keywords::*;
use super::literal::*;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::map, combinator::opt,
    multi::many0, sequence::tuple, IResult,
};

#[derive(PartialEq, Debug)]
pub struct Pattern {
    pub parts: Vec<PatternPart>,
}

#[derive(PartialEq, Debug)]
pub struct PatternPart {
    pub variable: Option<Variable>,
    pub element: PatternElement,
}

#[derive(PartialEq, Debug)]
pub struct PatternElement {
    pub node_patterns: Vec<NodePattern>,
    pub relationship_patterns: Vec<RelationshipPattern>,
}

type Variable = SymbolicName;

#[derive(PartialEq, Debug)]
pub struct NodePattern {
    pub variable: Option<Variable>,
    pub labels: Vec<NodeLabel>,
    pub properties: Option<Properties>,
}

#[derive(PartialEq, Debug)]
pub enum Properties {
    MapLiteral(MapLiteral),
    Parameter(Parameter),
}

type NodeLabel = SchemaName;

#[derive(PartialEq, Debug)]
pub enum Direction {
    Left,
    Right,
    Both,
}

#[derive(PartialEq, Debug)]
pub struct RelationshipPattern {
    pub detail: Option<RelationshipDetail>,
    pub direction: Direction,
}

#[derive(PartialEq, Debug)]
pub struct RelationshipDetail {
    pub variable: Option<Variable>,
    pub rel_types: Vec<RelationshipType>,
    pub range: (LiteralInteger, LiteralInteger),
    pub properties: Option<Properties>,
}

type RelationshipType = SchemaName;

pub fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(
        tuple((
            parse_pattern_part,
            many0(map(
                tuple((white_space, tag(","), white_space, parse_pattern_part)),
                |v| v.3,
            )),
        )),
        |v| {
            let mut pattern_parts = vec![v.0];
            pattern_parts.extend(v.1);
            Pattern {
                parts: pattern_parts,
            }
        },
    )(input)
}

pub fn parse_pattern_part(input: &str) -> IResult<&str, PatternPart> {
    map(
        tuple((
            opt(map(
                tuple((parse_variable, white_space, tag("="), white_space)),
                |v| v.0,
            )),
            parse_pattern_element,
        )),
        |v| PatternPart {
            variable: v.0,
            element: v.1,
        },
    )(input)
}

pub fn parse_pattern_element(input: &str) -> IResult<&str, PatternElement> {
    map(
        tuple((
            parse_node_pattern,
            many0(map(
                tuple((
                    white_space,
                    parse_relationship_pattern,
                    white_space,
                    parse_node_pattern,
                )),
                |v| (v.1, v.3),
            )),
        )),
        |v| {
            let mut nodes = vec![v.0];
            let mut relationships = vec![];
            for c in v.1 {
                relationships.push(c.0);
                nodes.push(c.1);
            }
            PatternElement {
                node_patterns: nodes,
                relationship_patterns: relationships,
            }
        },
    )(input)
}

pub fn parse_variable(input: &str) -> IResult<&str, Variable> {
    parse_symbolic_name(input)
}

pub fn parse_node_pattern(input: &str) -> IResult<&str, NodePattern> {
    map(
        tuple((
            char('('),
            opt(map(tuple((parse_variable, opt(white_space))), |v| v.0)),
            many0(map(tuple((parse_node_label, opt(white_space))), |v| v.0)),
            opt(map(tuple((parse_properties, opt(white_space))), |v| v.0)),
            char(')'),
        )),
        |v| NodePattern {
            variable: v.1,
            labels: v.2,
            properties: v.3,
        },
    )(input)
}

pub fn parse_node_label(input: &str) -> IResult<&str, NodeLabel> {
    map(tuple((char(':'), white_space, parse_schema_name)), |sch| {
        sch.2
    })(input)
}

pub fn parse_relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    alt((
        map(
            tuple((tag("<-"), opt(parse_relationship_detail), tag("->"))),
            |v| RelationshipPattern {
                detail: v.1,
                direction: Direction::Both,
            },
        ),
        map(
            tuple((tag("<-"), opt(parse_relationship_detail), tag("-"))),
            |v| RelationshipPattern {
                detail: v.1,
                direction: Direction::Left,
            },
        ),
        map(
            tuple((tag("-"), opt(parse_relationship_detail), tag("->"))),
            |v| RelationshipPattern {
                detail: v.1,
                direction: Direction::Right,
            },
        ),
        map(
            tuple((tag("-"), opt(parse_relationship_detail), tag("-"))),
            |v| RelationshipPattern {
                detail: v.1,
                direction: Direction::Both,
            },
        ),
    ))(input)
}

pub fn parse_relationship_detail(input: &str) -> IResult<&str, RelationshipDetail> {
    map(
        tuple((
            opt(white_space),
            opt(map(tuple((parse_variable, white_space)), |v| v.0)),
            map(
                opt(tuple((
                    char(':'),
                    opt(white_space),
                    parse_schema_name,
                    many0(map(
                        tuple((
                            opt(white_space),
                            char('|'),
                            opt(char(':')),
                            opt(white_space),
                            parse_schema_name,
                        )),
                        |v| v.4,
                    )),
                ))),
                |v| match v {
                    Some((_, _, n, nn)) => {
                        let types = vec![n];
                        types.extend(nn);
                        types
                    }
                    None => vec![],
                },
            ),
            map(
                opt(tuple((
                    char('*'),
                    opt(white_space),
                    opt(tuple((parse_literal_integer, opt(white_space)))),
                    opt(tuple((
                        tag(".."),
                        opt(white_space),
                        opt(tuple((parse_literal_integer, opt(white_space)))),
                    ))),
                ))),
                |v| {
                    (
                        match v {
                            Some((_, _, Some((v1, _)), _)) => v1,
                            _ => -1,
                        },
                        match v {
                            Some((_, _, _, Some((_, _, Some((v2, _)))))) => v2,
                            _ => -1,
                        },
                    )
                },
            ),
            opt(parse_properties),
        )),
        |v| RelationshipDetail {
            variable: v.1,
            rel_types: v.2,
            range: v.3,
            properties: v.4,
        },
    )(input)
}

pub fn parse_properties(input: &str) -> IResult<&str, Properties> {
    alt((
        map(parse_map_literal, |v| Properties::MapLiteral(v)),
        map(parse_parameter, |v| Properties::Parameter(v)),
    ))(input)
}
