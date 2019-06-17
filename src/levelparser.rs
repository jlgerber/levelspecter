use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming},
    bytes::complete::{tag},
    sequence::{tuple, preceded },
    multi::{ fold_many1},
};

use aschar_casesensitive::{ upperalphanum1, alpha_alphanum, alpha_alphanum_upper};


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

#[inline]
fn shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1(
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


#[inline]
fn seq_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( 
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
    fold_many1(
        parse_show, 
        Vec::new(), 
        |mut acc: Vec<_>, item| { 
            acc.push(item); 
            acc
        } 
    )(input)
}

fn levelparser(input: &str) -> IResult<&str, String> {
    let (leftover, result) = all_consuming(
        alt((
            shot_alt,
            seq_alt,
            show_alt,
        )))
     (input)?;

    Ok((leftover, result.join(".")))
}

pub fn levelspec_parser(input: &str) -> Result<String, String> {
    match levelparser(input) {
        Err(_) => Err(format!("Unable to parse levelspec for {}", input)),
        Ok((_,ls)) => Ok(ls),
    }
}