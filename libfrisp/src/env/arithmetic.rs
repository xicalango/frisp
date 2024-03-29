use crate::{value::{Variable, Value}, Error};

use super::Environment;

pub struct Add;

impl Variable for Add {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        args.into_iter().reduce(|accu, v| accu + v).unwrap_or_default().unwrap_err()
    }
}

pub struct Sub;

impl Variable for Sub {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        args.into_iter().reduce(|accu, v| accu - v).unwrap_or_default().unwrap_err()
    }
}

pub struct Mul;

impl Variable for Mul {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        args.into_iter().reduce(|accu, v| accu * v).unwrap_or_default().unwrap_err()
    }
}

pub struct Div;

impl Variable for Div {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        args.into_iter().reduce(|accu, v| accu / v).unwrap_or_default().unwrap_err()
    }
}

pub struct Mod;

impl Variable for Mod {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        match (&args[0],&args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::Integer(v1 % v2)),
            (Value::Integer(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 % v2)),
            (Value::Float(v1), Value::Integer(v2)) => Ok(Value::Float(v1 % *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 % v2)),
            (v1, v2) => Err(Error::VarEvalError(format!("cannot mod {v1:?} and {v2:?}"))),
        }
    }
}

pub struct Eq;

impl Variable for Eq {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        #[cfg(feature = "log")]
        println!("is {:?} == {:?}?", &args[0], &args[1]);
        Ok(Value::bool(&args[0] == &args[1]))
    }
}

pub struct Lt;

impl Variable for Lt {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::bool(v1 < v2)),
            e => Err(Error::VarEvalError(format!("cannot lt {e:?}")))
        }
    }
}

pub struct Gt;

impl Variable for Gt {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::bool(v1 > v2)),
            e => Err(Error::VarEvalError(format!("cannot gt {e:?}")))
        }
    }
}
