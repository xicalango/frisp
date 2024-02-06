use std::{collections::HashMap, rc::Rc};

use crate::value::{ConstVal, Value, Variable};

pub mod arithmetic;
pub mod list;
pub mod misc;
pub mod io;

pub trait Env {
    fn get_var(&self, name: &str) -> Option<&Rc<dyn Variable>>;

    fn insert_var(&mut self, name: impl ToString, var: impl Variable + 'static);
}

#[derive(Clone)]
pub struct Environment<'a> {
    env: HashMap<String, Rc<dyn Variable>>,
    parent_env: Option<&'a Environment<'a>>
}

impl Default for Environment<'_> {
    fn default() -> Self {
        Environment::empty()
    }
}

impl<'a> Environment<'a> {

    pub fn empty() -> Environment<'a> {
        Environment { env: Default::default(), parent_env: None }
    }

    pub fn with_default_content() -> Environment<'a> {
        let mut env = Self::default();

        env.insert_var("+", arithmetic::Add);
        env.insert_var("-", arithmetic::Sub);
        env.insert_var("*", arithmetic::Mul);
        env.insert_var("/", arithmetic::Div);
        env.insert_var("mod", arithmetic::Mod);
        env.insert_var("==", arithmetic::Eq);
        env.insert_var("<", arithmetic::Lt);

        env.insert_var("pi", ConstVal::from(Value::Float(std::f64::consts::PI)));
        
        env.insert_var("begin", list::Begin);
        env.insert_var("list", list::MkList);
        env.insert_var("car", list::Car);
        env.insert_var("cdr", list::Cdr);
        env.insert_var("cons", list::Cons);
        env.insert_var("length", list::Length);
        env.insert_var("endp", list::Endp);

        env.insert_var("readLine", io::ReadLine);
        env.insert_var("print", io::Print);
        env.insert_var("parseInt", io::ParseInt);

        env.insert_var("debug", misc::DebugPrint);
        
        env
    }

    pub fn sub_env(&'a self) -> Environment<'a> {
        Environment { env: Default::default(), parent_env: Some(self) }
    }

}

impl<'a> Env for Environment<'a> {
    fn get_var(&self, name: &str) -> Option<&Rc<dyn Variable>> {
        #[cfg(feature = "log")]
        println!("looking up {name} in env#{self:p}");
        self.env.get(name).or_else(|| self.parent_env.and_then(|pe| {
            pe.get_var(name)
        }))
    }

    fn insert_var(&mut self, name: impl ToString, var: impl Variable + 'static) {
        self.env.insert(name.to_string(), Rc::new(var));
    }
}

