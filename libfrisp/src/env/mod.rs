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
        let mut env: HashMap<String, Rc<dyn Variable>> = HashMap::new();
        env.insert("add".to_owned(), Rc::new(arithmetic::Add));
        env.insert("sub".to_owned(), Rc::new(arithmetic::Sub));
        env.insert("mul".to_owned(), Rc::new(arithmetic::Mul));
        env.insert("div".to_owned(), Rc::new(arithmetic::Div));
        env.insert("mod".to_owned(), Rc::new(arithmetic::Mod));
        env.insert("eq".to_owned(), Rc::new(arithmetic::Eq));
        env.insert("lt".to_owned(), Rc::new(arithmetic::Lt));

        env.insert("pi".to_owned(), Rc::new(ConstVal::from(Value::Float(std::f64::consts::PI))));
        
        env.insert("begin".to_owned(), Rc::new(list::Begin));
        env.insert("list".to_owned(), Rc::new(list::MkList));
        env.insert("car".to_owned(), Rc::new(list::Car));
        env.insert("cdr".to_owned(), Rc::new(list::Cdr));
        env.insert("cons".to_owned(), Rc::new(list::Cons));

        env.insert("readLine".to_owned(), Rc::new(io::ReadLine));
        env.insert("print".to_owned(), Rc::new(io::Print));
        env.insert("parseInt".to_owned(), Rc::new(io::ParseInt));

        env.insert("debug".to_owned(), Rc::new(misc::DebugPrint));
        Environment { env, parent_env: None }
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

