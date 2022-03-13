use std::path::Path;
use std::io::Read;
use json;
use json::JsonValue;
use json::object;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use colored::*;
use dirs::*;
use edit::*;

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

fn help_flat(message: &str) {
    eprint!("{}: {}", "help".bright_cyan(), message);
}

fn stop() {
    process::exit(0);
}

const USAGE_PREFIX: &str = "usage";
const OPTIONS_PREFIX: &str = "options";

const CATEGORIES: [&str; 3] = ["anime", "manga", "podcast"];

fn main() -> std::io::Result<()>  {
    let args: Vec<String> = env::args().collect();
    
    let mut data: JsonValue;

    let mut path: String;

    if cfg!(windows) {
        path = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        path.push_str("\\medialog.json");
    } else if cfg!(unix) {
        path = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        path.push_str("/medialog.json");
    } else {
        path = String::from("medialog.json");
    }

    if Path::new(&path).exists() {
        let mut file = File::open(&path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        data = json::parse(&contents).unwrap();
    } else {
        data = object! {
            series: {},
            movies: {},
            manga: {},
        };
    }

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
                // let media_object: JsonValue = object! {
                //     name: media_name,
                //     category: media_category,
                //     seasons: {}
                // };

                data[media_category][media_name] = object! {}; 
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
    } else if args[1].to_ascii_lowercase() == "editseason" {
        if args.len() == 6 {
            let season_name: &str = &args[2].to_ascii_lowercase();
            let edit_object: &str = &args[3].to_ascii_lowercase();
            let media_name: &str = &args[4];
            let media_category: &str = &args[5];
            if CATEGORIES.contains(&args[5].as_str()) {
                if data[media_category].has_key(media_name) {
                    if !data[media_category][media_name].has_key(season_name) {
                        error_flat("");
                        println!("Season {} doesn't exist in media {}!", season_name, media_name);
                        help_flat("");
                        println!("Consider running 'medialog createseason {} {} {}'!", season_name, media_name, media_category);
                        stop();
                    }

                    if !["studio", "rating", "notes", "json"].contains(&edit_object) {
                        error_flat("");
                        println!("Invalid media property {}!", edit_object);
                        help("Use 'studio', 'rating', 'notes' or 'json'!");
                        stop();
                    }
                    
                    
                    if edit_object != "json" {
                        let result: String = edit::edit(json::stringify(data[media_category][media_name][season_name][edit_object].clone())).unwrap();

                        data[media_category][media_name][season_name][edit_object] = json::parse(&result).unwrap();
                    } else {
                        let result: String = edit::edit(json::stringify(data[media_category][media_name][season_name].clone())).unwrap();

                        data[media_category][media_name][season_name] = json::parse(&result).unwrap();
                    }
                } else {
                    error_flat("");
                    println!("Media '{}' doesn't exist in category '{}'!", media_name, media_category);
                    stop();
                }
            } else {
                error("Invalid category in command 'editseason'");
                warning_flat("editseason <season> <edit> <name> ", USAGE_PREFIX);
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
            error("Insufficient arguments for command 'editseason'");
            warning("editseason <season> <edit> <name> <category>", USAGE_PREFIX);
            stop();
        }    
    } else if args[1].to_ascii_lowercase() == "createseason" {
        if args.len() == 5 {
            let season_name: &str = &args[2];
            let media_name: &str = &args[3];
            let media_category: &str = &args[4];
            if CATEGORIES.contains(&args[4].as_str()) {
                if data[media_category][media_name].has_key(season_name) {
                    error_flat("");
                    println!("Media '{}' already has a season named '{}'!", media_name, season_name);
                    stop();
                }

                data[media_category][media_name][season_name] = object! {
                    "studio": "",
                    "rating": 0,
                    "notes": ""
                }
            } else {
                error("Invalid category in command 'createseason'");
                warning_flat("createseason <season> <name> ", USAGE_PREFIX);
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
            error("Insufficient arguments for command 'createseason'");
            warning("createseason <season> <name> <category>", USAGE_PREFIX);
            stop();
        }   
    } else {
        error_flat("Could not recognize command '");
        print!("{}'!", args[1]);
        stop();
    }

    let mut path: String;

    if cfg!(windows) {
        path = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        path.push_str("\\medialog.json");
    } else if cfg!(unix) {
        path = dirs::home_dir().unwrap().into_os_string().into_string().unwrap();
        path.push_str("/medialog.json");
    } else {
        path = String::from("medialog.json");
    }

    let mut file = File::create(&path)?;
    file.write_all(json::stringify(data).as_bytes())?;
    Ok(())
}