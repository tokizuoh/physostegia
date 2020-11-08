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

struct Line {
    number: usize,
    content: String,
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
    let mut lines: Vec<Line> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        for word in words.iter_mut() {
            let re = Regex::new(word).unwrap();
            if !re.is_match(line) {
                continue
            }
            lines.push(Line{number: i, content: word.to_string()});
        }
    }

    if lines.len() > 0 {
        panic!("hoge");
    } else {
        println!("{}", "No matched.".red().bold());
    }
    Ok(())
}
