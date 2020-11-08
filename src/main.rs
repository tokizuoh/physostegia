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

struct Query {
    _type: u8,
    word: String,
}

struct Line {
    number: usize,
    matched_words: Vec<String>,
    content: String,
}

fn input_string(message: &str) -> String {
    print!("{}: ", message.bold());
    io::stdout().flush().unwrap(); 

    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}

// FIXME: Resultで返す
fn input_u8(message: &str) -> u8 {
    let word = input_string(message);
    if word.is_empty() {
        return 9
    }
    translate_string_to_u8(word)
}

fn translate_string_to_u8(word: String) -> u8 {
    let num: u8 = word.parse().unwrap();
    num
}

fn wrap_brackets(word: &str) -> String {
    "[".to_string() + word + "]"
}

fn main() -> Result<()> {
    let args = Argument::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:p}`", &args.path))?;

    println!();
    // println!("{} {}", "📝 Please input target words!".bold(), "(Type enter to exit)".yellow());
    println!("{} {}", "📝 Please input regex type & target!".bold(), "(Type enter to exit)".yellow());
    println!("{}", "[Type]");
    println!("{}", "1: PARTIAL");
    println!("{}", "2: END");
    println!();
    let mut queries: Vec<Query> = Vec::new();
    loop {
        // let input_word = input("[Target]");
        let input_type = input_u8("[Type]");
        
        // FIXME: Resultで受け取る
        if input_type == 9 {
            break;
        }
        let input_word = input_string("[Target]");
        if input_word == "" {
            break;
        }
        queries.push(Query{_type: input_type, word: input_word});
    }

    let mut lines: Vec<Line> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        let mut matched_words: Vec<String> = Vec::new();
        for query in queries.iter_mut() {
            let word = match query._type {
                1 => query.word.clone(),
                2 => format!("{}$", &query.word),
                _ => panic!("NOT AVAILABLE")
            };
            let re = Regex::new(&word).unwrap();
            if !re.is_match(line) {
                continue
            }
            matched_words.push(query.word.to_string());
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
            // FIXME: 入力のタイプで出力を分ける
            println!("line{}: {} {}", line.number, line.content.bold(), matched_words_string.blue());
        }
    }
    
    Ok(())
}

#[test]
fn test_wrap_brackets() {
    let a = "[test]";
    let b = wrap_brackets("test");
    assert_eq!(a, b);
}

#[test]
fn test_translate_string_to_u8() {
    let a: u8 = 1;
    let b = translate_string_to_u8("1".to_string());
    assert_eq!(a, b);
}