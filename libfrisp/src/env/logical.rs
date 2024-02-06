
use crate::{value::{Variable, Value}, Error};

use super::Environment;

pub struct Not;

impl Variable for Not {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }
        match &args[0] {
            Value::Integer(v) => Ok(Value::bool(*v == 0)),
            e => Err(Error::VarEvalError(format!("cannot not {e:?}")))
        }
    }
}

pub struct And;

impl Variable for And {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::bool(*v1 == 1 && *v2 == 1)),
            e => Err(Error::VarEvalError(format!("cannot and {e:?}")))
        }
    }
}

pub struct Or;

impl Variable for Or {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::bool(*v1 == 1 || *v2 == 1)),
            e => Err(Error::VarEvalError(format!("cannot or {e:?}")))
        }
    }
}


