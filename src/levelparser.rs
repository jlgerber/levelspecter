#![allow(unused_imports)]
use nom::{
    IResult,
    Err as NomErr,
    error::ErrorKind,
    branch::alt,
    combinator::{all_consuming, map},
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
/// // parse shot
/// let results = levelspec_parser("DEV01.RD.0001");
/// assert_eq!(results, Ok(vec!["DEV01", "RD", "0001"]));
/// 
/// // parse relative shot
/// let results = levelspec_parser(".RD.0001");
/// assert_eq!(results, Ok(vec!["", "RD", "0001"]));
/// ```
pub fn levelspec_parser(input: &str) -> Result<Vec<&str>, LevelSpecterError> {
    match levelparser(input) {
        Err(_) => Err( LevelSpecterError::ParseError(format!("Unable to parse levelspec for {}", input))),
        Ok((_,ls)) => Ok(ls),

    }
}

//-------------------//
//    parse_show     //
//-------------------//
#[inline]
fn parse_show(input: &str) -> IResult<&str, &str> {
    alt((
        if cfg!(feature = "case-insensitive") {alpha_alphanum} else {alpha_alphanum_upper},
        tag("%")
    ))
    (input)
}

#[cfg(test)]
mod parse_show {
    use super::*;

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_show() {
        let ls = parse_show("dev01");
        assert_eq!(ls, Ok(("","dev01")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_show() {
        let ls = rel_shot_alt("dev01");
        assert_eq!(ls, Err(NomErr::Error(("dev01", ErrorKind::Tag))))
    }  
}


//--------------------//
//     parse_seq      //
//--------------------//
#[inline]
fn parse_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."), if cfg!(feature = "case-insensitive") {alpha_alphanum_alpha} else {alpha_alphanum_upper_alpha}),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

#[cfg(test)]
mod parse_seq {
    use super::*;

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_seq() {
        let ls = parse_seq(".rd");
        assert_eq!(ls, Ok(("", "rd")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_seq() {
        let ls = rel_shot_alt(".rd");
        assert_eq!(ls, Err(NomErr::Error(("rd", ErrorKind::Tag))))
    }  
}


//---------------------//
//      parse_shot     //
//---------------------//
#[inline]
fn parse_shot(input: &str) -> IResult<&str, &str> {
    alt((
    preceded(tag("."), digit1 ),
    preceded(tag("."), tag("%"))
    ))
    (input)
}

#[cfg(test)]
mod parse_shot {
    use super::*;

    #[test]
    fn can_parse_shot() {
        let ls = parse_shot(".0001");
        assert_eq!(ls, Ok(("", "0001")))
    }
}


//-----------------------//
//   parse_assetdev_seq  //
//-----------------------//
#[inline]
#[cfg(feature = "case-insensitive")]
fn parse_assetdev_seq(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."),tag_no_case("ASSETDEV")),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

#[cfg(test)]
mod parse_assetdev_seq_case_insensitive {
    use super::*;

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_assetdev() {
        let ls = parse_assetdev_seq(".assetdev");
        assert_eq!(ls, Ok(("","assetdev")))
    }  

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_assetdev_capital() {
        let ls = parse_assetdev_seq(".ASSETDEV");
        assert_eq!(ls, Ok(("","ASSETDEV")))
    }  

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_wildcard() {
        let ls = parse_assetdev_seq(".%");
        assert_eq!(ls, Ok(("","%")))
    }  
    
}


//---------------------------//
//    parse_assetdev_seq     //
//---------------------------//
// parse the assetdev sequence
#[inline]
#[cfg(not(feature = "case-insensitive"))]
fn parse_assetdev_seq(input: &str) -> IResult<&str, &str> {
    // TODO: this may be a problem as we are not backtracking
    // if .% is matched here, will that limit shots that are
    // matched afterwards to the assetdev_shot?. probably. this
    // is order dependent i would surmise. i should probably 
    // remove the % as w dont want to match against asssetdev shots 
    // if the sequence is unknown
    //alt((
        preceded(tag("."),tag("ASSETDEV"))//,
        //preceded(tag("."), tag("%"))
    //))
    (input)
}

#[cfg(test)]
mod parse_assetdev_seq_case_sensitive {
    use super::*;

    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_assetdev_lower() {
        let ls = parse_assetdev_seq(".assetdev");
        assert_eq!(ls, Err(NomErr::Error(("assetdev", ErrorKind::Tag))))
    }  

    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_assetdev_capital() {
        let ls = parse_assetdev_seq(".ASSETDEV");
        assert_eq!(ls, Ok(("","ASSETDEV")))
    }  

    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_seq_other_than_assetdev() {
        let ls = parse_assetdev_seq(".RD");
        assert_eq!(ls, Err(NomErr::Error(("RD", ErrorKind::Tag))))
    }  

/*
    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_wildcard() {
        let ls = parse_assetdev_seq(".%");
        assert_eq!(ls, Ok(("","%")))
    }  
    */
}


//---------------------//
// parse_assetdev_shot //
//---------------------//
#[inline]
fn parse_assetdev_shot(input: &str) -> IResult<&str, &str> {
    alt((
        preceded(tag("."), if cfg!(feature = "case-insensitive") {alpha_alphanum_alpha} else {alpha_alphanum_upper_alpha} ),
        preceded(tag("."), tag("%"))
    ))
    (input)
}

#[cfg(test)]
mod parse_assetdev_shot {
    use super::*;

    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_assetdev_shot_lower() {
        let ls = parse_assetdev_shot(".foobar");
        assert_eq!(ls, Err(NomErr::Error(("foobar", ErrorKind::Tag))))
    }  

    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_assetdev_shot_capital() {
        let ls = parse_assetdev_shot(".FOOBAR");
        assert_eq!(ls, Ok(("","FOOBAR")))
    }  

    #[test]
   #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_wildcard() {
        let ls = parse_assetdev_shot(".%");
        assert_eq!(ls, Ok(("","%")))
    }  
}


//------------------------//
//     parse_rel_seq      //
//------------------------//
// parse relative sequence. 
// EG .RD or .%
fn parse_rel_seq(input: &str) -> IResult<&str, &str> {
     alt((
        preceded(tag("."), if cfg!(feature = "case-insensitive") {alpha_alphanum_alpha} else {alpha_alphanum_upper_alpha}),
        preceded(tag("."), tag("%")),
     ))
    (input)
}

#[cfg(test)]
mod parse_rel_seq {
    use super::*;

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_relseq_lower() {
        let ls = parse_rel_seq(".rd");
        assert_eq!(ls, Err(NomErr::Error(("rd", ErrorKind::Tag))))
    }  

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_relseq_lower() {
        let ls = parse_rel_seq(".rd");
        assert_eq!(ls, Ok(("","rd")))
    }  
    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_seq_capital() {
        let ls = parse_rel_seq(".RD");
        assert_eq!(ls, Ok(("","RD")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_wildcard() {
        let ls = parse_rel_seq(".%");
        assert_eq!(ls, Ok(("","%")))
    }  
}


//--------------------------//
//  parse_rel_assetdev_seq  //
//--------------------------//
// parse relative assetdev sequence, case sensitive version
// EG .ASSETDEV
#[inline]
#[cfg(not(feature = "case-insensitive"))]
fn parse_rel_assetdev_seq(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), tag("ASSETDEV"))
    (input)
}


//--------------------------//
//  parse_rel_assetdev_seq  //
//--------------------------//
// parse relative assetdev sequence, case insensitive version
// EG .assetdev or .ASSETDEV
#[inline]
#[cfg(feature = "case-insensitive")]
fn parse_rel_assetdev_seq(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), tag_no_case("assetdev"))
    (input)
}

#[cfg(test)]
mod parse_rel_assetdev_seq {
    use super::*;

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_relseq_lower() {
        let ls = parse_rel_assetdev_seq(".assetdev");
        assert_eq!(ls, Err(NomErr::Error(("assetdev", ErrorKind::Tag))))
    }  

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_relseq_lower() {
        let ls = parse_rel_assetdev_seq(".assetdev");
        assert_eq!(ls, Ok(("","assetdev")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_seq_capital() {
        let ls = parse_rel_assetdev_seq(".ASSETDEV");
        assert_eq!(ls, Ok(("","ASSETDEV")))
    }  

}


//---------------------------//
//    parse_rel_seq_rel      //
//---------------------------//
// parse relative sequence with trailing relative 
// EG .RD.
#[inline]
fn parse_rel_seq_rel(input: &str) -> IResult<&str, &str> {
    terminated( 
        alt((
            parse_rel_assetdev_seq, 
            parse_rel_seq
        )), 
        tag(".")
    )
    (input)
}

#[cfg(test)]
mod parse_rel_seq_rel {
    use super::*;

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_relseq_lower() {
        let ls = parse_rel_seq_rel(".rd.");
        assert_eq!(ls, Err(NomErr::Error(("rd.", ErrorKind::Tag))))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn cannot_parse_relseq_assetdev_lower() {
        let ls = parse_rel_seq_rel(".assetdev.");
        assert_eq!(ls, Err(NomErr::Error(("assetdev.", ErrorKind::Tag))))
    }  

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_relseq_lower() {
        let ls = parse_rel_seq_rel(".rd.");
        assert_eq!(ls, Ok(("","rd")))
    }  

    #[test]
    #[cfg(feature = "case-insensitive")]
    fn can_parse_relseq_assetdev_lower() {
        let ls = parse_rel_seq_rel(".assetdev.");
        assert_eq!(ls, Ok(("","assetdev")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_seq_capital() {
        let ls = parse_rel_seq_rel(".RD.");
        assert_eq!(ls, Ok(("","RD")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_seq_assetdev_capital() {
        let ls = parse_rel_seq_rel(".ASSETDEV.");
        assert_eq!(ls, Ok(("","ASSETDEV")))
    }  

    #[test]
    #[cfg(not(feature = "case-insensitive"))]
    fn can_parse_wildcard() {
        let ls = parse_rel_seq_rel(".%.");
        assert_eq!(ls, Ok(("","%")))
    }  
}


//---------------------//
//   parse_rel_shot    //
//---------------------//
// parse relative shot
// EG ..0001
#[inline]
fn parse_rel_shot(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), parse_shot )(input)
}

#[cfg(test)]
mod parse_rel_shot {
    use super::*;

    #[test]
    fn can_parse_relshot_lower() {
        let ls = parse_rel_shot("..0001");
        assert_eq!(ls, Ok(("","0001")))
    }  

    #[test]
    fn can_parse_wildcard() {
        let ls = parse_rel_shot("..%");
        assert_eq!(ls, Ok(("","%")))
    }  
}


//----------------------//
//       shot_alt       //
//----------------------// 
// The shot alternative, has a show a sequence, and a shot
// accumulated into a vector. 
#[inline]
// EG DEV01.RD.0001
fn shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    map( //used to turn the tuple into a vector
        alt((
            tuple((parse_show, parse_assetdev_seq, parse_assetdev_shot)),
            tuple((parse_show, parse_seq, parse_shot)),
        )),
        |item| {
            let (show, seq, shot) = item;
            let mut acc = Vec::with_capacity(3);
            acc.push(show); 
            acc.push(seq); 
            acc.push(shot);
            acc
        }
    )
    (input)
}

#[cfg(test)]
mod shot_alt {
    use super::*;

    #[test]
    fn can_parse() {
        let ls = shot_alt("DEV01.RS.0001");
        assert_eq!(ls, Ok(("",vec!["DEV01", "RS", "0001"])))
    }

    #[test]
    fn can_parse_assetdev() {
        let ls = shot_alt("DEV01.ASSETDEV.FOOBAR");
        assert_eq!(ls, Ok(("",vec!["DEV01", "ASSETDEV", "FOOBAR"])))
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = shot_alt("dev01.assetdev.foobar");
        assert_eq!(ls, Ok(("", vec!["dev01", "assetdev", "foobar"])));
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn cannot_parse_assetdev_lowercase() {
        let ls = shot_alt("dev01.assetdev.foobar");
        assert_eq!(ls, Err(NomErr::Error(("dev01.assetdev.foobar", ErrorKind::Tag))));
    }

    #[test]
    fn cannot_start_with_letter() {
        let ls = shot_alt("DEV01.RD.R0001");
        assert_eq!(ls, Err(NomErr::Error(("R0001", ErrorKind::Tag))));
    }
    
    #[test]
    fn cannot_have_space() {
        let ls = shot_alt("DEV01.RD.0 001");
        assert_eq!(ls,  Ok((" 001", vec!["DEV01", "RD", "0"])));
    }
    
    #[test]
    fn cannot_have_wildcard_and_chars() {
        let ls = shot_alt("DEV01.RD.00%");
        assert_eq!(ls, Ok(("%", vec!["DEV01", "RD", "00"])));
    }

    #[test]
    fn cannot_have_underscore() {
        let ls = shot_alt("DEV01.RD.0_001");
        assert_eq!(ls, Ok(("_001", vec!["DEV01", "RD", "0"])));
    }

    #[test]
    fn can_parse_wildcard() {
        let ls = shot_alt("DEV01.RS.%");
        assert_eq!(ls, Ok(("", vec!["DEV01", "RS", "%"])));
    }
}

//-----------------------//
//       seq_alt         //
//-----------------------//
// the sequence alternative has a show and a sequence
// separated by a period, accumulated into a vector
#[inline]
// EG DEV01.RD
fn seq_alt(input: &str) -> IResult<&str, Vec<&str>> {
    map(
        tuple((parse_show, parse_seq)),
        | item| {
            let mut acc = Vec::with_capacity(2);
            let (show, seq) = item ;
            acc.push(show); 
            acc.push(seq);
            acc
        } 
    )
    (input)
}

#[cfg(test)]
mod seq_alt {
    use super::*;
        
    #[test]
    fn can_parse() {
        let ls = seq_alt("DEV01.RD");
        assert_eq!(ls, Ok(("",vec!["DEV01", "RD"])));
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_lowercase() {
        let ls = seq_alt("dev01.rd");
        assert_eq!(ls, Ok(("", vec!["dev01", "rd"])));
    }

    #[test]
    fn can_parse_assetdev() {
        let ls = seq_alt("DEV01.ASSETDEV");
        assert_eq!(ls, Ok(("", vec!["DEV01", "ASSETDEV"])))
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = seq_alt("dev01.assetdev");
        assert_eq!(ls, Ok(("", vec!["dev01", "assetdev"])))
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = seq_alt("dev01.assetdev");
        assert_eq!(ls, Err(NomErr::Error(("dev01.assetdev", ErrorKind::Tag))));
    }

    #[test]
    fn cannot_start_with_number() {
        let ls = seq_alt("DEV01.1D");
        assert_eq!(ls, Err(NomErr::Error(("1D", ErrorKind::Tag))));
    }
    
    #[test]
    fn cannot_have_space() {
        let ls = seq_alt("DEV01.R D");
        assert_eq!(ls, Ok((" D", vec!["DEV01", "R"])));
    }
    
    #[test]
    fn cannot_have_wildcard_and_chars() {
        let ls = seq_alt("DEV01.R%");
        assert_eq!(ls, Ok(("%", vec!["DEV01", "R"])));
    }

    #[test]
    fn cannot_have_underscore() {
        let ls = seq_alt("DEV01.R_D");
        assert_eq!(ls, Ok(("_D", vec!["DEV01", "R"])));
    }

    #[test]
    fn can_parse_wildcard() {
        let ls = seq_alt("DEV01.%");
        assert_eq!(ls, Ok(("",vec!["DEV01","%"])));
    }
}


//-----------------------//
//       show_alt        //
//-----------------------//
#[inline]
// EG DEV01
fn show_alt(input: &str) -> IResult<&str, Vec<&str>> {
    // unlike the other levels, we cannot keep parsing until we are done, as 
    // 
    map(parse_show, |item| { 
            let mut acc = Vec::new();
            acc.push(item); 
            acc
        } 
    )
    (input)
}


#[cfg(test)]
mod show_alt {
    use super::*;
        
    #[test]
    fn can_parse() {
        let ls = show_alt("DEV01");
        assert_eq!(ls, Ok(("",vec!["DEV01"])));
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_lowercase() {
        let ls = show_alt("dev01");
        assert_eq!(ls, Ok(("", vec!["dev01"])));
    }

    #[test]
    fn cannot_start_with_number() {
        let ls = show_alt("1DEV01");
        assert_eq!(ls, Err(NomErr::Error(("1DEV01", ErrorKind::Tag))));
    }
    
    #[test]
    fn cannot_have_space() {
        let ls = show_alt("DEV 01");
        assert_eq!(ls, Ok((" 01", vec!["DEV"])));
    }
    
    #[test]
    fn cannot_have_wildcard_and_chars() {
        let ls = show_alt("DEV01%");
        assert_eq!(ls, Ok(("%", vec!["DEV01"])));
    }

    #[test]
    fn cannot_have_underscore() {
        let ls = show_alt("DEV01_D");
        assert_eq!(ls, Ok(("_D", vec!["DEV01"])));
    }

    #[test]
    fn can_parse_wildcard() {
        let ls = show_alt("%");
        assert_eq!(ls, Ok(("",vec!["%"])));
    }
}

//--------------------//
//    rel_seq_alt     //
//--------------------//
// the sequence alternative has a show and a sequence
// separated by a period, accumulated into a vector
#[inline]
// .RD
fn rel_seq_alt(input: &str) -> IResult<&str, Vec<&str>> {
    map( //used to turn the tuple into a vector
        alt((
            parse_rel_assetdev_seq,
            parse_rel_seq,
        )),
        |item| {
            let mut acc = Vec::with_capacity(2);
            acc.push(""); 
            acc.push(item);
            acc
        } 
    )
    (input)
}


#[cfg(test)]
mod rel_seq_alt {
    use super::*;
        
    #[test]
    fn can_parse() {
        let ls = rel_seq_alt(".RD");
        assert_eq!(ls, Ok(("",vec!["", "RD"])));
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_lowercase() {
        let ls = rel_seq_alt(".rd");
        assert_eq!(ls, Ok(("", vec!["", "rd"])));
    }

    #[test]
    fn can_parse_assetdev() {
        let ls = rel_seq_alt(".ASSETDEV");
        assert_eq!(ls, Ok(("", vec!["", "ASSETDEV"])))
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = rel_seq_alt(".assetdev");
        assert_eq!(ls, Ok(("", vec!["", "assetdev"])))
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = rel_seq_alt(".assetdev");
        assert_eq!(ls, Err(NomErr::Error(("assetdev", ErrorKind::Tag))));
    }

    #[test]
    fn cannot_start_with_number() {
        let ls = rel_seq_alt(".1D");
        assert_eq!(ls, Err(NomErr::Error(("1D", ErrorKind::Tag))));
    }
    
    #[test]
    fn cannot_have_space() {
        let ls = rel_seq_alt(".R D");
        assert_eq!(ls, Ok((" D", vec!["", "R"])));
    }
    
    #[test]
    fn cannot_have_wildcard_and_chars() {
        let ls = rel_seq_alt(".R%");
        assert_eq!(ls, Ok(("%", vec!["", "R"])));
    }

    #[test]
    fn cannot_have_underscore() {
        let ls = rel_seq_alt(".R_D");
        assert_eq!(ls, Ok(("_D", vec!["", "R"])));
    }

    #[test]
    fn can_parse_wildcard() {
        let ls = rel_seq_alt(".%");
        assert_eq!(ls, Ok(("",vec!["","%"])));
    }
}

//---------------------//
//   rel_seq_rel_alt   //
//---------------------//
#[inline]
// EG .RD.
fn rel_seq_rel_alt(input: &str) -> IResult<&str, Vec<&str>> {
    fold_many1( //used to turn the tuple into a vector
        //terminated(parse_rel_seq, tag(".")),
        parse_rel_seq_rel,
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

#[cfg(test)]
mod rel_seq_rel_alt {
    use super::*;
        
    #[test]
    fn can_parse() {
        let ls = rel_seq_rel_alt(".RD.");
        assert_eq!(ls, Ok(("",vec!["", "RD", ""])));
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_lowercase() {
        let ls = rel_seq_rel_alt(".rd.");
        assert_eq!(ls, Ok(("", vec!["", "rd", ""])));
    }

    #[test]
    fn can_parse_assetdev() {
        let ls = rel_seq_rel_alt(".ASSETDEV.");
        assert_eq!(ls, Ok(("", vec!["", "ASSETDEV", ""])))
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = rel_seq_rel_alt(".assetdev.");
        assert_eq!(ls, Ok(("", vec!["", "assetdev", ""])))
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn can_parse_assetdev_lowercase() {
        let ls = rel_seq_rel_alt(".assetdev.");
        assert_eq!(ls, Err(NomErr::Error((".assetdev.", ErrorKind::Many1))));
    }

    #[test]
    fn cannot_start_with_number() {
        let ls = rel_seq_rel_alt(".1D.");
        assert_eq!(ls, Err(NomErr::Error((".1D.", ErrorKind::Many1))));
    }
    
    #[test]
    fn cannot_have_space() {
        let ls = rel_seq_rel_alt(".R D.");
        assert_eq!(ls,Err(NomErr::Error((".R D.", ErrorKind::Many1))));
    }
    
    #[test]
    fn cannot_have_wildcard_and_chars() {
        let ls = rel_seq_rel_alt(".R%.");
        assert_eq!(ls, Err(NomErr::Error((".R%.", ErrorKind::Many1))));
    }

    #[test]
    fn cannot_have_underscore() {
        let ls = rel_seq_rel_alt(".R_D.");
        assert_eq!(ls, Err(NomErr::Error((".R_D.", ErrorKind::Many1))));
    }

    #[test]
    fn can_parse_wildcard() {
        let ls = rel_seq_rel_alt(".%.");
        assert_eq!(ls, Ok(("",vec!["","%", ""])));
    }
}


//----------------------//
//   rel_seq_shot_alt   //
//----------------------//
#[inline]
// EG .RD.0001
fn rel_seq_shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    map( //used to turn the tuple into a vector
        tuple((parse_rel_seq, parse_shot)),
        |item| {
            let (seq, shot) = item;
            let mut acc = Vec::with_capacity(3);
            acc.push(""); 
            acc.push(seq);
            acc.push(shot); 
            acc
        } 
    )
    (input)
}


//------------------------//
//    show_seq_rel_alt    //
//------------------------//
#[inline]
// EG DEV01.RD.
fn show_seq_rel_alt(input: &str) -> IResult<&str, Vec<&str>> {
    map( //used to turn the tuple into a vector
        tuple((parse_show, terminated(parse_seq, tag(".")))), 
        |item| {
            let (show, seq) = item;
            let mut acc = Vec::with_capacity(3);
            acc.push(show);
            acc.push(seq); 
            acc.push(""); 
            acc
        } 
    )
    (input)
}


//----------------------//
//    rel_shot_alt      //
//----------------------//
#[inline]
// EG ..0001
fn rel_shot_alt(input: &str) -> IResult<&str, Vec<&str>> {
    map( //used to place into a vector
        parse_rel_shot, 
        | item| { 
            let mut acc = Vec::with_capacity(3);
            acc.push("");
            acc.push("");
            acc.push(item); 
            acc
        } 
    )
    (input)
}


//------------------------//
//       levelparser      //
//------------------------//
fn levelparser(input: &str) -> IResult<&str, Vec<&str>> {
    let (leftover, result) = all_consuming(
        alt(( // order is critical fyi
            rel_shot_alt,
            rel_seq_shot_alt,
            rel_seq_rel_alt,
            rel_seq_alt,
            shot_alt,
            show_seq_rel_alt,
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
        fn can_parse_rel_shot() {
            let ls = rel_shot_alt("..0001");
            assert_eq!(ls, Ok(("",vec!["", "", "0001"])))
        }  

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
        fn can_parse_show_seq_rel() {
            let ls = levelspec_parser("DEV01.RD.");
            assert_eq!(ls, Ok(vec!["DEV01", "RD", ""]))
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
mod parse_level {
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