
use std::io::stdin;

use crate::{value::{Variable, Value}, Error};

use super::Environment;

pub struct Print;

impl Variable for Print {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        for val in &args {
            print!("{val}");
        }
        println!();
        Ok(Value::Unit)
    }
}

pub struct ReadLine;

impl Variable for ReadLine {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 0 {
            return Err(Error::VarEvalArgNumError { expected: 0, actual: args.len() });
        }
        
        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|e| Error::VarEvalError(format!("error while reading from stdin: {e}")))?;

        Ok(Value::string(input.trim_end()))
    }
}

pub struct ParseInt;

impl Variable for ParseInt {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }

        match &args[0] {
            Value::String(s) => {
                let v: isize = s.parse().map_err(|e| Error::VarEvalError(format!("error parsing {s}: {e}")))?;
                Ok(Value::int(v))
            },
            v@Value::Integer(_) => Ok(v.clone()), // TODO probably possible without cloning...
            e => Err(Error::VarEvalError(format!("cannot evaluate {e:?} to int")))
        }        
    }
}
