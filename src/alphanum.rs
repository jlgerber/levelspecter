
use nom::{
    error::ParseError,
    InputTakeAtPosition,
    AsChar,
    IResult,
    character::complete::{anychar, alpha1},
    combinator::{all_consuming, complete},
    error::ErrorKind,
    Err,
};

pub trait AsCharCaseSensitive : AsChar {

    #[inline]
    fn is_alpha_lower(self) -> bool;

    #[inline]
    fn is_alpha_upper(self) -> bool;

    #[inline]
    fn is_alphanum_lower(self) -> bool;

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

pub fn loweralpha1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alpha_lower(), ErrorKind::Alpha)
}

pub fn loweralphanum1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alphanum_lower() , ErrorKind::AlphaNumeric)
}


pub fn loweralpha0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alpha_lower())
}

pub fn loweralphanum0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alphanum_lower())
}

pub fn upperalpha1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alpha_upper(), ErrorKind::Alpha)
}

pub fn upperalphanum1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position1_complete(|item| !item.is_alphanum_upper(), ErrorKind::Alpha)
}

pub fn upperalpha0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alpha_upper())
}

pub fn upperalphanum0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
  T: InputTakeAtPosition,
  <T as InputTakeAtPosition>::Item: AsCharCaseSensitive,
{
  input.split_at_position_complete(|item| !item.is_alphanum_upper())
}