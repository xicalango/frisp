
use std::{fmt::Display, rc::Rc};

use crate::{ast::AstNode, env::{Env, Environment}, Error};


#[derive(Debug, PartialEq, Clone)]
pub struct Lambda {
    vars: Vec<String>,
    body: Vec<AstNode>,
}

impl Lambda {

    pub fn new(args: Vec<String>, body: Vec<AstNode>) -> Lambda {
        Lambda { vars: args, body }
    }
    
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    String(String),
    Integer(isize),
    Float(f64),
    List(Vec<Value>),
    Lambda(Rc<Lambda>),
    SymbolRef(String),
    Error(String),
}

impl Default for Value {
    fn default() -> Self {
        Value::Unit
    }
}

impl Value {

    pub fn bool(v: bool) -> Value {
        Value::int(v)
    }

    pub fn int<T: Into<isize>>(v: T) -> Value {
        Value::Integer(v.into())
    }

    pub fn float<T: Into<f64>>(v: T) -> Value {
        Value::Float(v.into())
    }

    pub fn string<T: ToString>(v: T) -> Value {
        Value::String(v.to_string())
    }

    pub fn as_str(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s.as_str())
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&Vec<Value>> {
        if let Value::List(list) = self {
            Some(list)
        } else {
            None
        }
    }

    pub fn to_list(self) -> Option<Vec<Value>> {
        match self {
            Value::List(list) => Some(list),
            Value::Unit => Some(Vec::new()),
            _ => None,
        }
    }

    pub fn unwrap_err(self) -> Result<Value, Error> {
        if let Value::Error(e) = self {
            Err(Error::VarEvalError(e))
        } else {
            Ok(self)
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            Value::Integer(i) => Some(i != 0),
            _ => None,
        }
    }

}


impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Unit => write!(f, ""),
            Value::String(v) => write!(f, "{v}"),
            Value::Integer(v) => write!(f, "{v}"),
            Value::Float(v) => write!(f, "{v}"),
            Value::List(v) => {
                let s: Vec<_> = v.iter().map(|vv| vv.to_string()).collect();
                write!(f, "({})", s.join(","))
            },
            Value::Lambda(lambda) => {
                write!(f, "(lambda {:?} {:?})", &lambda.vars, &lambda.body)
            },
            Value::SymbolRef(v) => write!(f, "@{v}"),
            Value::Error(e) => write!(f, "Value Error: {e}"),
        }
    }
}

pub trait Variable {
    fn eval(&self, env: &Environment, args: Vec<Value>) -> Result<Value, Error>;

    fn val(&self) -> Option<Value> {
        None
    }
}

impl Variable for Lambda {
    fn eval(&self, env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        let vars = &self.vars;
        if vars.len() != args.len() {
            return Err(Error::VarEvalArgNumError { expected: vars.len(), actual: args.len() });
        }
        let mut local_env = env.local_env();

        #[cfg(feature = "log")]
        println!("created local_env#{:p} from env#{env:p}", &local_env);

        for (name, value) in vars.iter().zip(args.into_iter()) {
            #[cfg(feature = "log")]
            println!("local env setting {name} to {value}");
            local_env.insert_var(name.clone(), ConstVal(value));
        }

        let mut last_value = None;

        for stmt in &self.body {
            last_value = Some(stmt.eval(&mut local_env)
                .map_err(|e| Error::VarEvalError(format!("eval error: {e}")))?)
        }
        
        last_value.ok_or(Error::VarEvalError(format!("no value")))
    }
}

pub struct ConstVal(Value);

impl ConstVal {

    pub fn wrap(value: Value) -> ConstVal {
        ConstVal(value)
    }
    
}

impl From<Value> for ConstVal {
    fn from(value: Value) -> Self {
        ConstVal::wrap(value)
    }
}

impl Variable for ConstVal {
    fn eval(&self, env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        let ConstVal(v) = self;

        match v {
            Value::Lambda(lambda) => {
                lambda.eval(env, args)
            }
            Value::SymbolRef(sym) => {
                #[cfg(feature = "log")]
                println!("getting symbol ref {sym}");
                let var = env.get_var(&sym)
                    .ok_or(Error::VarEvalError(format!("unknown symbol: {sym}")))?;
                var.eval(env, args)
            }
            _ => {
                if !args.is_empty() {
                    Err(Error::VarEvalArgNumError { expected: 0, actual: args.len() })
                } else {
                    #[cfg(feature = "log")]
                    println!("getting value {v:?}");
                    Ok(v.clone())
                }
            }
        }
    }

    fn val(&self) -> Option<Value> {
        let ConstVal(v) = self;

        #[cfg(feature = "log")]
        println!("val access to {v:?}");
        Some(v.clone())
    }
}

mod arithmetic_impl {

use std::ops::{Add, Sub, Mul, Div};

use super::Value;

// use proc macro

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 + v2),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 + v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 + v2 as f64),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 + v2),
            (v1, v2) => Value::Error(format!("Cannot add {v1:?} and {v2:?}")),
        }
    }

}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 - v2),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 - v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 - v2 as f64),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 - v2),
            (v1, v2) => Value::Error(format!("cannot sub {v1:?} and {v2:?}")),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 * v2),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 * v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 * v2 as f64),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 * v2),
            (v1, v2) => Value::Error(format!("cannot mul {v1:?} and {v2:?}")),
        }
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Integer(v1), Value::Integer(v2)) => Value::Integer(v1 / v2),
            (Value::Integer(v1), Value::Float(v2)) => Value::Float(v1 as f64 / v2),
            (Value::Float(v1), Value::Integer(v2)) => Value::Float(v1 / v2 as f64),
            (Value::Float(v1), Value::Float(v2)) => Value::Float(v1 / v2),
            (v1, v2) => Value::Error(format!("cannot div {v1:?} and {v2:?}")),
        }
    }
}

}