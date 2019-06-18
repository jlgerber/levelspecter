#![allow(unused_imports)]
use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming},
    bytes::complete::{tag, tag_no_case},
    character::complete::digit1,
    sequence::{tuple, preceded, terminated },
    multi::{ fold_many1},
};

use crate::LevelSpecterError;
use aschar_casesensitive::{ upperalphanum1, alpha_alphanum_upper, alpha_alphanum, alpha_alphanum_upper_alpha, alpha_alphanum_alpha};


/// Parse a levelspec from a string
/// 
/// # Parameters
/// 
/// * `input` - str we wish to convert to a levelspec
/// 
/// # Returns
/// 
/// A `Vec` of `&str` capturing the show, sequence, shot, if successful. Otherwise,
/// a LevelSpecterError
/// 
/// # Example
/// 
/// ```
/// use levelspecter::{levelspec_parser, LevelSpecterError};
/// 
/// let results = levelspec_parser("DEV01.RD.0001");
/// assert_eq!(results, Ok(vec!["DEV01", "RD", "0001"]));
/// ```
pub fn levelspec_parser(input: &str) -> Result<Vec<&str>, LevelSpecterError> {
    match levelparser(input) {
        Err(_) => Err( LevelSpecterError::ParseError(format!("Unable to parse levelspec for {}", input))),
        Ok((_,ls)) => Ok(ls),

    }
}

#[inline]
fn parse_show(input: &str) -> IResult<&str, &str> {
    alt((
        if cfg!(feature = "case-insensitive") {alpha_alphanum} else {alpha_alphanum_upper},
        tag("%")
    ))
    (input)
}

#[inline]
fn parse_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."), if cfg!(feature = "case-insensitive") {alpha_alphanum_alpha} else {alpha_alphanum_upper_alpha}),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

#[inline]
fn parse_shot(input: &str) -> IResult<&str, &str> {
    alt((
    preceded(tag("."), digit1 ),
    preceded(tag("."), tag("%"))
    ))
    (input)
}

// NOTE: if I decide to go case insensitive, there is tag_no_case()
#[inline]
#[cfg(feature = "case-insensitive")]
fn parse_assetdev_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."),tag_no_case("ASSETDEV")),
        preceded(tag("."), tag("%"))
    ))
    (input)
}
// NOTE: if I decide to go case insensitive, there is tag_no_case()
#[inline]
#[cfg(not(feature = "case-insensitive"))]
fn parse_assetdev_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."),tag("ASSETDEV")),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

#[inline]
fn parse_assetdev_shot(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."), if cfg!(feature = "case-insensitive") {alpha_alphanum_alpha} else {alpha_alphanum_upper_alpha} ),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

// TODO: support relative levelspec (eg  .AA or ..0001 or .AA. )
// rel_seq tuple((parse_seq))
fn parse_rel_seq(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), if cfg!(feature = "case-insensitive") {alpha_alphanum_alpha} else {alpha_alphanum_upper_alpha})(input)
}


// rel_seqshot tuple((terminated(parse_seq,tag("."))))
fn parse_rel_seqshot(input: &str) -> IResult<&str, &str> {
    terminated(parse_rel_seq, tag("."))(input)
}

// The shot alternative, has a show a sequence, and a shot
// accumulated into a vector. 
#[inline]
fn shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        alt((
            tuple((parse_show, parse_seq, parse_shot)),
            tuple((parse_show, parse_assetdev_seq, parse_assetdev_shot))
        )),
        Vec::with_capacity(3), 
        |mut acc: Vec<_>, item| {
            let (show, seq, shot) = item;
            acc.push(show); 
            acc.push(seq); 
            acc.push(shot);
            acc
        }
    )
    (input)
}

// the sequence alternative has a show and a sequence
// separated by a period, accumulated into a vector
#[inline]
fn seq_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        tuple((parse_show, parse_seq)),
        Vec::with_capacity(2), 
        |mut acc: Vec<_>, item| {
            let (show, seq) = item ;
            acc.push(show); 
            acc.push(seq);
            acc
        } 
    )
    (input)
}

