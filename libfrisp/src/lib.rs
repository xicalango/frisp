use std::{fmt::Display, fs::read_to_string, path::Path};

use ast::AstNodeStream;
use env::Environment;
use token::TokenStream;
use value::Value;

pub mod value;
pub mod token;
pub mod ast;
pub mod env;

#[cfg(test)]
mod frisp_test;

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
    run_with_env(script, &mut Environment::with_default_content())
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

pub fn eval_file<P: AsRef<Path>>(path: P) -> Result<Value, Error> {
    eval_file_with_env(path, &mut Environment::with_default_content())
}

pub fn eval_file_with_env<P: AsRef<Path>>(path: P, env: &mut Environment) -> Result<Value, Error> {
    let file_contents = read_to_string(path.as_ref()).map_err(|e| Error::EvalError(format!("Error when reading from file {:?}: {e}", path.as_ref())))?;
    run_with_env(&file_contents, env)
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
        let gcd_code = include_str!("../../res/gcd.lisp");

        assert_eq!(Value::Unit, run_with_env(gcd_code, &mut env).unwrap());
        assert_eq!(Value::int(3isize), run_with_env(&format!("(gcd 1098 1173)"), &mut env).unwrap());
    }

    #[test]
    fn test_include() {
        let mut env = Environment::with_default_content();

        assert_eq!(Value::Unit, run_with_env(&format!("(include \"../res/include_test.lisp\")"), &mut env).unwrap());
        assert_eq!(Value::Integer(2), run_with_env("a", &mut env).unwrap());
        assert_eq!(Value::Integer(3), run_with_env("b", &mut env).unwrap());
        assert_eq!(Value::Integer(5), run_with_env("c", &mut env).unwrap());
    }

}