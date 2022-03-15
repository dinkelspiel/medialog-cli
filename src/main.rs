use std::path::Path;
use std::io::Read;
use json;
use json::JsonValue;
use json::object;
use std::env;
use std::fs::File;
use std::io::Write;
use colored::*;
use strsim::{jaro};

fn error(message: &str) {
    println!("{}: {}!", "error".bright_red(), message);
}

fn error_flat(message: &str) {
    print!("{}: {}", "error".bright_red(), message);
}

fn warning(message: &str, prefix: &str) {
    println!("{}: {}.", prefix.yellow(), message); 
}

fn warning_flat(message: &str, prefix: &str) {
    print!("{}: {}", prefix.yellow(), message); 
}

fn help(message: &str) {
    println!("{}: {}.", "help".bright_cyan(), message);
}

fn help_flat(message: &str) {
    print!("{}: {}", "help".bright_cyan(), message);
}

const USAGE_PREFIX: &str = "usage";
const OPTIONS_PREFIX: &str = "options";

const CATEGORIES: [&str; 5] = ["anime", "manga", "movie", "book", "podcast"];

const STATUS: [&str; 5] = ["planned", "watching", "completed", "paused", "dropped"];
const STATUS_LOWER: [char; 5] = ['p', 'w', 'c', 'p', 'd'];

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
        return Ok(());
    }

    if args[1].to_ascii_lowercase() == "add" {
        if args.len() >= 4 {
            let media_name: &str = &args[2].to_ascii_lowercase();
            let media_display_name: &str = &args[2];
            let media_category: &str = &args[3].to_ascii_lowercase();
            if CATEGORIES.contains(&args[3].to_ascii_lowercase().as_str()) {
                // let media_object: JsonValue = object! {
                //     name: media_name,
                //     category: media_category,
                //     seasons: {}
                // };

                if data[media_category].has_key(media_name) {
                    error_flat("");
                    println!("Media '{}' in category '{}' already exists.", media_name, media_category);
                    return Ok(());
                }

                data[media_category][media_name] = object! {
                    "disname": media_display_name,
                    "status": "planned",
                }; 
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
            return Ok(());
        }   
    } else if args[1].to_ascii_lowercase() == "edit" {
        if args.len() >= 4 {
            let media_name: &str = &args[2].to_ascii_lowercase();
            let media_category: &str = &args[3].to_ascii_lowercase();
            if CATEGORIES.contains(&args[3].to_ascii_lowercase().as_str()) {
                if !data[media_category].has_key(media_name) {
                    error_flat("");
                    println!("Media '{}' doesn't exist in category '{}'!", media_name, media_category);
                    
                    let mut highest_similarity: f64 = 0.0;
                    let mut highest_similarity_media: &str = "";
                    
                    for i in data[media_category].entries() {
                        // println!("{}, {}", i.0, media_name);
                        let similarity: f64 = jaro(media_name, i.0);
                        if similarity > highest_similarity {
                            highest_similarity = similarity;
                            highest_similarity_media = i.0;
                        }
                    }

                    if highest_similarity > 0.80 {
                        help_flat("");
                        println!("Found media with a name with {}% similarity called '{}'.", &(highest_similarity * 100.0).to_string()[0..2], data[media_category][highest_similarity_media]["disname"]);
                    }
                    return Ok(());
                }

                let result: String = edit::edit(json::stringify(data[media_category][media_name].clone())).unwrap();
                data[media_category][media_name] = json::parse(&result).unwrap();
            } else {
                error("Invalid category in command 'edit'");
                warning_flat("edit <name> ", USAGE_PREFIX);
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
            error("Insufficient arguments for command 'edit'");
            warning("edit <name> <category>", USAGE_PREFIX);
            return Ok(());
        } 
    } else if args[1].to_ascii_lowercase() == "editstatus" {
        if args.len() >= 5 {
            let media_name: &str = &args[3].to_ascii_lowercase();
            let media_category: &str = &args[4].to_ascii_lowercase();
            if CATEGORIES.contains(&args[4].to_ascii_lowercase().as_str()) {
                if !STATUS.contains(&args[2].to_ascii_lowercase().as_str()) || !STATUS_LOWER.contains(&args[2].to_ascii_lowercase().as_str().chars().nth(0).unwrap()) {
                    error_flat("");
                    println!("Status '{}' doesn't exist!", args[2]);
                    help("Consider using 'planned', 'watching', 'completed', 'paused' or 'dropped'");
                    return Ok(());
                }

                if !data[media_category].has_key(media_name) {
                    error_flat("");
                    println!("Media '{}' doesn't exist in category '{}'!", media_name, media_category);
                    return Ok(());
                }

                data[media_category][media_name]["status"] = json::parse(&args[2]).unwrap();
            } else {
                error("Invalid category in command 'edit'");
                warning_flat("editstatus <status> <name> ", USAGE_PREFIX);
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
            error("Insufficient arguments for command 'edit'");
            warning("editstatus <status> <name> <category>", USAGE_PREFIX);
            return Ok(());
        }   
    } else if args[1].to_ascii_lowercase() == "editseason" {
        if args.len() >= 6 {
            let season_name: &str = &args[2].to_ascii_lowercase();
            let edit_object: &str = &args[3].to_ascii_lowercase();
            let media_name: &str = &args[4].to_ascii_lowercase();
            let media_category: &str = &args[5].to_ascii_lowercase();
            if CATEGORIES.contains(&args[5].to_ascii_lowercase().as_str()) {
                if data[media_category].has_key(media_name) {
                    if !data[media_category][media_name].has_key(season_name) {
                        error_flat("");
                        println!("Season {} doesn't exist in media {}!", season_name, media_name);
                        help_flat("");
                        println!("Consider running 'medialog createseason {} {} {}'!", season_name, media_name, media_category);
                        return Ok(());
                    }

                    if !["studio", "rating", "notes", "json"].contains(&edit_object) {
                        error_flat("");
                        println!("Invalid media property {}!", edit_object);
                        help("Use 'studio', 'rating', 'notes' or 'json'!");
                        return Ok(());
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
                    return Ok(());
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
            return Ok(());
        }    
    } else if args[1].to_ascii_lowercase() == "createseason" {
        if args.len() >= 5 {
            let season_name: &str = &args[2].to_ascii_lowercase();
            let media_name: &str = &args[3].to_ascii_lowercase();
            let season_display: &str = &args[2]; 
            let media_category: &str = &args[4].to_ascii_lowercase();
            if CATEGORIES.contains(&args[4].to_ascii_lowercase().as_str()) {
                if data[media_category][media_name].has_key(season_name) {
                    error_flat("");
                    println!("Media '{}' already has a season named '{}'!", media_name, season_name);
                    return Ok(());
                }

                let mut studio: String = String::from("");
                if args.len() == 6 {
                    studio = args[5].clone().to_ascii_lowercase();
                }

                data[media_category][media_name][season_name] = object! {
                    "disname": season_display,
                    "studio": studio,
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
            return Ok(());
        }
    } else if args[1].to_ascii_lowercase() == "next" {
        if args.len() >= 3 {
            if CATEGORIES.contains(&args[2].to_ascii_lowercase().as_str()) {
                let mut watched: bool = false;
                for (key, value) in data[&args[2]].entries() {
                    if value["status"] == "planned" {
                        println!("Your next {} on the list is {}!", args[2], key);
                        watched = true;
                        break;
                    }
                }
                if !watched {
                    println!("You have watched all your {}.", args[2]);
                }
                return Ok(());
            } else {
                error("Invalid category in command 'next'");
                warning_flat("next ", USAGE_PREFIX);
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
            error("Insufficient arguments for command 'next'");
            warning("createseason <category>", USAGE_PREFIX);
            return Ok(());
        }
    } else {
        error_flat("Could not recognize command '");
        print!("{}'!", args[1]);
        return Ok(());
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