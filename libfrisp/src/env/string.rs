
use crate::{value::{Variable, Value}, Error};

use super::Environment;

pub struct Split;

impl Variable for Split {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        let string = args[0].as_str()
            .ok_or(Error::VarEvalError(format!("not a string: {:?}", &args[0])))?;

        let split = args[1].as_str()
            .ok_or(Error::VarEvalError(format!("not a string: {:?}", &args[1])))?;

        let parts: Vec<_> = string.split(split).map(|s| Value::string(s)).collect();

        Ok(Value::List(parts))
    }
}

pub struct Lines;

impl Variable for Lines {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 1 {
            return Err(Error::VarEvalArgNumError { expected: 1, actual: args.len() });
        }

        let string = args[0].as_str()
            .ok_or(Error::VarEvalError(format!("not a string: {:?}", &args[0])))?;

        let parts: Vec<_> = string.lines().map(|s| Value::string(s)).collect();

        Ok(Value::List(parts))
    }
}


pub struct ToString;

impl ToString {

    pub fn value_to_string(value: &Value) -> Result<String, Error> {
        match value {
            Value::Unit => Ok("".to_string()),
            Value::String(s) => Ok(s.to_owned()),
            Value::Integer(v) => Ok(v.to_string()),
            Value::Float(v) => Ok(v.to_string()),
            Value::Error(e) => Ok(format!("Error: {e}")),
            v => Err(Error::VarEvalError(format!("cannot make into string: {v:?}"))),
        }
    }

}

impl Variable for ToString {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        
        let mut strings = args.iter().map(ToString::value_to_string);

        if strings.len() == 1 {
            strings.next().unwrap().map(Value::string)
        } else {
            strings.map(|e| e.map(Value::string)).collect()
        }
    }
}

pub struct Concat;

impl Variable for Concat {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        let mut result = String::new();

        for arg in &args {
            let str = arg.as_str()
                .ok_or(Error::VarEvalError(format!("not a string: {:?}", &args[0])))?;
            result.push_str(str);
        }

        Ok(Value::String(result))
    }
}

pub struct Join;

impl Variable for Join {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        let sep = args[0].as_str()
            .ok_or(Error::VarEvalError(format!("not a string: {:?}", &args[0])))?;

        let list = args[1].as_list()
            .ok_or(Error::VarEvalError(format!("not a list: {:?}", &args[1])))?;

        let list: Result<Vec<_>, _> = list.iter().map(|v| v.as_str()
            .ok_or(Error::VarEvalError(format!("not a string: {:?}", &args[0])))).collect();
        let list = list?;

        Ok(Value::String(list.join(sep)))
    }
}

