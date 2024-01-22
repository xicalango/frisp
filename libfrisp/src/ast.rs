use std::{fmt::{Display}, collections::HashMap, rc::Rc};

use crate::{token::{TokenStream, Token}, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Unit,
    String(String),
    Integer(isize),
    Float(f64),
    List(Vec<Value>),
    Lambda(Vec<String>, Box<AstNode>),
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

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    List(Vec<AstNode>),
    Symbol(String),
    Value(Value),
}

impl AstNode {

    pub fn try_to_list(self) -> Result<Vec<AstNode>, AstNode> {
        match self {
            AstNode::List(list) => Ok(list),
            o => Err(o),
        }
    }

    pub fn try_to_symbol(self) -> Result<String, AstNode> {
        match self {
            AstNode::Symbol(value) => Ok(value),
            o => Err(o),
        }
    }
    
    pub fn try_to_value(self) -> Result<Value, AstNode> {
        match self {
            AstNode::Value(value) => Ok(value),
            o => Err(o),
        }
    }

}

impl Default for AstNode {
    fn default() -> Self {
        AstNode::List(Vec::new())
    }
}

impl<I> TryFrom<TokenStream<I>> for AstNode 
where I: Iterator<Item = char> {
    type Error = Error;

    fn try_from(mut value: TokenStream<I>) -> Result<Self, Self::Error> {
        let mut lists = Vec::new();
        let mut current_list: Option<Vec<AstNode>> = None;

        while let Some(t) = value.next() {
            match t {
                Token::ListStart => {
                    if let Some(l) = current_list.take() {
                        lists.push(l);
                    }
                    current_list = Some(Vec::new());
                },
                Token::ListEnd => {
                    let list = current_list.take().ok_or(Error::ParserError("list end without current list".to_string()))?;
                    let parent_list = lists.pop();
                    match parent_list {
                        Some(mut pl) => {
                            pl.push(AstNode::List(list));
                            current_list = Some(pl);
                        },
                        None => return Ok(AstNode::List(list))
                    }
                },
                Token::Integer(i) => {
                    let value = AstNode::Value(Value::Integer(i));
                    if let Some(l) = current_list.as_mut() {
                        l.push(value);
                    } else {
                        return Ok(value);
                    }
                },
                Token::Float(i) => {
                    let value = AstNode::Value(Value::Float(i));
                    if let Some(l) = current_list.as_mut() {
                        l.push(value);
                    } else {
                        return Ok(value);
                    }
                },
                Token::Symbol(s) => {
                    let value = AstNode::Symbol(s);
                    if let Some(l) = current_list.as_mut() {
                        l.push(value);
                    } else {
                        return Ok(value);
                    }
                },
                Token::String(s) => {
                    let value = AstNode::Value(Value::String(s));
                    if let Some(l) = current_list.as_mut() {
                        l.push(value);
                    } else {
                        return Ok(value);
                    }
                },
                Token::Error(e) => {
                    return Err(Error::TokenizerError(e));
                }
            }
        }

        return Err(Error::ParserError("reached end of stream without end of list".to_string()));
    }
}


pub trait Variable {
    fn eval(&self, env: &Environment, args: Vec<Value>) -> Result<Value, Error>;
}

pub struct ConstVal(Value);

impl Variable for ConstVal {
    fn eval(&self, env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        let ConstVal(v) = self;

        if args.len() == 0 {
            return Ok(v.clone());
        }

        if let Value::Lambda(vars, body) = v {
            if vars.len() != args.len() {
                return Err(Error::VarEvalArgNumError { expected: vars.len(), actual: args.len() });
            }
            let mut local_env = env.sub_env();

            for (name, value) in vars.iter().zip(args.into_iter()) {
                local_env.insert_var(name.clone(), ConstVal(value));
            }

            body.eval(&mut local_env)
                .map_err(|e| Error::VarEvalError(format!("eval error: {e}")))
        } else {
            return Err(Error::VarEvalArgNumError { expected: 0, actual: args.len() });
        }
    }
}


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

pub struct Eq;

impl Variable for Eq {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }
        Ok(Value::bool(&args[0] == &args[1]))
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

pub trait Env {
    fn get_var(&self, name: &str) -> Option<&Rc<dyn Variable>>;

    fn insert_var(&mut self, name: impl ToString, var: impl Variable + 'static);
}

#[derive(Clone)]
pub struct Environment<'a> {
    env: HashMap<String, Rc<dyn Variable>>,
    parent_env: Option<&'a Environment<'a>>
}

impl Default for Environment<'_> {
    fn default() -> Self {
        Environment::empty()
    }
}

