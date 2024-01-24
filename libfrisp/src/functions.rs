use crate::{ast::{Variable, Environment, Value}, Error};

pub struct Mul;

impl Variable for Mul {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::Integer(v1 * v2)),
            (Value::Integer(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 * v2)),
            (Value::Float(v1), Value::Integer(v2)) => Ok(Value::Float(v1 * *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 * v2)),
            (v1, v2) => Err(Error::VarEvalError(format!("cannot add {v1:?} and {v2:?}"))),
        }
    }
}

pub struct Add;

impl Variable for Add {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::Integer(v1 + v2)),
            (Value::Integer(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 + v2)),
            (Value::Float(v1), Value::Integer(v2)) => Ok(Value::Float(v1 + *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 + v2)),
            (v1, v2) => Err(Error::VarEvalError(format!("cannot add {v1:?} and {v2:?}"))),
        }
    }
}

pub struct Sub;

impl Variable for Sub {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        match (&args[0], &args[1]) {
            (Value::Integer(v1), Value::Integer(v2)) => Ok(Value::Integer(v1 - v2)),
            (Value::Integer(v1), Value::Float(v2)) => Ok(Value::Float(*v1 as f64 - v2)),
            (Value::Float(v1), Value::Integer(v2)) => Ok(Value::Float(v1 - *v2 as f64)),
            (Value::Float(v1), Value::Float(v2)) => Ok(Value::Float(v1 - v2)),
            (v1, v2) => Err(Error::VarEvalError(format!("cannot add {v1:?} and {v2:?}"))),
        }
    }
}


pub struct Eq;

impl Variable for Eq {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
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

pub struct DebugPrint;

impl Variable for DebugPrint {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        for (i, arg) in args.iter().enumerate() {
            println!("{i}: {arg:?}");
        }
        Ok(Value::Unit)
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
