use std::env;

use colored::*;


const INVALID_PATH: &'static str = "???";

pub fn render() {
    let current_path: String = env::current_dir()
        .map(|p| p.to_str().unwrap().into())
        .unwrap_or(INVALID_PATH.into());
    // TODO: Add some form of shortening?
    let display_path = current_path.blue();

    println!();
    println!("{}", display_path);
}
