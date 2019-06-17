use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming, recognize},
    bytes::complete::{tag},
    sequence::{tuple, preceded },
    multi::many_m_n,
};

use crate::{upperalphanum0, upperalpha1, upperalphanum1};


fn alpha_alphanum(input: &str) -> IResult<&str, &str> {
    recognize(tuple((upperalpha1, upperalphanum0)))(input)
}

fn levelparser(input: &str) -> IResult<&str, String> {
    let result = all_consuming(
        tuple(( // inner tuple start
            alpha_alphanum,
            //upperalphanum1, 
            many_m_n(
                0, 
                1, 
                alt((
                    recognize( 
                        tuple((
                            preceded(
                                tag("."),
                                alpha_alphanum
                            ), 
                            preceded(
                                tag("."), 
                                upperalphanum1
                            )
                        ))
                    ),
                    preceded(
                        tag("."),
                        alpha_alphanum
                    )
                 ))
            ) // many_m_n end  
        )) // tuple end
    )(input)?;
    
    let (_,(show, mut seqshot)) = result; 

    let seqshot = match seqshot.len() {
        2 => seqshot.join("."),
        1 =>  {
            let seqshot = seqshot.pop().unwrap();
            if seqshot.starts_with(".") {seqshot.to_string()} else {format!(".{}", seqshot)}
        },
        0 => "".to_owned(),
        _ => panic!("unexpected number")
    };
    Ok(("", format!("{}{}", show,seqshot)))
}

pub fn levelspecparser(input: &str) -> Result<String, String> {
    match levelparser(input) {
        Err(_) => Err(format!("Unable to parse levelspec for {}", input)),
        Ok((_,ls)) => Ok(ls),
    }
}