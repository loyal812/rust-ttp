#![forbid(unsafe_code)]
use clap::Parser;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
#[clap(name = "Telegram theme parser", author, version, about)]
struct Args {
    /// Path to the .attheme (or .ttp if used with -r) file
    #[clap(value_parser, value_name = "theme")]
    file: PathBuf,

    /// Use this flag to reverse parsing of a theme
    ///
    /// You have to provide a .ttp file in <theme>
    #[clap(short, long, action)]
    revert: bool,

    /// Use this flag to sort lines in alphabetical order
    #[clap(short, long, action)]
    sort: bool,

    /// Path to the output file
    #[clap(short, long, value_parser)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    let contents: String = match fs::read_to_string(&args.file) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to read the file: {e}");
            process::exit(1);
        }
    };
    if contents.is_empty() {
        println!("File is empty");
        process::exit(1);
    }
    let pattern = match Regex::new(if args.revert {
        r"^([a-zA-Z_\d]*): #([A-Fa-f\d]{8})$"
    } else {
        r"^([a-zA-Z_\d]*)=(-?\d*)$"
    }) {
        Ok(r) => r,
        Err(e) => {
            println!("Can't compile regex: {e}");
            process::exit(1);
        }
    };
    let mut lines: Vec<String> = contents
        .lines()
        .filter_map(|x| {
            if pattern.is_match(x) {
                let parts = match pattern.captures(x) {
                    Some(e) => e,
                    None => {
                        println!("Weird line (can't create captures): {x}"); // Should be impossible, but okay
                        return None;
                    }
                };
                if parts[1].is_empty() {
                    println!("Weird line (empty parameter): {x}");
                    None
                } else if parts[2].is_empty() {
                    println!("Weird line (empty value or value is not a number): {x}");
                    None
                } else if args.revert {
                    Some(format!(
                        "{}={}",
                        &parts[1],
                        u32::from_str_radix(&parts[2], 16).unwrap_or_default() as i32
                    ))
                } else {
                    Some(format!(
                        "{}: #{:08X}",
                        &parts[1],
                        &parts[2].parse::<i32>().unwrap_or_default()
                    ))
                }
            } else {
                println!("Weird line (can't parse): {x}");
                None
            }
        })
        .collect::<Vec<String>>();
    if args.sort {
        lines.sort();
    }
    if lines.is_empty() {
        println!("There's nothing to write!");
        process::exit(1);
    }
    let outpath: String = match args.output {
        Some(path) => path.to_str().unwrap_or("out").to_owned(),
        None => {
            format!(
                "{}.{}",
                &args
                    .file
                    .file_stem()
                    .expect("No file provided")
                    .to_str()
                    .unwrap_or("out"),
                if args.revert { "attheme" } else { "ttp" }
            )
        }
    };
    match fs::write(&outpath, lines.join("\n")) {
        Ok(_o) => println!("Done writing to file: {}", outpath),
        Err(e) => {
            println!("Error writing to file: {e}");
            process::exit(1);
        }
    };
}
