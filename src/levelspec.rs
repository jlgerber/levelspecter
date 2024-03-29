use crate::{LevelSpecterError as LSE, levelspec_parser, LevelType};
use  std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LevelName {
    Show,
    Sequence,
    Shot,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LevelSpec {
    pub show: LevelType,
    pub sequence: Option<LevelType>,
    pub shot: Option<LevelType>
}

impl LevelSpec {
    /// New up a LevelSpec from a str or string. This is the primary entrypoint for the crate. 
    /// 
    /// # Parameters
    /// 
    /// * `levelspec` - The string we wish to convert to a levelspec. This takes any type 
    ///                 which is AsRef<str>
    /// 
    /// # Returns
    /// A LevelSpec instance or error
    /// 
    /// # Example
    /// 
    /// ```
    /// use levelspecter::{LevelSpec};
    /// 
    /// let result = LevelSpec::new("DEV01.RD.0001");
    /// let expected = LevelSpec::from_shot("DEV01", "RD", "0001");
    /// assert_eq!(result, Ok(expected));
    /// ```
    pub fn new<I>(levelspec: I) -> Result<LevelSpec, LSE> 
    where
        I: AsRef<str> + std::fmt::Debug
    {
        LevelSpec::from_str(levelspec.as_ref())
    }
    
    /// Convert to uppercase
    pub fn set_upper(&mut self) {
        if let LevelType::Term(ref mut show) = self.show {*show = show.to_uppercase()}
        if let Some(LevelType::Term(ref mut sequence)) = self.sequence {*sequence = sequence.to_uppercase()}
        if let Some(LevelType::Term(ref mut shot)) = self.shot {*shot = shot.to_uppercase()}
    }

    /// Convert to uppercase and return self. Used to chain after from
    pub fn upper(mut self) -> Self {
        if let LevelType::Term(ref mut show) = self.show {*show = show.to_uppercase()}
        if let Some(LevelType::Term(ref mut sequence)) = self.sequence {*sequence = sequence.to_uppercase()}
        if let Some(LevelType::Term(ref mut shot)) = self.shot {*shot = shot.to_uppercase()}
        self
    }

    /// Return a new LevelSpec instance that removes any relative LevelTypes.
    /// rel_to_abs takes a closure to perform said magic
    /// 
    /// # Parameters
    /// 
    /// * `replacer` - Closure which takes a LevelName and returns an Option<String>
    /// 
    /// # Returns
    /// A new LevelSpec without any relative components if successful
    /// Otherwise, a LevelSpecterError
    /// 
    pub fn rel_to_abs<P>(&self, replacer: P ) -> Result<Self, LSE> 
    where
        P: Fn(LevelName) -> Option<String>
    {
        let mut return_value = self.clone();

        if return_value.show.is_relative() {
            let new_show_val = replacer(LevelName::Show)
                .ok_or(LSE::RelToAbsError(format!("Unable to retrieve {:?} in rel_to_abs",
                LevelName::Show)))?;
            let new_show = LevelType::from(new_show_val.as_ref());

            if new_show.is_relative() { 
                return Err(LSE::RelToAbsError(format!("show returned by closure is relative '{}'", new_show_val))) ;
            }

            return_value.show = new_show;
        }

        if let Some(ref seq) = return_value.sequence {
            if seq.is_relative() {
                let new_seq_val = replacer(LevelName::Sequence)
                .ok_or(LSE::RelToAbsError(format!("Unable to retrieve {:?} in rel_to_abs",
                LevelName::Sequence)))?;
                let new_seq = LevelType::from(new_seq_val.as_ref());

                if new_seq.is_relative() {
                    return Err(LSE::RelToAbsError(format!("sequence returned by closure is relative '{}'", new_seq_val))) ;
                }

                return_value.sequence = Some(new_seq);
            }
        }

        if let Some(ref shot) = return_value.shot {
            if shot.is_relative() {
                let new_shot_val = replacer(LevelName::Shot)
                .ok_or(LSE::RelToAbsError(format!("Unable to retrieve {:?} in rel_to_abs",
                LevelName::Shot)))?;
                let new_shot = LevelType::from(new_shot_val.as_ref());

                if new_shot.is_relative() {
                    return Err(LSE::RelToAbsError(format!("shot returned by closure is relative '{}'", new_shot_val))) ;
                }

                return_value.shot = Some(new_shot);
            }
        }
        Ok(return_value)
    }

    /// new up a show
    pub fn from_show<I>(input: I ) -> Self
    where 
        I: AsRef<str>
    {
        let ls = Self {
            show: LevelType::from(input.as_ref()), 
            sequence: None, 
            shot: None
        };
        if cfg!(feature = "case-insensitive") {ls} else {ls.upper()}
    }
    /// new up a sequence
    pub fn from_sequence<I>(show: I, sequence: I ) -> Self  
    where 
        I: AsRef<str>
    {
        let ls = Self {
            show: LevelType::from(show.as_ref()), 
            sequence: Some(LevelType::from(sequence.as_ref())), 
            shot: None
        };
        if cfg!(feature = "case-insensitive") {ls} else {ls.upper()}
    }

    pub fn from_shot<I>(show: I, sequence: I, shot: I) -> Self  
    where 
        I: AsRef<str>
    {
        let ls = Self {
            show: LevelType::from(show.as_ref()), 
            sequence: Some(LevelType::from(sequence.as_ref())), 
            shot: Some(LevelType::from(shot.as_ref()))
        };
        if cfg!(feature = "case-insensitive") {ls} else {ls.upper()}
    }

   pub fn is_concrete(&self) -> bool {
        if self.show.is_wildcard() {
           return false;
        }
        
        if let Some(ref ls) = self.sequence {
            if ls.is_wildcard() {
                return false
            }
        }
        
        if let Some(ref ls) = self.shot {
            if ls.is_wildcard() {
                return false
            }
        }
        true
   }


    /// Retrieve the show if it exists. Otherwise return None
    pub fn show(&self) -> &LevelType {
        &self.show
    }

    /// Retrieve the sequence as a string wrapped in an Option
    pub fn sequence(&self) -> Option<&LevelType> {
        if let Some(ref val) = self.sequence {
            Some(val)
        } else {
            None
        }
    }

    /// Retrieve the sequence as a string wrapped in an Option
    pub fn shot(&self) -> Option<&LevelType> {
        if let Some(ref val) = self.shot {
            Some(val)
        } else {
            None
        }
    }

    /// Convert to a vector of &str
    pub fn to_vec_str<'a>(&'a self) -> Vec<&'a LevelType> {
        let mut vec_strs = Vec::<&'a LevelType>::new();
        //let val = self.show.to_str();
        vec_strs.push(self.show());
        if let Some(ref val) = self.sequence {
            vec_strs.push(val);
            if let Some(ref val) = self.shot {
                vec_strs.push(val);
            }
        }
        vec_strs
    }

}

