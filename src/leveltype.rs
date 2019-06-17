use std::fmt;

/// Enum which models types of entries available in 
/// the LevelSpec
#[derive(Debug, PartialEq, Eq)]
pub enum LevelType {
    Dir(String),
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
