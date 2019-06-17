use nom::{
    IResult,
    combinator::{all_consuming},
    bytes::complete::{tag},
    sequence::{tuple, Tuple, preceded },
    multi::many_m_n,
};

use levelspecter::*;

fn all_lower_parser(input: &str) -> IResult<&str, &str> {
    all_consuming(loweralpha1)(input)
}

fn all_lower_parser2(input: &str) -> IResult<&str, &str> {
    all_consuming(loweralphanum1)(input)
}

fn all_upper_parser(input: &str) -> IResult<&str, &str> {
    all_consuming(upperalpha1)(input)
}

fn all_upper_parser2(input: &str) -> IResult<&str, &str> {
    all_consuming(upperalpha1)(input)
}

fn shotparser(input: &str) -> IResult<&str, String> {
    let result = tuple((upperalphanum1, preceded(tag("."),upperalphanum1), preceded(tag("."), upperalphanum1) ))(input)?;
    if let (_,(show,seq,shot)) = result {
        return Ok(("", format!("{}.{}.{}", show, seq, shot)));
    }
    Ok(("","foo".to_string()))

}


fn levelparser(input: &str) -> IResult<&str, String> {
    let result = tuple((upperalphanum1, many_m_n(0,2,preceded(tag("."),upperalphanum1))  ))(input)?;
    if let (_,(show,seqshot)) = result {
        let seqshot = if seqshot.len() > 0 { format!(".{}",seqshot.join(".")) } else {"".to_owned()};
    
        return Ok(("", format!("{}{}", show,seqshot)));
    }
    Ok(("","foo".to_string()))

}

fn test() {
    use nom::bytes::complete::{tag};
    fn abcd_parser(i: &str) -> IResult<&str, &str> {
    tag("abcd")(i) // will consume bytes if the input begins with "abcd"
    }

    let result = abcd_parser("abcdabcdef");
    println!("abcd parser {:?}", result);
}

fn main() {

    //let parser = all_consuming(alpha1);
    let result = all_lower_parser("abcdef");
    println!("all lower parser(abcdef) {:?}",result);

     let result = all_lower_parser2("abcdef123");
    println!("all lower parser(abcdef123) {:?}",result);
    
    let result = all_upper_parser("ABCDEF");
    println!("all upper parser(ABCDEF) {:?}",result);

    let result = shotparser("DEV01.RD.9999");
    println!("shotparser(\"DEV01.RD.9999\") {:?}",result);

    println!("levelparser(\"DEV01.RD\") {:?}",levelparser("DEV01.RD"));
    println!("levelparser(\"DEV01\") {:?}",levelparser("DEV01"));


    //test()
}
