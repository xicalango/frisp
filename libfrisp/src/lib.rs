use std::fmt::Display;

use ast::AstNodeStream;
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
    let ast_nodes = AstNodeStream::new(tokens);

    let mut last_value = None;

    for ast_node in ast_nodes {
        let ast_node = ast_node?;
        last_value = Some(ast_node.eval(env)?);
    }
    
    Ok(last_value.unwrap_or_default())
}

#[cfg(test)]
mod test {
    use crate::{env::Environment, run_with_env, value::Value};


    #[test]
    fn test_fib() {
        let mut env = Environment::with_default_content();
        let fib_code = include_str!("../../res/fib.lisp");

        assert_eq!(Value::Unit, run_with_env(fib_code, &mut env).unwrap());
        assert_eq!(Value::int(55isize), run_with_env(&format!("(fib 10)"), &mut env).unwrap());
    }

    #[test]
    fn test_gcd() {
        let mut env = Environment::with_default_content();
        let fib_code = include_str!("../../res/gcd.lisp");

        assert_eq!(Value::Unit, run_with_env(fib_code, &mut env).unwrap());
        assert_eq!(Value::int(3isize), run_with_env(&format!("(gcd 1098 1173)"), &mut env).unwrap());
    }

}