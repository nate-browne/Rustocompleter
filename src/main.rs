use std::env;
use std::io::{stdin, stdout, Write};
use std::process::{exit, ExitCode};

mod autocompleter;
use autocompleter::Autocompleter;

// Maximum number of command line arguments expected
const MAX_ARG_NUM: usize = 2;

// Filename index
const FILE_IDX: usize = 1;

// Prompt string used in the main program loop
const PROMPT: &str = "Enter a command ((p)redict completions, (a)dd word, (q)uit): ";

/// Small struct only used for parsing command line arguments.
struct Config {
    filename: String,
}

impl Config {
    /// Constructs a new Config object.
    ///
    /// Provides arg parsing and returns a Result of either the constructed object
    /// or an error string.
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
                filename: args[FILE_IDX].clone(),
            })
        }
    }
}

/// Function used to grab user input from the command line.
/// Prints out the given prompt first before grabbing.
fn grab_input(prompt: &str) -> String {
    // Print the prompt out first.
    print!("{prompt}");
    if let Err(e) = stdout().flush() {
        eprintln!("Error flushing output stream: {e}");
        exit(1);
    }

    // Grab the user's input string, and strip trailing characters.
    let mut option = String::new();
    if let Err(e) = stdin().read_line(&mut option) {
        eprintln!("Error occurred reading input from stdin: {e}");
        exit(1);
    };
    String::from(option.trim())
}

/// Main program driver. Parses command line args, instantiates an autocompleter,
/// and starts the main program loop.
fn main() -> ExitCode {
    println!();

    // Grab the command line arguments to start.
    let args: Vec<String> = env::args().collect();
    let conf = match Config::new(&args) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing command line arguments: {e}");
            eprintln!("USAGE: {} path/to/dictionary/file (optional)", args[0]);
            return ExitCode::FAILURE;
        }
    };

    // Instantiate an autocompleter.
    // If no arg is provided, start a blank one. Else, parse the file and load it in.
    let mut ac = if conf.filename.as_str() == "" {
        Autocompleter::new()
    } else {
        match Autocompleter::from_file(&conf.filename) {
            Ok(acc) => acc,
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }
    };

    loop {
        let input = grab_input(PROMPT);

        match input.as_str() {
            "a" => {
                // Add a word to the dictionary
                let st = grab_input("Enter string to add to completer: ");
                ac.add_word(st);
                println!("String added!");
            }
            "p" => {
                // Do a prediction search.
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
