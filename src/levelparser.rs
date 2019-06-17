use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming},
    bytes::complete::{tag},
    sequence::{tuple, preceded },
    multi::{ fold_many1},
};
use crate::LevelSpecterError;
use aschar_casesensitive::{ upperalphanum1, alpha_alphanum_upper};

#[inline]
fn parse_show(input: &str) -> IResult<&str, &str> {
    alpha_alphanum_upper(input)
}

#[inline]
fn parse_seq(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), alpha_alphanum_upper)(input)
}

#[inline]
fn parse_shot(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), upperalphanum1 )(input)
}

// The shot alternative, has a show a sequence, and a shot
// accumulated into a vector. 
#[inline]
fn shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        tuple(( parse_show, parse_seq, parse_shot)),
        Vec::new(), 
        |mut acc: Vec<_>, item| {
            let (show,seq,shot) = item;
            acc.push(show); 
            acc.push(seq); 
            acc.push(shot);
            acc
        }
    )(input)
}

// the sequence alternative has a show and a sequence
// separated by a period, accumulated into a vector
#[inline]
fn seq_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        tuple((parse_show, parse_seq)),
        Vec::new(), 
        |mut acc: Vec<_>, item| {
            let (show,seq) = item ;
            acc.push(show); 
            acc.push(seq);
            acc
        } 
    )(input)
}


#[inline]
fn show_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to place into a vector
        parse_show, 
        Vec::new(), 
        |mut acc: Vec<_>, item| { 
            acc.push(item); 
            acc
        } 
    )(input)
}

fn levelparser(input: &str) -> IResult<&str, Vec<&str>> {
    let (leftover, result) = all_consuming(
        alt((
            shot_alt,
            seq_alt,
            show_alt,
        )))
     (input)?;

    Ok((leftover, result))
}

/// Parse a levelspec from a string
pub fn levelspec_parser(input: &str) -> Result<Vec<&str>, LevelSpecterError> {
    match levelparser(input) {
        Err(_) => Err( LevelSpecterError::ParseError(format!("Unable to parse levelspec for {}", input))),
        //Ok((_,ls)) => Ok(ls.iter().map(|x| x.to_string()).collect::<Vec<_>>() ),
        Ok((_,ls)) => Ok(ls),

    }
}