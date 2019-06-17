use crate::{LevelSpecterError, levelspec_parser};
use  std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LevelType {
    Dir(String),
    Wildcard,
}
impl LevelType {
    pub fn is_wildcard(&self) -> bool {
        if self == &LevelType::Wildcard {
            true
        } else {
            false
        }
    }
}

impl From<&str> for LevelType {
    fn from(input: &str) -> Self {
        if input == "%" { 
            LevelType::Wildcard 
        } else {
            LevelType::Dir(input.to_owned())
        }  
    }
}

impl fmt::Display for LevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &self {
           &LevelType::Dir(d) => write!(f, "{}", d),
           &LevelType::Wildcard => write!(f, "%"),
       }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct LevelSpec {
    show: LevelType,
    sequence: Option<LevelType>,
    shot: Option<LevelType>
}

impl LevelSpec {
    /// new up a show
    pub fn new_show(input: &str ) -> Self  {
        Self {
            show: LevelType::from(input), 
            sequence: None, 
            shot: None
        }
    }
    /// new up a sequence
    pub fn new_sequence(show: &str, sequence: &str ) -> Self  {
        Self {
            show: LevelType::from(show), 
            sequence: Some(LevelType::from(sequence)), 
            shot: None
        }
    }

    pub fn new_shot(show: &str, sequence: &str, shot: &str) -> Self  {
        Self {
            show: LevelType::from(show), 
            sequence: Some(LevelType::from(sequence)), 
            shot: Some(LevelType::from(shot))
        }
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
}

impl FromStr for LevelSpec {
    type Err = LevelSpecterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = levelspec_parser(s)?;
        match levels.len() {
            3 => Ok(LevelSpec::new_shot(levels[0], levels[1], levels[2])),
            2 => Ok(LevelSpec::new_sequence(levels[0], levels[1])),
            1 => Ok(LevelSpec::new_show(levels[0])),
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
    fn can_parse_show() {
        let result = LevelSpec::from_str("DEV01");
        let expect = Ok(LevelSpec {show: LevelType::from("DEV01"), sequence: None, shot: None });
        assert_eq!(result, expect);
    }

 #[test]
    fn can_parse_seq() {
        let result = LevelSpec::from_str("DEV01.RD");
        let expect = Ok(LevelSpec {show: LevelType::from("DEV01"), sequence: Some(LevelType::from("RD")), shot: None });
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
    fn is_concrete_works() {
        let level = LevelSpec::from_str("DEV01.RD.0001").unwrap();
        assert!(level.is_concrete());
    }

    #[test]
    fn is_concrete_for_show_wildcard_works() {
        let level = LevelSpec::from_str("%.RD.0001").unwrap();
        assert!(!level.is_concrete());
    }

    #[test]
    fn is_concrete_for_seq_wildcard_works() {
        let level = LevelSpec::from_str("DEV01.%.0001").unwrap();
        assert!(!level.is_concrete());
    }

    #[test]
    fn is_concrete_for_shot_wildcard_works() {
        let level = LevelSpec::from_str("DEV01.RD.%").unwrap();
        assert!(!level.is_concrete());
    }
}