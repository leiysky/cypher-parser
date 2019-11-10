use super::keywords::*;
use nom::{bytes::complete::tag, combinator::opt, multi::many0, sequence::tuple, IResult};

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
    node_patterns: Vec<NodePattern>,
    relationship_patterns: Vec<RelationshipPattern>,
}

#[derive(PartialEq, Debug)]
pub struct Variable {}

#[derive(PartialEq, Debug)]
pub struct NodePattern {}

#[derive(PartialEq, Debug)]
pub struct RelationshipPattern {}

pub fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    match tuple((
        parse_pattern_part,
        many0(tuple((
            white_space,
            tag(","),
            white_space,
            parse_pattern_part,
        ))),
    ))(input)
    {
        Ok((o, (part, parts))) => {
            let mut pattern_parts = vec![part];
            pattern_parts.extend(parts.into_iter().map(|p| p.3));
            Ok((
                o,
                Pattern {
                    parts: pattern_parts,
                },
            ))
        }
        Err(e) => Err(e),
    }
}

pub fn parse_pattern_part(input: &str) -> IResult<&str, PatternPart> {
    match tuple((
        opt(tuple((parse_variable, white_space, tag("="), white_space))),
        parse_pattern_element,
    ))(input)
    {
        Ok((o, (var, element))) => Ok((
            o,
            PatternPart {
                variable: match var {
                    Some((v, _, _, _)) => Some(v),
                    None => None,
                },
                element: element,
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn parse_pattern_element(input: &str) -> IResult<&str, PatternElement> {
    match tuple((
        parse_node_pattern,
        many0(tuple((
            white_space,
            parse_relationship_pattern,
            white_space,
            parse_node_pattern,
        ))),
    ))(input)
    {
        Ok((o, (node, chains))) => Ok((o, {
            let mut nodes = vec![node];
            let mut relationships = vec![];
            for c in chains {
                relationships.push(c.1);
                nodes.push(c.3);
            }
            PatternElement {
                node_patterns: nodes,
                relationship_patterns: relationships,
            }
        })),
        Err(e) => Err(e),
    }
}

pub fn parse_variable(input: &str) -> IResult<&str, Variable> {
    unimplemented!()
}

pub fn parse_node_pattern(input: &str) -> IResult<&str, NodePattern> {
    unimplemented!()
}

pub fn parse_relationship_pattern(input: &str) -> IResult<&str, RelationshipPattern> {
    unimplemented!()
}
