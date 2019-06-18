use std::fmt;

/// Enum which models types of entries available in 
/// the LevelSpec
#[derive(Debug, PartialEq, Eq)]
pub enum LevelType {
    Term(String),
    Wildcard,
}

impl LevelType {
    /// Wildcard leveltype implies that the LevelSpec
    /// is not concrete.
    pub fn is_wildcard(&self) -> bool {
        if self == &LevelType::Wildcard {
            true
        } else {
            false
        }
    }

    /// Convert to a str
    pub fn to_str(&self) -> &str {
        match *self {
            LevelType::Term(ref val) => val,
            LevelType::Wildcard => "%",
        }
    }
}

impl From<&str> for LevelType {
    fn from(input: &str) -> Self {
        if input == "%" { 
            LevelType::Wildcard 
        } else {
            LevelType::Term(input.to_owned())
        }  
    }
}

impl fmt::Display for LevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &self {
           &LevelType::Term(d) => write!(f, "{}", d),
           &LevelType::Wildcard => write!(f, "%"),
       }
    }
}
