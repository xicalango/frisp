use std::{error::Error, fmt::Display, io::stdin, path::PathBuf};

use libfrisp::{env::Environment, value::Value};

use clap::Parser;

#[derive(Debug)]
enum CliError {
    FrispError(libfrisp::Error),
    GenericError(Box<dyn Error>),
    Quit(isize),
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

#[derive(Debug, Parser)]
struct Args {

    #[arg(short, long)]
    include: Vec<String>,

    #[arg(short, long)]
    cli_script: Option<String>,

    script: Option<PathBuf>,

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

    let args = Args::parse();

    let mut env = Environment::with_default_content();

    for include_glob in &args.include {
        for include in glob::glob(include_glob).map_err(CliError::generic_error)? {
            libfrisp::eval_file_with_env(include.map_err(CliError::generic_error)?, &mut env)?;
        }
    }

    let has_script = args.cli_script.is_some() || args.script.is_some();

    if has_script {
        if let Some(script) = &args.cli_script {
            libfrisp::run_with_env(script, &mut env)?;
        }

        if let Some(script_path) = &args.script {
            libfrisp::eval_file_with_env(script_path, &mut env)?;
        }
    } else {
        run_repl(&mut env)?;
    }

    Ok(())
}
