
use nom::{
    error::ParseError,
    InputTakeAtPosition,
    AsChar,
    IResult,
    character::complete::{alpha1, alphanumeric0},
    combinator::{ recognize},
    error::ErrorKind,
    sequence::tuple,
};

/// AsCharCaseSensitive extends nom::AsChar, providing
/// case sensitive analogs of a subset of methods found in 
/// the AsChar trait.
pub trait AsCharCaseSensitive : AsChar {
    /// Is the provided character a lowercase letter?
    #[inline]
    fn is_alpha_lower(self) -> bool;

    /// Is the provided character an uppercase letter?
    #[inline]
    fn is_alpha_upper(self) -> bool;

    /// Is the provided character a lowercase letter or number?
    #[inline]
    fn is_alphanum_lower(self) -> bool;

    /// Is the provided character an uppercase letter or number?
    #[inline]
    fn is_alphanum_upper(self) -> bool;
}

impl AsCharCaseSensitive for u8 {
    
    #[inline]
    fn is_alpha_lower(self) -> bool {
        (self >= 0x61 && self <= 0x7A)
    }

    #[inline]
    fn is_alpha_upper(self) -> bool {
        (self >= 0x41 && self <= 0x5A)
    }

    #[inline]
    fn is_alphanum_lower(self) -> bool {
        self.is_alpha_lower() || self.is_dec_digit()
    }

    #[inline]
    fn is_alphanum_upper(self) -> bool {
        self.is_alpha_upper() || self.is_dec_digit()
    }
}

impl<'a> AsCharCaseSensitive for &'a u8 {

    #[inline]
    fn is_alpha_lower(self) -> bool {
        (*self >= 0x61 && *self <= 0x7A)
    }

    #[inline]
    fn is_alpha_upper(self) -> bool {
        (*self >= 0x41 && *self <= 0x5A)
    }

    #[inline]
    fn is_alphanum_lower(self) -> bool {
        self.is_alpha_lower() || self.is_dec_digit()
    }

    #[inline]
    fn is_alphanum_upper(self) -> bool {
        self.is_alpha_upper() || self.is_dec_digit()
    }
}

impl AsCharCaseSensitive for char {
    
    #[inline]
    fn is_alpha_lower(self) -> bool {
        (self as u8 >= 0x61 && self as u8 <= 0x7A)
    }

    #[inline]
    fn is_alpha_upper(self) -> bool {
        (self as u8 >= 0x41 && self as u8 <= 0x5A)
    }

    #[inline]
    fn is_alphanum_lower(self) -> bool {
        self.is_alpha_lower() || self.is_dec_digit()
    }

    #[inline]
    fn is_alphanum_upper(self) -> bool {
        self.is_alpha_upper() || self.is_dec_digit()
    }
}

impl<'a> AsCharCaseSensitive for &'a char {
    
    #[inline]
    fn is_alpha_lower(self) -> bool {
        (*self as u8 >= 0x61 && *self as u8 <= 0x7A)
    }

    #[inline]
    fn is_alpha_upper(self) -> bool {
        (*self as u8 >= 0x41 && *self as u8 <= 0x5A)
    }

    #[inline]
    fn is_alphanum_lower(self) -> bool {
        self.is_alpha_lower() || self.is_dec_digit()
    }

    #[inline]
    fn is_alphanum_upper(self) -> bool {
        self.is_alpha_upper() || self.is_dec_digit()
    }
}

/// Parser which accepts lone or more lowercase letters
pub fn loweralpha1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alpha_lower(), ErrorKind::Alpha)
}

/// Parser which accepts one or more lowercase letters or numbers
pub fn loweralphanum1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alphanum_lower() , ErrorKind::AlphaNumeric)
}

/// Parser which accepts zero or more lowercase letters
pub fn loweralpha0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alpha_lower())
}

/// Parser which accepts zero or more lowercase letters or numbers
pub fn loweralphanum0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alphanum_lower())
}

/// Parser which accepts one or more uppercase letters
pub fn upperalpha1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alpha_upper(), ErrorKind::Alpha)
}

