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
    print!("{}: ", message);
    io::stdout().flush().unwrap(); 

    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    word.trim().to_string()
}

fn main() -> Result<()> {
    // 入力受付
    let args = Argument::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:p}`", &args.path))?;
    
    // TODO: マッチさせたい正規表現を生成
    // loop {
    //     let a = input("入力させたい");
    //     println!("Result: {}", a.bold());
    //     if a == "q" {
    //         break;
    //     }
    // }

    // 一行ずつ呼んで処理
    for line in content.lines() {
        println!("{}", line);
    }
        
    // let _words = r"^\d{4}";
    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // let re2 = Regex::new(r"^\d{4}-\d{2}-\d{2}").unwrap();
    // assert!(re.is_match("2014-01-01ああ"));
    // println!("{}", 1);
    Ok(())
}