impl FromStr for LevelSpec {
    type Err = LSE;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = levelspec_parser(s)?;
        match levels.len() {
            3 => {
                let shot = levels.pop();
                let sequence = levels.pop();
                let show = levels.pop().unwrap();
                Ok(LevelSpec{show, sequence, shot})
            },
            2 => {
                let sequence = levels.pop();
                let show = levels.pop().unwrap();
                Ok(LevelSpec{show, sequence, shot:None})
            },
            1 => {
                Ok(LevelSpec{show:levels.pop().unwrap(), sequence:None, shot:None})
            },
            _ => panic!("cannot create levelspec with more than 3 levels")
        }
    }
}

impl fmt::Display for LevelSpec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LevelSpec{show, sequence: Some(seq), shot: Some(sht)} => {
                write!(f, "{}.{}.{}", show, seq, sht)
            },
            LevelSpec{show, sequence: Some(seq), shot: None } => {
                write!(f, "{}.{}", show, seq)
            },
            LevelSpec{show, sequence: None, shot: None } => {
                write!(f, "{}", show)
            },
            _ => panic!("non legal levelspec")
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_replace_relative_shot_with_absolute() {
        let ls = LevelSpec::from_str("..0001").unwrap();
        let new_ls = ls.rel_to_abs(|level|{
            match level {
                LevelName::Show => Some("DEV01".to_string()),
                LevelName::Sequence => Some("RD".to_string()),
                _ => None
            }
        });

        assert_eq!(new_ls, Ok(LevelSpec::from_shot("DEV01", "RD", "0001")));
    }
    #[test]
    fn can_parse_show() {
        let result = LevelSpec::from_str("DEV01");
        let expect = Ok(LevelSpec {show: LevelType::from("DEV01"), sequence: None, shot: None });
        assert_eq!(result, expect);
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_show_with_lowercase_name() {
        let result = LevelSpec::from_str("dev01");
        let expect = Ok(LevelSpec {show: LevelType::from("dev01"), sequence: None, shot: None });
        assert_eq!(result, expect);
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn cannot_parse_show_with_lowercase_name() {
        let result = LevelSpec::from_str("dev01");
        assert_eq!(
            result, 
            Err(LSE::ParseError(
                "Unable to parse levelspec for dev01".to_string())));
    }

    #[test]
    fn can_parse_sequence() {
        let result = LevelSpec::from_str("DEV01.RD");
        let expect = Ok(LevelSpec { 
            show: LevelType::from("DEV01"), 
            sequence: Some(LevelType::from("RD")), 
            shot: None 
        });
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_shot() {
        let result = LevelSpec::from_str("DEV01.RD.0001");
        let expect = Ok(LevelSpec {
            show: LevelType::from("DEV01"), 
            sequence: Some(LevelType::from("RD")), 
            shot: Some(LevelType::from("0001")) });
        assert_eq!(result, expect);
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn can_parse_shot_with_lowercase_show_and_sequence() {
        let result = LevelSpec::from_str("dev01.rd.0001");
        let expect = Ok(LevelSpec {
            show: LevelType::from("dev01"), 
            sequence: Some(LevelType::from("rd")), 
            shot: Some(LevelType::from("0001")) });
        assert_eq!(result, expect);
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn cannot_parse_shot_with_lowercase_shot_and_sequence() {
        let result = LevelSpec::from_str("dev01.rd.0001");
        assert_eq!(
            result, 
            Err(LSE::ParseError(
                "Unable to parse levelspec for dev01.rd.0001".to_string()))
        );
    }


    #[cfg(feature = "case-insensitive")]
    #[test]
    fn will_convert_lowercase_to_uppercase_shot() {
        let result = LevelSpec::from_str("dev01.rd.0001").unwrap().upper();
        let expect = LevelSpec {
            show: LevelType::from("DEV01"), 
            sequence: Some(LevelType::from("RD")), 
            shot: Some(LevelType::from("0001")) };
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_shot_with_wildcard() {
        let result = LevelSpec::from_str("DEV01.RD.%");
        let expect = Ok(LevelSpec {
            show: LevelType::from("DEV01"), 
            sequence: Some(LevelType::from("RD")), 
            shot: Some(LevelType::from("%")) });
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_sequence_with_relative_show() {
        let result = LevelSpec::from_str(".RD");
        let expect = Ok(LevelSpec {
            show: LevelType::from(""), 
            sequence: Some(LevelType::from("RD")), 
            shot: None });
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_shot_with_relative_show() {
        let result = LevelSpec::from_str(".RD.0001");
        let expect = Ok(LevelSpec {
            show: LevelType::from(""), 
            sequence: Some(LevelType::from("RD")), 
            shot: Some(LevelType::from("0001")) });
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_shot_with_relative_show_and_shot() {
        let result = LevelSpec::from_str(".RD.");
        let expect = Ok(LevelSpec {
            show: LevelType::from(""), 
            sequence: Some(LevelType::from("RD")), 
            shot: Some(LevelType::from("")) });
        assert_eq!(result, expect);
    }

    #[test]
    fn can_parse_shot_with_relative_show_and_sequence() {
        let result = LevelSpec::from_str("..9999");
        let expect = Ok(LevelSpec {
            show: LevelType::from(""), 
            sequence: Some(LevelType::from("")), 
            shot: Some(LevelType::from("9999")) });
        assert_eq!(result, expect);
    }

    #[test]
    fn is_concrete_returns_true_for_shot_without_wildcard() {
        let level = LevelSpec::from_str("DEV01.RD.0001").unwrap();
        assert!(level.is_concrete());
    }

    #[test]
    fn is_concrete_returns_true_for_show_with_relative_show() {
        let level = LevelSpec::from_str(".RD.0001").unwrap();
        assert!(level.is_concrete());
    }

    #[test]
    fn is_concrete_returns_true_for_show_with_relative_seq_and_show() {
        let level = LevelSpec::from_str("..0001").unwrap();
        assert!(level.is_concrete());
    }

    #[test] 
    fn is_concrete_returns_true_for_show_with_relative_seq_and_shot() {
        let level = LevelSpec::from_str("DEV01..").unwrap();
        assert!(level.is_concrete());
    }

    #[test]
    fn is_concrete_returns_false_for_show_with_wildcard() {
        let level = LevelSpec::from_str("%.RD.0001").unwrap();
        assert!(!level.is_concrete());
    }

    #[test]
    fn is_concrete_returns_false_for_seq_with_wildcard() {
        let level = LevelSpec::from_str("DEV01.%.0001").unwrap();
        assert!(!level.is_concrete());
    }

    #[test]
    fn is_concrete_returns_false_for_shot_with_wildcard() {
        let level = LevelSpec::from_str("DEV01.RD.%").unwrap();
        assert!(!level.is_concrete());
    }


    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn from_shot_instantiates_uppercase_levelspec_given_lowercase_inputs() {
        let result = LevelSpec::from_shot("dev01", "rd", "0001");
        assert_eq!(
            result, 
            LevelSpec{
                show: LevelType::from("DEV01"),
                sequence: Some(LevelType::from("RD")),
                shot: Some(LevelType::from("0001"))
            }
        );
    }


    #[cfg(feature = "case-insensitive")]
    #[test]
    fn from_shot_instantiates_levelspec_given_lowercase_inputs() {
        let result = LevelSpec::from_shot("dev01", "rd", "0001");
        assert_eq!(
            result, 
            LevelSpec{
                show: LevelType::from("dev01"),
                sequence: Some(LevelType::from("rd")),
                shot: Some(LevelType::from("0001"))
            }
        );
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn from_sequence_instantiates_uppercase_levelspec_given_lowercase_inputs() {
        let result = LevelSpec::from_sequence("dev01", "rd");
        assert_eq!(
            result, 
            LevelSpec{
                show: LevelType::from("DEV01"),
                sequence: Some(LevelType::from("RD")),
                shot: None
            }
        );
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn from_sequence_instantiates_new_levelspec_given_lowercase_inputs() {
        let result = LevelSpec::from_sequence("dev01", "rd");
        assert_eq!(
            result, 
            LevelSpec{
                show: LevelType::from("dev01"),
                sequence: Some(LevelType::from("rd")),
                shot: None
            }
        );
    }

    #[cfg(not(feature = "case-insensitive"))]
    #[test]
    fn from_show_instantiates_uppercase_levelspec_given_lowercase_show() {
        let result = LevelSpec::from_show("dev01");
        assert_eq!(
            result, 
            LevelSpec{
                show: LevelType::from("DEV01"),
                sequence: None,
                shot: None
            }
        );
    }

    #[cfg(feature = "case-insensitive")]
    #[test]
    fn from_show_instantiates_levelspec_given_lowercase_show() {
        let result = LevelSpec::from_show("dev01");
        assert_eq!(
            result, 
            LevelSpec{
                show: LevelType::from("dev01"),
                sequence: None,
                shot: None
            }
        );
    }

    #[test]
    fn can_get_show_from_levelspec() {
        let ls = LevelSpec::from_show("DEV01");
        let show = ls.show();
        assert_eq!(show, &LevelType::Term("DEV01".to_string()));
        assert_eq!(ls.sequence(), None);
        assert_eq!(ls.shot(), None);
    }

    #[test]
    fn can_get_sequence_from_levelspec() {
        let ls = LevelSpec::from_sequence("DEV01","RD");
        assert_eq!(ls.show(), &LevelType::Term("DEV01".to_string()));
        assert_eq!(ls.sequence(), Some(&LevelType::Term("RD".to_string())));
        assert_eq!(ls.shot(), None);
    }

    #[test]
    fn can_get_shot_from_levelspec() {
        let ls = LevelSpec::from_shot("DEV01","RD", "0001");
        assert_eq!(ls.show(), &LevelType::Term("DEV01".to_string()));
        assert_eq!(ls.sequence(), Some(&LevelType::Term("RD".to_string())));
        assert_eq!(ls.shot(), Some(&LevelType::Term("0001".to_string())));
    }

}