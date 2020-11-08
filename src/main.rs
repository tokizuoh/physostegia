use std::io;
use std::io::Write;
use colored::*;
use regex::Regex;
use structopt::StructOpt;
use anyhow::{Context, Result};
use itertools::join;

#[derive(StructOpt)]
struct Argument {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Line {
    number: usize,
    matched_words: Vec<String>,
    content: String,
}

fn input(message: &str) -> String {
    print!("{}: ", message.bold());
    io::stdout().flush().unwrap(); 

    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}

fn wrap_brackets(word: &str) -> String {
    "[".to_string() + word + "]"
}

fn main() -> Result<()> {
    let args = Argument::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:p}`", &args.path))?;

    println!();
    println!("{} {}", "📝 Please input target words!".bold(), "(Type enter to exit)".yellow());
    let mut words: Vec<String> = Vec::new();
    loop {
        let input_word = input("[Target]");
        if input_word == "" {
            break;
        }
        words.push(input_word);
    }

    let mut lines: Vec<Line> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let mut matched_words: Vec<String> = Vec::new();
        for word in words.iter_mut() {
            let re = Regex::new(word).unwrap();
            if !re.is_match(line) {
                continue
            }
            matched_words.push(word.to_string());
        }
        if matched_words.is_empty() {
            continue
        }
        lines.push(Line{number: i, matched_words:matched_words, content: line.to_string()});
    }

    println!();
    if lines.is_empty() {
        println!("{}", "No matched.".red().bold());
    } else {
        println!("{}", "Matched!".green().bold());
        for line in lines {
            let brackets_matched_words: Vec<String> = line.matched_words.into_iter().map(|x| wrap_brackets(&x)).collect();
            let matched_words_string = join(brackets_matched_words, "");
            println!("line{}: {} {}", line.number, line.content.bold(), matched_words_string.blue());
        }
    }
    Ok(())
}

#[test]
fn wrap_brackets_side() {
    let a = "[test]";
    let b = wrap_brackets("test");
    assert_eq!(a, b);
}