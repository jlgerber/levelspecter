use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming, recognize},
    bytes::complete::{tag},
    sequence::{tuple, preceded },
    multi::many_m_n,
};

use aschar_casesensitive::{ upperalphanum1, alpha_alphanum};


// fn alpha_alphanum(input: &str) -> IResult<&str, &str> {
//     recognize(tuple((upperalpha1, upperalphanum0)))(input)
// }

fn levelparser(input: &str) -> IResult<&str, String> {
    let result = all_consuming(
        tuple(( // inner tuple start
            alpha_alphanum, // show has to start with a letter
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

pub fn levelspec_parser(input: &str) -> Result<String, String> {
    match levelparser(input) {
        Err(_) => Err(format!("Unable to parse levelspec for {}", input)),
        Ok((_,ls)) => Ok(ls),
    }
}