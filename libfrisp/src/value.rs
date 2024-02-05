
use std::fmt::Display;

use crate::{ast::AstNode, env::{Env, Environment}, token::Token, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    String(String),
    Integer(isize),
    Float(f64),
    List(Vec<Value>),
    Lambda(Vec<String>, Vec<AstNode>),
    SymbolRef(String),
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
        if let Value::List(list) = self {
            Some(list)
        } else {
            None
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
            Value::Lambda(args, body) => {
                write!(f, "(lambda {args:?} {body:?})")
            },
            Value::SymbolRef(v) => write!(f, "@{v}"),
        }
    }
}

impl TryFrom<Token> for Value {
    type Error = Error;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Integer(v) => Ok(Value::Integer(v)),
            Token::Float(f) => Ok(Value::Float(f)),
            Token::String(s) => Ok(Value::String(s)),
            o => Err(Error::ParserError(format!("cannot make value from {o:?}"))),
        }
    }
}

pub trait Variable {
    fn eval(&self, env: &Environment, args: Vec<Value>) -> Result<Value, Error>;

    fn val(&self) -> Option<Value> {
        None
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
            Value::Lambda(vars, body) => {
                if vars.len() != args.len() {
                    return Err(Error::VarEvalArgNumError { expected: vars.len(), actual: args.len() });
                }
                let mut local_env = env.sub_env();

                #[cfg(feature = "log")]
                println!("created sub_env#{:p} from env#{env:p}", &local_env);

                for (name, value) in vars.iter().zip(args.into_iter()) {
                    #[cfg(feature = "log")]
                    println!("local env setting {name} to {value}");
                    local_env.insert_var(name.clone(), ConstVal(value));
                }

                let mut last_value = None;

                for stmt in body {
                    last_value = Some(stmt.eval(&mut local_env)
                        .map_err(|e| Error::VarEvalError(format!("eval error: {e}")))?)
                }
                
                last_value.ok_or(Error::VarEvalError(format!("no value")))
            }
            Value::SymbolRef(sym) => {
                let var = env.get_var(&sym)
                    .ok_or(Error::VarEvalError(format!("unknown symbol: {sym}")))?;
                var.eval(env, args)
            }
            _ => {
                if !args.is_empty() {
                    Err(Error::VarEvalArgNumError { expected: 0, actual: args.len() })
                } else {
                    Ok(v.clone())
                }
            }
        }
    }

    fn val(&self) -> Option<Value> {
        let ConstVal(v) = self;

        Some(v.clone())
    }
}
