use failure::Fail;

#[derive(Debug, Fail, PartialEq, Eq)]
pub enum LevelSpecterError {
    #[fail(display = "Placeholder error")]
    Placeholder,
    
    #[fail(display = "Parse Error {}", _0)]
    ParseError(String),
    
}