// the sequence alternative has a show and a sequence
// separated by a period, accumulated into a vector
#[inline]
// .seq
fn rel_seq_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        parse_rel_seq,
        Vec::with_capacity(2), 
        |mut acc: Vec<_>, item| {
            acc.push(""); 
            acc.push(item);
            acc
        } 
    )
    (input)
}

#[inline]
// .seq.
fn rel_seq_rel_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        //terminated(parse_rel_seq, tag(".")),
        parse_rel_seqshot,
        Vec::with_capacity(3), 
        |mut acc: Vec<_>, item| {
            acc.push(""); 
            acc.push(item);
            acc.push(""); 
            acc
        } 
    )
    (input)
}

#[inline]
// .seq.
fn rel_seq_shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        tuple((parse_rel_seq, parse_shot)),
        Vec::with_capacity(3), 
        |mut acc: Vec<_>, item| {
            let (seq, shot) = item;
            acc.push(""); 
            acc.push(seq);
            acc.push(shot); 
            acc
        } 
    )
    (input)
}

#[inline]
fn show_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to place into a vector
        parse_show, 
        Vec::with_capacity(1), 
        |mut acc: Vec<_>, item| { 
            acc.push(item); 
            acc
        } 
    )
    (input)
}

fn levelparser(input: &str) -> IResult<&str, Vec<&str>> {
    let (leftover, result) = all_consuming(
        alt((
            rel_seq_shot_alt,
            rel_seq_rel_alt,
            rel_seq_alt,
            shot_alt,
            seq_alt,
            show_alt,
        )))
     (input)?;

    Ok((leftover, result))
}

#[cfg(test)]
mod levelspec {
    use super::*;

    mod show {
        use super::*;
            
        #[test]
        fn can_parse() {
            let ls = levelspec_parser("DEV01");
            assert_eq!(ls, Ok(vec!["DEV01"]))
        }
        
        #[cfg(feature = "case-insensitive")]
        #[test]
        fn can_parse_lowercase() {
            let ls = levelspec_parser("dev01");
            assert_eq!(ls, Ok(vec!["dev01"]))
        }

        #[cfg(not(feature = "case-insensitive"))]
        #[test]
        fn cannot_parse_lowercase() {
            let ls = levelspec_parser("dev01");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for dev01".to_string())));
        }

        #[test]
        fn cannot_start_with_number() {
            let ls = levelspec_parser("1DEV01");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for 1DEV01".to_string())));
        }
        
        #[test]
        fn cannot_have_space() {
            let ls = levelspec_parser("DEV 01");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV 01".to_string())));
        }
        
        #[test]
        fn cannot_have_wildcard_and_chars() {
            let ls = levelspec_parser("DEV%01");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV%01".to_string())));
        }

        #[test]
        fn cannot_have_underscore() {
            let ls = levelspec_parser("DEV_01");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV_01".to_string())));
        }

        #[test]
        fn can_parse_show_wildcard() {
            let ls = levelspec_parser("%");
            assert_eq!(ls, Ok(vec!["%"]))
        }
    }

    mod seq {
        use super::*;
            
        #[test]
        fn can_parse() {
            let ls = levelspec_parser("DEV01.RD");
            assert_eq!(ls, Ok(vec!["DEV01", "RD"]))
        }

        #[cfg(feature = "case-insensitive")]
        #[test]
        fn can_parse_lowercase() {
            let ls = levelspec_parser("dev01.rd");
            assert_eq!(ls, Ok(vec!["dev01", "rd"]))
        }
    
        #[test]
        fn can_parse_assetdev() {
            let ls = levelspec_parser("DEV01.ASSETDEV");
            assert_eq!(ls, Ok(vec!["DEV01", "ASSETDEV"]))
        }

        #[cfg(feature = "case-insensitive")]
        #[test]
        fn can_parse_assetdev_lowercase() {
            let ls = levelspec_parser("dev01.assetdev");
            assert_eq!(ls, Ok(vec!["dev01", "assetdev"]))
        }

