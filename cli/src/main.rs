use std::io::stdin;

use libfrisp::{env::Environment, value::Value, Error};

fn main() -> Result<(), Error> {

    let mut env = Environment::with_default_content();

    for arg in std::env::args().skip(1) {
        libfrisp::eval_file_with_env(arg, &mut env)?;
    }

    loop {
        let mut input = String::new();

        let count = stdin().read_line(&mut input).map_err(|e| Error::EvalError(e.to_string()))?;

        if count == 0 {
            break;
        }

        match libfrisp::run_with_env(&input, &mut env) {
            Ok(Value::Unit) => {},
            Ok(v) => println!("{v}"),
            Err(e) => println!("{e}"),
        }
    }

    Ok(())
}
