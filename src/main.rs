use std::{env, fs};
use std::process;
use std::error::Error;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // read file
    println!("In file {}", conf.file_path);
    if let Err(e) = minigrep::run(conf) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

