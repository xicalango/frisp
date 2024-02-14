
use std::{fmt::Debug, rc::Rc};

use crate::{env::{Env, Environment}, token::{Token, TokenStream}, value::{ConstVal, Lambda, Value}, Error};

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

    pub fn parse_raw_symbol(raw_symbol: &str) -> AstNode {
        if let Ok(int_value) = raw_symbol.parse::<isize>() {
            return AstNode::Value(Value::Integer(int_value));
        }

        if let Ok(float_value) = raw_symbol.parse::<f64>() {
            return AstNode::Value(Value::Float(float_value));
        }
        AstNode::Symbol(raw_symbol.to_string())
    }

}

impl Default for AstNode {
    fn default() -> Self {
        AstNode::List(Vec::new())
    }
}

pub struct AstNodeStream<I> {
    token_stream: TokenStream<I>,
}

impl<I> AstNodeStream<I> {
    pub fn new(token_stream: TokenStream<I>) -> AstNodeStream<I> {
        AstNodeStream { token_stream }
    }
}

impl<I> Debug for AstNodeStream<I> 
where I: Debug + Iterator<Item = char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AstNodeStream").field("token_stream", &self.token_stream).finish()
    }
}

impl<I> Iterator for AstNodeStream<I> 
where I: Iterator<Item = char> {
    type Item = Result<AstNode, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lists = Vec::new();
        let mut current_list: Option<Vec<AstNode>> = None;

        while let Some(t) = self.token_stream.next() {
            match t {
                Ok(Token::ListStart) => {
                    if let Some(l) = current_list.take() {
                        lists.push(l);
                    }
                    current_list = Some(Vec::new());
                },
                Ok(Token::ListEnd) => {
                    let list = current_list.take().ok_or(Error::ParserError("list end without current list".to_string()));
                    if let Err(e) = list {
                        return Some(Err(e))
                    }
                    let list = list.unwrap();
                    let parent_list = lists.pop();
                    match parent_list {
                        Some(mut pl) => {
                            pl.push(AstNode::List(list));
                            current_list = Some(pl);
                        },
                        None => return Some(Ok(AstNode::List(list)))
                    }
                },
                Ok(Token::Symbol(s)) => {
                    let value = AstNode::parse_raw_symbol(&s);
                    if let Some(l) = current_list.as_mut() {
                        l.push(value);
                    } else {
                        return Some(Ok(value));
                    }
                },
                Ok(Token::String(s)) => {
                    let value = AstNode::Value(Value::String(s));
                    if let Some(l) = current_list.as_mut() {
                        l.push(value);
                    } else {
                        return Some(Ok(value));
                    }
                },
                Err(e) => {
                    return Some(Err(e));
                }
            }
        }

        if current_list.is_none() {
            return None;
        } else {
            return Some(Err(Error::ParserError("reached end of stream without end of list".to_string())));
        }

    }

}

impl AstNode {

    fn eval_list(env: &mut Environment, symbol: &str, l: &Vec<AstNode>) -> Result<Value, Error> {
        match symbol {
            "if" => {
                let test = l.get(1).ok_or(Error::EvalError(format!("missing test")))?;
                let conseq = l.get(2).ok_or(Error::EvalError(format!("missing conseq")))?;
                let alt = l.get(3).ok_or(Error::EvalError(format!("missing alt")))?;


                if test.eval(env)? == Value::Integer(1) {
                    return conseq.eval(env);
                } else {
                    return alt.eval(env);
                }
            },
            "define" => {
                let symbol = l.get(1).ok_or(Error::EvalError(format!("no symbol for define")))?;
                let val = l.get(2).ok_or(Error::EvalError(format!("no value for define")))?;

                if let AstNode::Symbol(sym) = symbol {
                    let value = val.eval(env)?;
                    #[cfg(feature = "log")]
                    println!("defined {sym} to be {value:?}");
                    env.insert_var(sym, ConstVal::from(value));
                }


                return Ok(Value::Unit);
            },
            "lambda" => {
                let args = l.get(1).ok_or(Error::EvalError(format!("no args for lambda")))?;
                let body: Vec<_> = l[2..].iter().map(|n| n.to_owned()).collect();

                let args = args.to_owned().try_to_list().map_err(|n| Error::EvalError(format!("not a list: {n:?}")))?;

                let args: Result<Vec<String>, Error> = args.into_iter()
                    .map(|v| v.try_to_symbol()
                        .map_err(|n| Error::EvalError(format!("not a symbol: {n:?}")))
                    ).collect();

                let args = args?;

                return Ok(Value::Lambda(Rc::new(Lambda::new(args, body))));
            },
            "progn" => {
                let mut last_value = None;

                for stmt in &l[1..] {
                    last_value = Some(stmt.eval(env)?)
                }
                
                last_value.ok_or(Error::VarEvalError(format!("no value")))
            },

            #[cfg(feature = "eval")]
            "eval" => {
                let script = l.get(1).ok_or(Error::EvalError(format!("no args for eval")))?;
                let script_val = script.to_owned().try_to_value().map_err(|v| Error::EvalError(format!("{v:?} is not a value")))?;
                let script_str = script_val.as_str().ok_or(Error::EvalError(format!("{script_val:?} is not a string")))?;

                let res = crate::run_with_env(script_str, env)?;

                #[cfg(feature = "log")]
                println!("evaluated {script_str:?} to {res:?}");
                Ok(res)
            },

            #[cfg(feature = "include")]
            "include" => {
                let path = l.get(1).ok_or(Error::EvalError(format!("no args for include")))?;
                let path_val = path.to_owned().try_to_value().map_err(|v| Error::EvalError(format!("{v:?} is not a value")))?;
                let path_str = path_val.as_str().ok_or(Error::EvalError(format!("{path_val:?} is not a string")))?;

                crate::eval_file_with_env(path_str, env)
            },

            s => {
                let mut args: Vec<Value> = Vec::new();

                for v in &l[1..] {
                    args.push(v.eval(env)?);
                }

                let var = env.get_var(s).ok_or(Error::EvalError(format!("proc not found: {s}")))?;
                let value = var.eval(&env, args);
                #[cfg(feature = "log")]
                println!("evaluated {s} to {value:?}");
                value
            }

        }
    }

    pub fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self {
            AstNode::List(l) => {
                #[cfg(feature = "log")]
                println!("evaluating {:?}", l.get(0));

                match l.get(0) {
                    Some(AstNode::Symbol(s)) => {
                        Self::eval_list(env, s, l)
                    },
                    None => {
                        Ok(Value::Unit)
                    },
                    _ => {
                        return Err(Error::EvalError(format!("invalid at this point in time: {l:?}")))
                    }
                }
            },
            AstNode::Symbol(s) => {
                let var = env.get_var(&s).ok_or(Error::EvalError(format!("symbol not found: {s:?}")))?;
                let res = Ok(var.val().unwrap_or_else(|| Value::SymbolRef(s.clone())));
                #[cfg(feature = "log")]
                println!("Symboling {s:?} to {res:?}");
                res
            },
            AstNode::Value(v) => {
                #[cfg(feature = "log")]
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

        let ast_nodes = AstNodeStream::new(tokens);

        let nodes: Result<Vec<AstNode>, Error> = ast_nodes.collect();

        println!("{nodes:#?}");

        let nodes = nodes.unwrap();

        let mut env = Environment::with_default_content();

        for node in nodes {
            let res = node.eval(&mut env);
            println!("{res:?}");
        }


    }

}
