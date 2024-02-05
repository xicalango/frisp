
use crate::{value::{Variable, Value}, Error};

use super::Environment;

pub struct Begin;

impl Variable for Begin {
    fn eval(&self, _env: &Environment, mut args: Vec<Value>) -> Result<Value, Error> {
        if let Some(v) = args.last_mut() {
            Ok(std::mem::take(v))
        } else {
            Ok(Value::Unit)
        }
    }
}

pub struct MkList;

impl Variable for MkList {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        Ok(Value::List(args))
    }
}

pub struct Car;

impl Variable for Car {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }
        let first_arg = &args[0];
        Ok(first_arg.as_list()
            .ok_or(Error::VarEvalError(format!("{:?} is not a list", first_arg)))?
            .get(0).ok_or(Error::VarEvalError(format!("list does not have an element")))?
            .to_owned()
        )
    }
}

pub struct Cdr;

impl Variable for Cdr {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }
    
        let list = args[0].as_list()
            .ok_or(Error::VarEvalError(format!("{:?} is not a list", args[0])))?;

        let list = list[1..].into_iter().map(|v| v.to_owned()).collect();

        Ok(Value::List(list))
    }
}

pub struct Cons;

impl Variable for Cons {
    fn eval(&self, _env: &Environment, mut args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        let e = std::mem::take(&mut args[0]);
        let v = std::mem::take(&mut args[1]);
        let mut l = v.to_list().ok_or(Error::VarEvalError(format!("cdr on not a list")))?;
        l.insert(0, e); 

        Ok(Value::List(l))
    }
}

pub struct Length;

impl Variable for Length {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }
        
        let len = match &args[0] {
            Value::Unit => 0,
            Value::String(s) => s.len(),
            Value::List(l) => l.len(),
            v => return Err(Error::VarEvalError(format!("{v:?} does not have a length"))),
        };

        Ok(Value::int(len as isize))
    }
}

pub struct Endp;

impl Variable for Endp {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }

        match &args[0] {
            Value::List(list) => Ok(Value::bool(list.is_empty())),
            v => Err(Error::VarEvalError(format!("not a list: {v:?}"))),
        } 
    }
}
