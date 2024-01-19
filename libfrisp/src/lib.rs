use std::fmt::Display;

pub mod token;
pub mod ast;

#[derive(Debug)]
pub enum Error {
    TokenizerError(String),
    ParserError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {

}
