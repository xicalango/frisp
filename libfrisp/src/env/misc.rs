
use crate::{value::{Variable, Value}, Error};

use super::{Env, Environment};

pub struct DebugPrint;

impl Variable for DebugPrint {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        for (i, arg) in args.iter().enumerate() {
            println!("{i}: {arg:?}");
        }
        Ok(Value::Unit)
    }
}

pub struct TypeOf;

impl TypeOf {

    pub fn type_str(value: &Value) -> &'static str {
        match value {
            Value::Unit => "unit",
            Value::String(_) => "string",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::List(_) => "list",
            Value::Lambda(_) => "lambda",
            Value::SymbolRef(_) => "symbolref",
            Value::Error(_) => "error",
        }
    }

}

impl Variable for TypeOf {
    fn eval(&self, _env: &Environment, args: Vec<Value>) -> Result<Value, Error> {
        let mut types = args.iter().map(TypeOf::type_str).map(Value::string);
        if types.len() == 1 {
            Ok(types.next().unwrap())
        } else {
            Ok(types.collect())
        }
    }
}

pub struct DumpEnv<const LOCAL_ONLY: bool>;

impl<const LOCAL_ONLY: bool> Variable for DumpEnv<LOCAL_ONLY> {
    fn eval(&self, env: &Environment, _args: Vec<Value>) -> Result<Value, Error> {
        let vars = if LOCAL_ONLY {
            env.local_vars()
        } else {
            env.all_vars()
        };
        Ok(vars.iter().map(Value::string).collect())
    }
}
