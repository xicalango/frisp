use std::fmt::Display;

use ast::AstNode;
use env::Environment;
use token::TokenStream;
use value::Value;

pub mod value;
pub mod token;
pub mod ast;
pub mod env;

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
        match self {
            Error::TokenizerError(e) => write!(f, "Tokenizer error: {e}"),
            Error::ParserError(e) => write!(f, "Parser error: {e}"),
            Error::EvalError(e) => write!(f, "Evaluation error: {e}"),
            Error::VarEvalError(e) => write!(f, "{e}"),
            Error::VarEvalArgNumError { expected, actual } => write!(f, "Invalid number of arguments. Expected {expected} but got {actual}"),
        }
    }
}

impl std::error::Error for Error {

}

pub fn run(script: &str) -> Result<Value, Error> {
    let mut env = Environment::with_default_content();
    run_with_env(script, &mut env)
}

pub fn run_with_env(script: &str, env: &mut Environment) -> Result<Value, Error> {
    let tokens = TokenStream::new(script.chars());

    let ast_node = AstNode::try_from(tokens)?;
    ast_node.eval(env)
}
