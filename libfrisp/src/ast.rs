
use std::fs::read_to_string;

use crate::{env::{Env, Environment}, token::{Token, TokenStream}, value::{ConstVal, Value}, Error};

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
                        // TODO this will end at the first occurrence of a complete expression
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

impl AstNode {

    pub fn eval(&self, env: &mut Environment) -> Result<Value, Error> {
        match self {
            AstNode::List(l) => {
                #[cfg(feature = "log")]
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
                            #[cfg(feature = "log")]
                            println!("defined {sym} to be {value:?}");
                            env.insert_var(sym, ConstVal::from(value));
                        }


                        return Ok(Value::Unit);
                    },
                    Some(AstNode::Symbol(s)) if s == "lambda" => {
                        let args = l.get(1).ok_or(Error::EvalError(format!("no args for lambda")))?;
                        let body: Vec<_> = l[2..].iter().map(|n| n.to_owned()).collect();

                        let args = args.to_owned().try_to_list().map_err(|n| Error::EvalError(format!("not a list: {n:?}")))?;

                        let args: Result<Vec<String>, Error> = args.into_iter()
                            .map(|v| v.try_to_symbol()
                                .map_err(|n| Error::EvalError(format!("not a symbol: {n:?}")))
                            ).collect();

                        let args = args?;

                        return Ok(Value::Lambda(args, body));
                    },
                    Some(AstNode::Symbol(s)) if s == "eval" => {
                        let script = l.get(1).ok_or(Error::EvalError(format!("no args for eval")))?;
                        let script_val = script.to_owned().try_to_value().map_err(|v| Error::EvalError(format!("{v:?} is not a value")))?;
                        let script_str = script_val.as_str().ok_or(Error::EvalError(format!("{script_val:?} is not a string")))?;

                        let tokens = TokenStream::new(script_str.chars());
                        let ast_node = AstNode::try_from(tokens)?;
                        let res = ast_node.eval(env)?;

                        #[cfg(feature = "log")]
                        println!("evaluated {script_str:?} to {res:?}");
                        Ok(res)
                    },
                    #[cfg(feature = "include")]
                    Some(AstNode::Symbol(s)) if s == "include" => {
                        let path = l.get(1).ok_or(Error::EvalError(format!("no args for include")))?;
                        let path_val = path.to_owned().try_to_value().map_err(|v| Error::EvalError(format!("{v:?} is not a value")))?;
                        let path_str = path_val.as_str().ok_or(Error::EvalError(format!("{path_val:?} is not a string")))?;

                        let file_contents = read_to_string(path_str).map_err(|e| Error::EvalError(format!("Error when reading from file {path_str:?}: {e}")))?;

                        let tokens = TokenStream::new(file_contents.chars());
                        let ast_node = AstNode::try_from(tokens)?;
                        ast_node.eval(env)?;
                        Ok(Value::Unit)
                    },
                    Some(AstNode::Symbol(s)) => {
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

        let ast: Result<AstNode, Error> = tokens.try_into();

        println!("{ast:#?}");

        let ast = ast.unwrap();

        let mut env = Environment::with_default_content();

        let res = ast.eval(&mut env);

        println!("{res:?}");
    }

}