/// Parser which accepts one or more uppercase letters or numbers
pub fn upperalphanum1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alphanum_upper(), ErrorKind::AlphaNumeric)
}

/// Parser which accepts zero or more uppercase letters 
pub fn upperalpha0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alpha_upper())
}

/// Parser which accepts zero or more uppercase letters or numbers
pub fn upperalphanum0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alphanum_upper())
}

/// a letter followed by zero or more letters and numbers
pub fn alpha_alphanum(input: &str) -> IResult<&str, &str> {
    recognize(tuple((alpha1, alphanumeric0)))(input)
}

/// an uppercase letter followed by zero or more uppercase letters and numbers
pub fn alpha_alphanum_upper(input: &str) -> IResult<&str, &str> {
    recognize(tuple((upperalpha1, upperalphanum0)))(input)
}

/// a lowercase letter followed by zero or more lowercase letters and numbers
pub fn alpha_alphanum_lower(input: &str) -> IResult<&str, &str> {
    recognize(tuple((loweralpha1, loweralphanum0)))(input)
}



#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Err};
    use nom::error::ErrorKind::{Alpha, AlphaNumeric};

    //-----------------------//
    //     LOWER ALPHA  1    //
    //-----------------------//

    #[test]
    fn loweralpha1_succeeds_with_expected_input() {
        let la: IResult<&str, &str> = loweralpha1("thisisatest");
        assert_eq!(la, Ok(("","thisisatest")));
    }

    #[test]
    fn loweralpha1_fails_with_numeric_input() {
        let la: IResult<&str, &str> = loweralpha1("1thisisatest");
        assert_eq!(la, Err(Err::Error(("1thisisatest", Alpha))));
    }

    #[test]
    fn loweralpha1_fails_with_uppercase_input() {
        let la: IResult<&str, &str> = loweralpha1("Thisisatest");
        assert_eq!(la, Err(Err::Error(("Thisisatest", Alpha))));
    }

    #[test]
    fn loweralpha1_fails_with_no_input() {
        let la: IResult<&str, &str> = loweralpha1("");
        assert_eq!(la, Err(Err::Error(("", Alpha))));
    }

    //-----------------------//
    //   LOWER ALPHA NUM 1   //
    //-----------------------//

    #[test]
    fn loweralphanum1_succeeds_with_lowercase_alpha_input() {
        let la: IResult<&str, &str> = loweralphanum1("thisisatest");
        assert_eq!(la, Ok(("","thisisatest")));
    }

    #[test]
    fn loweralphanum1_succeeds_with_numeric_input() {
        let la: IResult<&str, &str> = loweralphanum1("1thisisatest");
        assert_eq!(la, Ok(("","1thisisatest")));
    }

    #[test]
    fn loweralphanum1_fails_with_uppercase_input() {
        let la: IResult<&str, &str> = loweralphanum1("Thisisatest");
        assert_eq!(la, Err(Err::Error(("Thisisatest", AlphaNumeric))));
    }

    #[test]
    fn loweralphanum1_fails_with_no_input() {
        let la: IResult<&str, &str> = loweralphanum1("");
        assert_eq!(la, Err(Err::Error(("", AlphaNumeric))));
    }

    //-----------------------//
    //    LOWER ALPHA 0      //
    //-----------------------//

    #[test]
    fn loweralpha0_succeeds_with_expected_input() {
        let la: IResult<&str, &str> = loweralpha0("thisisatest");
        assert_eq!(la, Ok(("","thisisatest")));
    }

    #[test]
    fn loweralpha0_doenst_make_progress_with_numeric_input() {
        let la: IResult<&str, &str> = loweralpha0("1thisisatest");
        assert_eq!(la, Ok(("1thisisatest", "")));
    }

    #[test]
    fn loweralpha0_doesnt_make_progress_with_uppercase_input() {
        let la: IResult<&str, &str> = loweralpha0("Thisisatest");
        assert_eq!(la, Ok(("Thisisatest", "")));
    }

    #[test]
    fn loweralpha0_succeeds_with_no_input() {
        let la: IResult<&str, &str> = loweralpha0("");
        assert_eq!(la, Ok(("",""))) ;
    }

    //-----------------------//
    //   LOWER ALPHA NUM 0   //
    //-----------------------//

    #[test]
    fn loweralphanum0_succeeds_with_expected_input() {
        let la: IResult<&str, &str> = loweralphanum0("thisisatest");
        assert_eq!(la, Ok(("","thisisatest")));
    }

    #[test]
    fn loweralphanum0_makes_progress_with_numeric_input() {
        let la: IResult<&str, &str> = loweralphanum0("1thisisatest");
        assert_eq!(la, Ok(("", "1thisisatest")));
    }

    #[test]
    fn loweralphanum0_doesnt_make_progress_with_uppercase_input() {
        let la: IResult<&str, &str> = loweralphanum0("Thisisatest");
        assert_eq!(la, Ok(("Thisisatest", "")));
    }

    #[test]
    fn loweralphanum0_succeeds_with_no_input() {
        let la: IResult<&str, &str> = loweralphanum0("");
        assert_eq!(la, Ok(("",""))) ;
    }

    //-----------------------//
    //     UPPER ALPHA 1     //
    //-----------------------//

    #[test]
    fn upperalpha1_succeeds_with_expected_input() {
        let la: IResult<&str, &str> = upperalpha1("THISISATEST");
        assert_eq!(la, Ok(("","THISISATEST")));
    }

    #[test]
    fn upperalpha1_fails_with_numeric_input() {
        let la: IResult<&str, &str> = upperalpha1("1THISISATEST");
        assert_eq!(la, Err(Err::Error(("1THISISATEST", Alpha))));
    }

    #[test]
    fn upperalpha1_fails_with_lowercase_input() {
        let la: IResult<&str, &str> = upperalpha1("tHISISATEST");
        assert_eq!(la, Err(Err::Error(("tHISISATEST", Alpha))));
    }

    #[test]
    fn upperalpha1_fails_with_no_input() {
        let la: IResult<&str, &str> = upperalpha1("");
        assert_eq!(la, Err(Err::Error(("", Alpha))));
    }

    //-----------------------//
    //   UPPER ALPHA NUM 1   //
    //-----------------------//

    #[test]
    fn upperalphanum1_succeeds_with_uppercase_alpha_input() {
        let la: IResult<&str, &str> = upperalphanum1("THISISATEST");
        assert_eq!(la, Ok(("","THISISATEST")));
    }

    #[test]
    fn upperalphanum1_succeeds_with_numeric_input() {
        let la: IResult<&str, &str> = upperalphanum1("1THISISATEST");
        assert_eq!(la, Ok(("","1THISISATEST")));
    }

    #[test]
    fn upperalphanum1_fails_with_lowercase_input() {
        let la: IResult<&str, &str> = upperalphanum1("tHISISATEST");
        assert_eq!(la, Err(Err::Error(("tHISISATEST", AlphaNumeric))));
    }

    #[test]
    fn upperalphanum1_fails_with_no_input() {
        let la: IResult<&str, &str> = upperalphanum1("");
        assert_eq!(la, Err(Err::Error(("", AlphaNumeric))));
    }

    //-----------------------//
    //     UPPER ALPHA 0     //
    //-----------------------//    

    #[test]
    fn upperalpha0_succeeds_with_expected_input() {
        let la: IResult<&str, &str> = upperalpha0("THISISATEST");
        assert_eq!(la, Ok(("","THISISATEST")));
    }

    #[test]
    fn upperalpha0_doenst_make_progress_with_numeric_input() {
        let la: IResult<&str, &str> = upperalpha0("1THISISATEST");
        assert_eq!(la, Ok(("1THISISATEST", "")));
    }

    #[test]
    fn upperalpha0_doesnt_make_progress_with_lowercase_input() {
        let la: IResult<&str, &str> = upperalpha0("tHISISATEST");
        assert_eq!(la, Ok(("tHISISATEST", "")));
    }

    #[test]
    fn upperalpha0_succeeds_with_no_input() {
        let la: IResult<&str, &str> = upperalpha0("");
        assert_eq!(la, Ok(("",""))) ;
    }

    //-----------------------//
    //   UPPER ALPHA NUM 0   //
    //-----------------------// 

    #[test]
    fn upperalphanum0_succeeds_with_expected_input() {
        let la: IResult<&str, &str> = upperalphanum0("THISISATEST");
        assert_eq!(la, Ok(("","THISISATEST")));
    }

    #[test]
    fn upperalphanum0_makes_progress_with_numeric_input() {
        let la: IResult<&str, &str> = upperalphanum0("1THISISATEST");
        assert_eq!(la, Ok(("", "1THISISATEST")));
    }

    #[test]
    fn upperalphanum0_doesnt_make_progress_with_lowercase_input() {
        let la: IResult<&str, &str> = upperalphanum0("1tHISISATEST");
        assert_eq!(la, Ok(("tHISISATEST", "1")));
    }

    #[test]
    fn upperalphanum0_succeeds_with_no_input() {
        let la: IResult<&str, &str> = upperalphanum0("");
        assert_eq!(la, Ok(("",""))) ;
    }

    //-----------------------//
    //    ALPHA ALPHANUM     //
    //-----------------------// 

    #[test]
    fn alpha_alphanum_succeeds_with_letter_followed_by_number_and_letters() {
        let la: IResult<&str, &str> = alpha_alphanum("f1bar");
        assert_eq!(la, Ok(("","f1bar"))) ;
    }

    #[test]
    fn alpha_alphanum_succeeds_with_uppercase_letter_followed_by_number_and_uppercase_letters() {
        let la: IResult<&str, &str> = alpha_alphanum("F1BAR");
        assert_eq!(la, Ok(("","F1BAR"))) ;
    }

    #[test]
    fn alpha_alphanum_fails_with_number_followed_by_numbers_and_letters() {
        let la: IResult<&str, &str> = alpha_alphanum("1f1Bar");
        assert_eq!(la, Err(Err::Error(("1f1Bar", Alpha)))) ;
    }

    //-----------------------//
    // ALPHA ALPHANUM UPPER  //
    //-----------------------// 

    #[test]
    fn alpha_alphanum_upper_fails_with_lowercase_letter_followed_by_number_and_letters() {
        let la: IResult<&str, &str> = alpha_alphanum_upper("f1BAR");
        assert_eq!(la, Err(Err::Error(("f1BAR", Alpha)))) ;
    }

    #[test]
    fn alpha_alphanum_upper_succeeds_with_uppercase_letter_followed_by_number_and_uppercase_letters() {
        let la: IResult<&str, &str> = alpha_alphanum_upper("F1BAR");
        assert_eq!(la, Ok(("","F1BAR"))) ;
    }

    #[test]
    fn alpha_alphanum_upper_fails_with_number_followed_by_numbers_and_uppercase_letters() {
        let la: IResult<&str, &str> = alpha_alphanum_upper("1F1BAR");
        assert_eq!(la, Err(Err::Error(("1F1BAR", Alpha)))) ;
    }


    //-----------------------//
    // ALPHA ALPHANUM LOWER  //
    //-----------------------// 

    #[test]
    fn alpha_alphanum_lower_fails_with_uppercase_letter_followed_by_number_and_letters() {
        let la: IResult<&str, &str> = alpha_alphanum_lower("F1BAR");
        assert_eq!(la, Err(Err::Error(("F1BAR", Alpha)))) ;
    }

    #[test]
    fn alpha_alphanum_lower_succeeds_with_lowercase_letter_followed_by_number_and_lowercase_letters() {
        let la: IResult<&str, &str> = alpha_alphanum_lower("f1bar");
        assert_eq!(la, Ok(("","f1bar"))) ;
    }

    #[test]
    fn alpha_alphanum_lower_fails_with_number_followed_by_numbers_and_lowercase_letters() {
        let la: IResult<&str, &str> = alpha_alphanum_lower("1f1bar");
        assert_eq!(la, Err(Err::Error(("1f1bar", Alpha)))) ;
    }

}

