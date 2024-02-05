use std::io::stdin;

use libfrisp::{ast::AstNode, env::Environment, token::TokenStream, value::Value};

fn main() {

    let mut env = Environment::with_default_content();

    for arg in std::env::args().skip(1) {
        let script = format!("(include {arg:?})");
        let tokens = TokenStream::new(script.chars());
        AstNode::try_from(tokens).unwrap().eval(&mut env).unwrap();
    }

    loop {
        let mut input = String::new();

        let count = stdin().read_line(&mut input).unwrap();

        if count == 0 {
            break;
        }

        let tokens = TokenStream::new(input.chars());
        match AstNode::try_from(tokens) {
            Ok(node) => {
                match node.eval(&mut env) {
                    Ok(Value::Unit) => {},
                    Ok(v) => println!("{v}"),
                    Err(e) => println!("error evaluating: {e:?}"),
                }
            },
            Err(e) => println!("Error: {e:?}"),
        }


    }

}
