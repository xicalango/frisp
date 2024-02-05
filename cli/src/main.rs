use std::io::stdin;

use libfrisp::{env::Environment, value::Value};

fn main() {

    let mut env = Environment::with_default_content();

    for arg in std::env::args().skip(1) {
        let script = format!("(include {arg:?})");
        libfrisp::run_with_env(&script, &mut env).unwrap();
    }

    loop {
        let mut input = String::new();

        let count = stdin().read_line(&mut input).unwrap();

        if count == 0 {
            break;
        }

        match libfrisp::run_with_env(&input, &mut env) {
            Ok(Value::Unit) => {},
            Ok(v) => println!("{v}"),
            Err(e) => println!("{e}"),
        }
    }

}
