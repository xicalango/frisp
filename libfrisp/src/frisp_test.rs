use std::{fs, path::PathBuf, str::FromStr};

use crate::{env::{Env, Environment}, value::{Value, Variable}, Error};

struct Assert;

impl Variable for Assert {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        for arg in &args {
            match arg {
                Value::Integer(1) => {}
                Value::Integer(0) => return Err(Error::VarEvalError(format!("assertion failed"))),
                v => return Err(Error::VarEvalError(format!("assertion failed: {v}"))),
            }
        }

        Ok(Value::Unit)
    }
}
struct AssertEq;

impl Variable for AssertEq {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        if args.len() != 2 {
            return Err(Error::VarEvalArgNumError { expected: 2, actual: args.len() });
        }

        if &args[0] != &args[1] {
            Err(Error::VarEvalError(format!("assertion failed: {:?} != {:?}", &args[0], &args[1])))
        } else {
            Ok(Value::Unit)
        }
    }
}

#[test]
fn run_frisp_tests() {

    let tests_path = PathBuf::from_str("../res/test").unwrap();

    let mut env = Environment::with_default_content();
    env.insert_var("assert", Assert);
    env.insert_var("assert-eq", AssertEq);

    for entry in fs::read_dir(&tests_path).unwrap() {
        let entry = entry.unwrap();

        let file_name_val = entry.file_name();
        let file_name = file_name_val.to_string_lossy();

        if !file_name.ends_with("test.lisp") {
            continue;
        }

        println!("test file {file_name}");

        let mut test_env = env.sub_env();

        crate::eval_file_with_env(entry.path(), &mut test_env).unwrap();

        let tests: Vec<_> = test_env.local_vars().iter().filter(|e| e.starts_with("test-")).map(|n| n.to_string()).collect();

        for test in tests {
            println!("running test {test}");
            if let Err(e) = crate::run_with_env(&format!("({test})"), &mut test_env) {
                panic!("test {file_name}/{test} failed: {e}")
            }
        }


    }

}