impl<'a> Environment<'a> {

    pub fn empty() -> Environment<'a> {
        Environment { env: Default::default(), parent_env: None }
    }

    pub fn with_default_content() -> Environment<'a> {
        let mut env: HashMap<String, Rc<dyn Variable>> = HashMap::new();
        env.insert("add".to_owned(), Rc::new(Add));
        env.insert("mul".to_owned(), Rc::new(Mul));
        env.insert("eq".to_owned(), Rc::new(Eq));
        env.insert("begin".to_owned(), Rc::new(Begin));
        env.insert("pi".to_owned(), Rc::new(ConstVal(Value::Float(std::f64::consts::PI))));
        env.insert("debug".to_owned(), Rc::new(DebugPrint));
        env.insert("list".to_owned(), Rc::new(MkList));
        Environment { env, parent_env: None }
    }

    pub fn sub_env(&'a self) -> Environment<'a> {
        Environment { env: Default::default(), parent_env: Some(self) }
    }

}

impl<'a> Env for Environment<'a> {
    fn get_var(&self, name: &str) -> Option<&Rc<dyn Variable>> {
        self.parent_env.and_then(|pe| pe.get_var(name)).or_else(|| self.env.get(name))
    }

    fn insert_var(&mut self, name: impl ToString, var: impl Variable + 'static) {
        self.env.insert(name.to_string(), Rc::new(var));
    }
}

impl AstNode {

    pub fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self {
            AstNode::List(l) => {
                println!("evaluating {:?}", l.get(0));

                match l.get(0) {
                    Some(AstNode::Symbol(s)) if s == "if" => {
                        let test = l.get(1).ok_or(Error::EvalError(format!("missing test")))?;
                        let conseq = l.get(2).ok_or(Error::EvalError(format!("missing conseq")))?;
                        let alt = l.get(3).ok_or(Error::EvalError(format!("missing alt")))?;


                        if test.eval(env)? == Value::Integer(1) {
                            return conseq.eval(env);
                        } else {
                            return alt.eval(env);
                        }
                    },
                    Some(AstNode::Symbol(s)) if s == "define" => {
                        let symbol = l.get(1).ok_or(Error::EvalError(format!("no symbol for define")))?;
                        let val = l.get(2).ok_or(Error::EvalError(format!("no value for define")))?;

                        if let AstNode::Symbol(sym) = symbol {
                            let value = val.eval(env)?;
                            println!("defined {sym} to be {value:?}");
                            env.insert_var(sym, ConstVal(value));
                        }


                        return Ok(Value::Unit);
                    },
                    Some(AstNode::Symbol(s)) if s == "lambda" => {
                        let args = l.get(1).ok_or(Error::EvalError(format!("no args for lambda")))?;
                        let body = l.get(2).ok_or(Error::EvalError(format!("no body for lambda")))?;

                        let args = args.to_owned().try_to_list().map_err(|n| Error::EvalError(format!("not a list: {n:?}")))?;

                        let args: Result<Vec<String>, Error> = args.into_iter()
                            .map(|v| v.try_to_symbol()
                                .map_err(|n| Error::EvalError(format!("not a symbol: {n:?}")))
                            ).collect();

                        let args = args?;

                        return Ok(Value::Lambda(args, Box::new(body.clone())));
                    }
                    Some(AstNode::Symbol(s)) => {
                        let mut args: Vec<Value> = Vec::new();

                        for v in &l[1..] {
                            args.push(v.eval(env)?);
                        }

                        let var = env.get_var(s).ok_or(Error::EvalError(format!("proc not found: {s}")))?;
                        let value = var.eval(&env, args);
                        println!("evaluated {s} to {value:?}");
                        value
                    }
                    _ => {
                        return Err(Error::EvalError(format!("invalid at this point in time: {l:?}")))
                    }
                }
            },
            AstNode::Symbol(s) => {
                println!("Symboling {s:?}");
                let var = env.get_var(&s).ok_or(Error::EvalError(format!("symbol not found: {s:?}")))?;
                var.eval(&env, Vec::new())
            },
            AstNode::Value(v) => {
                println!("Valuing {v:?}");
                Ok(v.clone())
            },
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let string = "(begin (define r 10) (mul 10 pi (mul r r)))";

        let tokens = TokenStream::new(string.chars());

        let ast: Result<AstNode, Error> = tokens.try_into();

        println!("{ast:#?}");

        let ast = ast.unwrap();

        let mut env = Environment::with_default_content();

        let res = ast.eval(&mut env);

        println!("{res:?}");
    }

}
