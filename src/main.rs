use std::env;
use std::io::{stdin, stdout, Write};
use std::process::{exit, ExitCode};

mod autocompleter;
use autocompleter::Autocompleter;

const MAX_ARG_NUM: usize = 2;
const PROMPT: &str = "Enter a command ((p)redict completions, (a)dd word, (q)uit): ";

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() > MAX_ARG_NUM {
            return Err("number of arguments passed in was incorrect.");
        }

        if args.len() == 1 {
            Ok(Config {
                filename: String::new(),
            })
        } else {
            Ok(Config {
                filename: args[1].clone(),
            })
        }
    }
}

fn grab_input(prompt: &str) -> String {
    print!("{prompt}");
    if let Err(e) = stdout().flush() {
        eprintln!("Error flushing output stream: {e}");
        exit(1);
    }

    let mut option = String::new();
    if let Err(e) = stdin().read_line(&mut option) {
        eprintln!("Error occurred reading input from stdin: {e}");
        exit(1);
    };
    String::from(option.trim())
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

    let mut ac = if conf.filename.as_str() == "" {
        Autocompleter::new()
    } else {
        match Autocompleter::from_dict(conf.filename) {
            Ok(acc) => acc,
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    };

    loop {
        let input = grab_input(PROMPT);

        match input.as_str() {
            "a" => {
                let st = grab_input("Enter string to add to completer: ");
                ac.add_word(st);
                println!("String added!");
            }
            "p" => {
                let prefix = grab_input("Enter prefix to get completions for: ");
                let result = ac.predict_completions(&prefix);
                println!(
                    "Completions for {prefix} (most to least popular): {:?}",
                    result
                );
            }
            "q" => break,
            _ => println!("Command {input} is not valid"),
        }
    }

    ExitCode::SUCCESS
}
