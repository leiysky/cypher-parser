use super::expression::*;
use super::keywords::white_space;
use nom::{
    branch::alt, bytes::complete::tag_no_case, combinator::map, combinator::opt, multi::many0,
    sequence::tuple, IResult,
};

#[derive(PartialEq, Debug)]
pub struct SinglePartQuery {
    pub reading_clauses: Vec<ReadingClause>,
    pub updating_clauses: Vec<UpdatingClause>,
    pub ret: Option<Return>,
}

#[derive(PartialEq, Debug)]
pub struct MultiPartQuery {}

#[derive(PartialEq, Debug)]
pub enum ReadingClause {
    Match(Match),
    Unwind(Unwind),
}

#[derive(PartialEq, Debug)]
pub enum UpdatingClause {
    Create(Create),
    Merge(Merge),
    Delete(Delete),
    Set(Set),
    Remove(Remove),
}

#[derive(PartialEq, Debug)]
pub struct Return {
    pub distinct: bool,
    pub return_body: ReturnBody,
}

#[derive(PartialEq, Debug)]
pub struct ReturnBody {}

#[derive(PartialEq, Debug)]
pub struct Match {
    pub optional: bool,
    pub pattern: Pattern,
    pub where_clause: Option<Where>,
}

#[derive(PartialEq, Debug)]
pub struct Where {}

#[derive(PartialEq, Debug)]
pub struct Unwind {}

#[derive(PartialEq, Debug)]
pub struct Create {}

#[derive(PartialEq, Debug)]
pub struct Merge {}

#[derive(PartialEq, Debug)]
pub struct Delete {}

#[derive(PartialEq, Debug)]
pub struct Set {}

#[derive(PartialEq, Debug)]
pub struct Remove {}

pub fn parse_single_part_query(input: &str) -> IResult<&str, SinglePartQuery> {
    alt((
        map(
            tuple((
                many0(tuple((parse_reading_clause, white_space))),
                parse_return,
            )),
            |res| match res {
                (readings, ret) => SinglePartQuery {
                    reading_clauses: readings
                        .into_iter()
                        .map(|x| match x {
                            (r, _) => r,
                        })
                        .collect(),
                    updating_clauses: Vec::new(),
                    ret: Some(ret),
                },
            },
        ),
        map(
            tuple((
                many0(tuple((parse_reading_clause, white_space))),
                parse_updating_clause,
                many0(tuple((parse_updating_clause, white_space))),
                opt(parse_return),
            )),
            |res| match res {
                (readings, updating, updatings, ret) => {
                    let mut updating_clauses: Vec<UpdatingClause> = vec![updating];
                    updating_clauses.extend(updatings.into_iter().map(|x| x.0));
                    SinglePartQuery {
                        reading_clauses: readings
                            .into_iter()
                            .map(|x| match x {
                                (r, _) => r,
                            })
                            .collect(),
                        updating_clauses: updating_clauses,
                        ret: ret,
                    }
                }
            },
        ),
    ))(input)
}

pub fn parse_multi_part_query(input: &str) -> IResult<&str, MultiPartQuery> {
    Ok((input, MultiPartQuery {}))
}

pub fn parse_reading_clause(input: &str) -> IResult<&str, ReadingClause> {
    alt((
        map(parse_match, |c| ReadingClause::Match(c)),
        map(parse_unwind, |c| ReadingClause::Unwind(c)),
    ))(input)
}

pub fn parse_updating_clause(input: &str) -> IResult<&str, UpdatingClause> {
    alt((
        map(parse_create, |c| UpdatingClause::Create(c)),
        map(parse_merge, |c| UpdatingClause::Merge(c)),
        map(parse_set, |c| UpdatingClause::Set(c)),
        map(parse_delete, |c| UpdatingClause::Delete(c)),
        map(parse_remove, |c| UpdatingClause::Remove(c)),
    ))(input)
}

pub fn parse_return(input: &str) -> IResult<&str, Return> {
    match tuple((
        tag_no_case("RETURN"),
        opt(tuple((white_space, tag_no_case("DISTINCT")))),
        white_space,
        parse_return_body,
    ))(input)
    {
        Ok((o, (_, distinct, _, return_body))) => Ok((
            o,
            Return {
                distinct: match distinct {
                    Some(_) => true,
                    None => false,
                },
                return_body: return_body,
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn parse_return_body(input: &str) -> IResult<&str, ReturnBody> {
    Ok((input, ReturnBody {}))
}

pub fn parse_match(input: &str) -> IResult<&str, Match> {
    match tuple((
        opt(tuple((tag_no_case("OPTIONAL"), white_space))),
        tag_no_case("MATCH"),
        white_space,
        parse_pattern,
        opt(tuple((white_space, parse_where))),
    ))(input)
    {
        Ok((o, (optional, _, _, pattern, where_clause))) => Ok((
            o,
            Match {
                optional: match optional {
                    Some(_) => true,
                    None => false,
                },
                pattern: pattern,
                where_clause: match where_clause {
                    Some((_, w)) => Some(w),
                    None => None,
                },
            },
        )),
        Err(e) => Err(e),
    }
}

pub fn parse_unwind(input: &str) -> IResult<&str, Unwind> {
    Ok((input, Unwind {}))
}

pub fn parse_create(input: &str) -> IResult<&str, Create> {
    unimplemented!()
}

pub fn parse_merge(input: &str) -> IResult<&str, Merge> {
    unimplemented!()
}

pub fn parse_set(input: &str) -> IResult<&str, Set> {
    unimplemented!()
}

pub fn parse_delete(input: &str) -> IResult<&str, Delete> {
    unimplemented!()
}

pub fn parse_remove(input: &str) -> IResult<&str, Remove> {
    unimplemented!()
}

pub fn parse_where(input: &str) -> IResult<&str, Where> {
    Ok((input, Where {}))
}

#[cfg(test)]
mod clause_test {
    use super::*;
}
