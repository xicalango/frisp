use std::io::stdin;

use libfrisp::{token::TokenStream, ast::{AstNode, Environment}};




fn main() {

    let mut env = Environment::default();

    for line in stdin().lines() {
        let line = line.unwrap();

        let tokens = TokenStream::new(line.chars());
        match AstNode::try_from(tokens) {
            Ok(node) => {
                match node.eval(&mut env) {
                    Ok(v) => println!("{v}"),
                    Err(e) => println!("error evaluating: {e:?}"),
                }
            },
            Err(e) => println!("Error: {e:?}"),
        }
    }
}
