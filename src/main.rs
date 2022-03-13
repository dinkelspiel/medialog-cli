use json;
use json::JsonValue;
use json::object;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use colored::*;

fn error(message: &str) {
    eprintln!("{}: {}!", "error".bright_red(), message);
}

fn error_flat(message: &str) {
    eprint!("{}: {}", "error".bright_red(), message);
}

fn warning(message: &str, prefix: &str) {
    eprintln!("{}: {}.", prefix.yellow(), message); 
}

fn warning_flat(message: &str, prefix: &str) {
    eprint!("{}: {}", prefix.yellow(), message); 
}

fn help(message: &str) {
    eprintln!("{}: {}.", "help".bright_cyan(), message);
}

fn stop() {
    process::exit(0);
}

const USAGE_PREFIX: &str = "usage";
const OPTIONS_PREFIX: &str = "options";

const CATEGORIES: [&str; 3] = ["anime", "manga", "podcast"];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error("Must supply command in arguments");
        help("Consider running the 'help' command");
        stop();
    }

    if args[1].to_ascii_lowercase() == "add" {
        if args.len() == 4 {
            let media_name: &str = &args[2];
            let media_category: &str = &args[3];
            if CATEGORIES.contains(&args[3].as_str()) {
                let media_object: JsonValue = object! {
                    name: media_name,
                    category: media_category,
                    seasons: {}
                };

                File::create("./data.json").unwrap();
                let mut file: File = File::open("./data.json").unwrap();
                file.write(json::stringify(media_object).as_bytes()).expect_err("Couldn't write to file.");
            } else {
                error("Invalid category in command 'add'");
                warning_flat("add <name> ", USAGE_PREFIX);
                println!("{}", "<category>".red().bold());
                warning_flat("", OPTIONS_PREFIX);
                for (i, item) in CATEGORIES.iter().enumerate() {
                    if i < CATEGORIES.len() - 1 {
                        eprint!("{}, ", item);
                        continue;
                    }
                    eprint!("{}", item);
                }
            } 

        } else {
            error("Insufficient arguments for command 'add'");
            warning("add <name> <category>", USAGE_PREFIX);
            stop();
        }   
    } else {
        error_flat("Could not recognize command '");
        print!("{}'!", args[1]);
        stop();
    }
}