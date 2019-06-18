use std::fmt;

/// Enum which models types of entries available in 
/// the LevelSpec
#[derive(Debug, PartialEq, Eq)]
pub enum LevelType {
    Term(String),
    Wildcard,
    Relative
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

    pub fn is_relative(&self) -> bool {
        if self == &LevelType::Relative {
            true
        } else {
            false
        }
    }

    pub fn is_term(&self) -> bool {
        if let &LevelType::Term(_) = self {
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
            LevelType::Relative => "",
        }
    }
}

impl From<&str> for LevelType {
    fn from(input: &str) -> Self {
        match input {
            "%" => LevelType::Wildcard,
            "" => LevelType::Relative,
            _ =>  LevelType::Term(input.to_owned())
        }
    }
}

impl fmt::Display for LevelType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match &self {
           &LevelType::Term(d) => write!(f, "{}", d),
           &LevelType::Wildcard => write!(f, "%"),
           &LevelType::Relative => write!(f, ""),
       }
    }
}