        #[cfg(not(feature = "case-insensitive"))]
        #[test]
        fn can_parse_assetdev_lowercase() {
            let ls = levelspec_parser("dev01.assetdev");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for dev01.assetdev".to_string())))
        }
        #[test]
        fn cannot_start_with_number() {
            let ls = levelspec_parser("DEV01.1D");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.1D".to_string())));
        }
        
        #[test]
        fn cannot_have_space() {
            let ls = levelspec_parser("DEV01.R D");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.R D".to_string())));
        }
        
        #[test]
        fn cannot_have_wildcard_and_chars() {
            let ls = levelspec_parser("DEV01.R%");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.R%".to_string())));
        }

        #[test]
        fn cannot_have_underscore() {
            let ls = levelspec_parser("DEV01.R_D");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.R_D".to_string())));
        }

        #[test]
        fn can_parse_wildcard() {
            let ls = levelspec_parser("DEV01.%");
            assert_eq!(ls, Ok(vec!["DEV01","%"]))
        }
    }
 mod rel_seq {
    use super::*;
    
    #[test]
    fn can_parse_rel_seq() {
        let ls = rel_seq_alt(".RD");
        assert_eq!(ls, Ok(("",vec!["", "RD"])))
    }  

    #[test]
    fn can_parse_rel_seq_shot() {
        let ls = levelspec_parser(".RD.0001");
        assert_eq!(ls, Ok(vec!["", "RD", "0001"]))
    }

    #[test]
    fn can_parse_rel_seq_alt() {
        let ls = levelspec_parser(".RD.");
        assert_eq!(ls, Ok(vec!["", "RD", ""]))
    }
 }
    mod shot {
        use super::*;
    
        #[test]
        fn can_parse() {
            let ls = levelspec_parser("DEV01.RS.0001");
            assert_eq!(ls, Ok(vec!["DEV01", "RS", "0001"]))
        }

        #[test]
        fn can_parse_assetdev() {
            let ls = levelspec_parser("DEV01.ASSETDEV.FOOBAR");
            assert_eq!(ls, Ok(vec!["DEV01", "ASSETDEV", "FOOBAR"]))
        }

        #[cfg(feature = "case-insensitive")]
        #[test]
        fn can_parse_assetdev_lowercase() {
            let ls = levelspec_parser("dev01.assetdev.foobar");
            assert_eq!(ls, Ok(vec!["dev01", "assetdev", "foobar"]))
        }

        #[cfg(not(feature = "case-insensitive"))]
        #[test]
        fn cannot_parse_assetdev_lowercase() {
            let ls = levelspec_parser("dev01.assetdev.foobar");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for dev01.assetdev.foobar".to_string())))
        }

        #[test]
        fn cannot_start_with_letter() {
            let ls = levelspec_parser("DEV01.RD.R0001");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.RD.R0001".to_string())));
        }
        
        #[test]
        fn cannot_have_space() {
            let ls = levelspec_parser("DEV01.RD.0 001");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.RD.0 001".to_string())));
        }
        
        #[test]
        fn cannot_have_wildcard_and_chars() {
            let ls = levelspec_parser("DEV01.RD.00%");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.RD.00%".to_string())));
        }

        #[test]
        fn cannot_have_underscore() {
            let ls = levelspec_parser("DEV01.RD.0_001");
            assert_eq!(ls, Err(LevelSpecterError::ParseError("Unable to parse levelspec for DEV01.RD.0_001".to_string())));
        }

        #[test]
        fn can_parse_wildcard() {
            let ls = levelspec_parser("DEV01.RS.%");
            assert_eq!(ls, Ok(vec!["DEV01", "RS", "%"]))
        }
    }
}

#[cfg(test)]
mod atoms {
    use super::*;

    #[test]
    fn show_can_parse_wildcard() {
        let result = parse_show("%");
        assert!(result.is_ok());
        assert_eq!(result, Ok(("", "%")));
    }

    #[test]
    fn seq_can_parse_wildcard() {
        let result = parse_seq(".%");
        assert!(result.is_ok());
        assert_eq!(result, Ok(("", "%")));
    }

    #[test]
    fn shot_can_parse_wildcard() {
        let result = parse_shot(".%");
        assert!(result.is_ok());
        assert_eq!(result, Ok(("", "%")));
    }
    
}