use std::env;

use colored::*;


const INVALID_PATH: &'static str = "???";

pub fn render() {
    let current_path: String = env::current_dir()
        .map(|p| p.to_str().unwrap().into())
        .unwrap_or(INVALID_PATH.into());
    // TODO: Add additional shortening?
    let display_path = if let Some(home) = env::home_dir() {
        if let Some(home) = home.to_str() {
            current_path.replace(home, "~")
        } else {
            current_path
        }
    } else {
        current_path
    };

    println!();
    println!("{}", display_path.blue());
}
