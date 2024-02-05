
use crate::{ast::{Value, Variable}, Error};

use super::Environment;

pub struct DebugPrint;

impl Variable for DebugPrint {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        for (i, arg) in args.iter().enumerate() {
            println!("{i}: {arg:?}");
        }
        Ok(Value::Unit)
    }
}
