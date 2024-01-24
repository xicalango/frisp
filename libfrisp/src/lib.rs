use std::fmt::Display;

pub mod token;
pub mod ast;
pub mod functions;

#[derive(Debug)]
pub enum Error {
    TokenizerError(String),
    ParserError(String),
    EvalError(String),
    VarEvalError(String),
    VarEvalArgNumError {
        expected: usize,
        actual: usize,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {

}
