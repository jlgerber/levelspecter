use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming},
    bytes::complete::{tag},
    character::complete::digit1,
    sequence::{tuple, preceded },
    multi::{ fold_many1},
};

use crate::LevelSpecterError;
use aschar_casesensitive::{ /*upperalphanum1,*/ alpha_alphanum_upper};

#[inline]
fn parse_show(input: &str) -> IResult<&str, &str> {
    alt((
        alpha_alphanum_upper,
        tag("%")
    ))
    (input)
}

#[inline]
fn parse_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."), alpha_alphanum_upper),
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
fn parse_assetdev_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."), tag("ASSETDEV")),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

#[inline]
fn parse_assetdev_shot(input: &str) -> IResult<&str, &str> {
    alt((
    preceded(tag("."), alpha_alphanum_upper ),
    preceded(tag("."), tag("%"))
    ))
    (input)
}

// The shot alternative, has a show a sequence, and a shot
// accumulated into a vector. 
#[inline]
fn shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        alt((
            tuple(( parse_show, parse_seq, parse_shot)),
            tuple((parse_show, parse_assetdev_seq, parse_assetdev_shot))
        )),
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
    
        #[test]
        fn can_parse_assetdev() {
            let ls = levelspec_parser("DEV01.ASSETDEV");
            assert_eq!(ls, Ok(vec!["DEV01", "ASSETDEV"]))
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