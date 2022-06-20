use std::env;
use std::process::ExitCode;

mod autocompleter;
use autocompleter::Autocompleter;

const MAX_ARG_NUM: usize = 2;
const USAGE: &str = "USAGE: {} path/to/dictionary/file (optional)";

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() > MAX_ARG_NUM {
            return Err("number of arguments passed in was incorrect.");
        }
        Ok(Config {
            filename: args[1].clone(),
        })
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    let conf = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing command line arguments: {e}");
            eprintln!("USAGE: {} path/to/dictionary/file (optional)", args[0]);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
