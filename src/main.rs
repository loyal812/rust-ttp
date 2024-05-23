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
    let mut lines: Vec<&str> = contents.lines().map(|x| x.trim()).collect::<Vec<&str>>();
    lines.sort();
    let mut lines_parsed: Vec<String> = vec![];
    let pattern = Regex::new(r"^([a-zA-Z_\d]*)=(-?\d*)$").unwrap();
    for line in lines {
        if pattern.is_match(line) {
            let parts = pattern.captures(line).unwrap();
            if parts[1].is_empty() {
                println!("Weird line (empty parameter): {line}");
            } else if parts[2].is_empty() {
                println!("Weird line (empty value or value is not a number): {line}");
            } else {
                lines_parsed.push(format!(
                    "{}: #{:08X}",
                    &parts[1],
                    &parts[2].parse::<i32>().unwrap()
                ))
            }
        } else {
            println!("Weird line (can't parse): {line}");
        }
    }
    _ = fs::write(
        format!(
            "{}.ttp",
            &args
                .file
                .file_stem()
                .expect("No file provided")
                .to_str()
                .unwrap()
        ),
        lines_parsed.join("\n"),
    );
    0;
}
