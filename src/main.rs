#![forbid(unsafe_code)]
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[clap(name = "Telegram theme parser", author, version, about)]
struct Args {
    /// Path to the .attheme file
    #[clap(value_parser, value_name = "theme")]
    file: PathBuf,

    /// Use this flag to reverse parsing of a theme.
    /// You have to provide a .ttp file in <theme>
    #[clap(short, long, action)]
    revert: bool,
}

fn main() {
    let args = Args::parse();
    let contents: String;
    match fs::read_to_string(&args.file) {
        Ok(c) => contents = c,
        Err(e) => {
            println!("Failed to read the file: {e}");
            process::exit(1);
        }
    }
    if contents.is_empty() {
        println!("File is empty");
        process::exit(1);
    }
    let pattern = if args.revert {
        Regex::new(r"^([a-zA-Z_\d]*): #([A-Fa-f\d]{8})$").unwrap()
    } else {
        Regex::new(r"^([a-zA-Z_\d]*)=(-?\d*)$").unwrap()
    };
    let mut lines: Vec<String> = contents
        .lines()
        .filter_map(|x| {
            if pattern.is_match(x) {
                let parts = pattern.captures(x).unwrap();
                if parts[1].is_empty() {
                    println!("Weird line (empty parameter): {x}");
                    return None;
                } else if parts[2].is_empty() {
                    println!("Weird line (empty value or value is not a number): {x}");
                    return None;
                } else {
                    return if args.revert {
                        Some(format!(
                            "{}={}",
                            &parts[1],
                            u32::from_str_radix(&parts[2], 16).expect("Invalid hex value") as i32
                        ))
                    } else {
                        Some(format!(
                            "{}: #{:08X}",
                            &parts[1],
                            &parts[2].parse::<i32>().unwrap()
                        ))
                    };
                }
            } else {
                println!("Weird line (can't parse): {x}");
                return None;
            }
        })
        .collect::<Vec<String>>();
    lines.sort();
    if lines.is_empty() {
        println!("There's nothing to write!");
        process::exit(1);
    }
    _ = fs::write(
        format!(
            "{}.{}",
            &args
                .file
                .file_stem()
                .expect("No file provided")
                .to_str()
                .unwrap(),
            if args.revert { "attheme" } else { "ttp" }
        ),
        lines.join("\n"),
    );
    0;
}
