use std::io;
use std::io::Write;
use colored::*;
use regex::Regex;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Argument {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn input(message: &str) -> String {
    print!("{}: ", message.bold());
    io::stdout().flush().unwrap(); 

    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}

fn main() -> Result<()> {
    let args = Argument::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:p}`", &args.path))?;

    println!();
    println!("{} {}", "📝 Please input target words!".bold(), "(Type enter or q to exit)".yellow());
    let mut words: Vec<String> = Vec::new();
    loop {
        let input_word = input("[Target]");
        if input_word == "q" || input_word == "" {
            break;
        }
        words.push(input_word);
        println!();
    }

    println!();
    println!("{}", "RESULT".blue().bold());
    let mut did_matched = false;
    for (i, line) in content.lines().enumerate() {
        for word in words.iter_mut() {
            let re = Regex::new(word).unwrap();
            if !re.is_match(line) {
                continue
            }
            did_matched = true;
            println!("line{}: {} [{}]", i + 1, line, word);
        }
    }

    if !did_matched {
        println!("{}", "No Matched.")
    }

    Ok(())
}
