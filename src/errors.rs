use failure::Fail;

#[derive(Debug, Fail, PartialEq, Eq, Clone)]
pub enum LevelSpecterError {
    #[fail(display = "Placeholder error")]
    Placeholder,
    
    #[fail(display = "Parse Error {}", _0)]
    ParseError(String),
    
    #[fail(display = "RelToAbs Error: {}", _0)]
    RelToAbsError(String),

}