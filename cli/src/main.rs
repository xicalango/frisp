use std::io::stdin;

use libfrisp::{ast::AstNode, env::Environment, token::TokenStream, value::Value};

fn main() {

    let mut env = Environment::with_default_content();

    for line in stdin().lines() {
        let line = line.unwrap();

        let tokens = TokenStream::new(line.chars());
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
