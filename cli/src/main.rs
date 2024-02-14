use std::{error::Error, fmt::Display, io::stdin, path::PathBuf};

use libfrisp::{env::{Env, Environment}, value::{ConstVal, Value}};

use clap::{Parser, Subcommand};

#[derive(Debug)]
enum CliError {
    FrispError(libfrisp::Error),
    GenericError(Box<dyn Error>),
}

impl CliError {

    fn generic_error(error: impl Error + 'static) -> CliError {
        CliError::GenericError(Box::new(error))
    }

}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CliError {

}

impl From<libfrisp::Error> for CliError {
    fn from(value: libfrisp::Error) -> Self {
        CliError::FrispError(value)
    }
}

/// frisp - a lisp interpreter
#[derive(Debug, Parser)]
struct CliArgs {

    /// list of files to evaluate before main script
    #[arg(short, long)]
    include: Vec<String>,

    /// subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,

    /// arguments that get passed to script as `cli-args`
    #[arg(global = true, trailing_var_arg = true)]
    args: Vec<String>,

}

#[derive(Debug, Subcommand)]
enum Commands {
    /// run the interactive repl
    Repl,

    /// run a script from a file
    Run {
        /// path to the script file to execute
        script_path: PathBuf,
    },

    /// execute an immediate script
    Exec {
        /// script to execute
        script: String,
    },
}

impl Default for Commands {
    fn default() -> Self {
        Commands::Repl
    }
}

fn run_repl(env: &mut Environment) -> Result<(), CliError> {
    loop {
        let mut input = String::new();

        let count = stdin().read_line(&mut input).map_err(CliError::generic_error)?;

        if count == 0 {
            break;
        }

        match libfrisp::run_with_env(&input, env) {
            Ok(Value::Unit) => {},
            Ok(v) => println!("{v}"),
            Err(e) => println!("{e}"),
        }
    }

    Ok(())
}

fn main() -> Result<(), CliError> {

    let args = CliArgs::parse();

    let mut env = Environment::with_default_content();

    for include_glob in &args.include {
        for include in glob::glob(include_glob).map_err(CliError::generic_error)? {
            libfrisp::eval_file_with_env(include.map_err(CliError::generic_error)?, &mut env)?;
        }
    }

    env.insert_var("cli-args", ConstVal::from(args.args.iter().map(Value::string).collect::<Value>()));
    env.insert_var("sys-env", ConstVal::from(std::env::vars().map(|(a, b)| Value::List(vec![Value::string(a), Value::string(b)])).collect::<Value>()));

    match &args.command.unwrap_or_default() {
        Commands::Repl => run_repl(&mut env),
        Commands::Run { script_path } => libfrisp::eval_file_with_env(script_path, &mut env).map(|_| ()).map_err(|e| e.into()),
        Commands::Exec { script } => libfrisp::run_with_env(script, &mut env).map(|_| ()).map_err(|e| e.into()),
    }?;

    Ok(())
}
