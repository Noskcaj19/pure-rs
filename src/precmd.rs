use std::env;
use git2::{self, Repository};

use colored::*;


const INVALID_PATH: &'static str = "???";

fn render_git_status(repo: &Repository) -> Option<String> {
    let shorthand: String = repo.head().unwrap().shorthand().unwrap().into();

    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);

    let statuses = repo.statuses(Some(&mut opts)).unwrap();

    let dirty = statuses
        .iter()
        .filter(|i| i.status() != git2::STATUS_CURRENT)
        .all(|e| match e.status() {
            s if s.contains(git2::STATUS_INDEX_NEW) => true,
            s if s.contains(git2::STATUS_INDEX_MODIFIED) => true,
            s if s.contains(git2::STATUS_INDEX_DELETED) => true,
            s if s.contains(git2::STATUS_INDEX_RENAMED) => true,
            s if s.contains(git2::STATUS_INDEX_TYPECHANGE) => true,
            s if s.contains(git2::STATUS_WT_NEW) => true,
            s if s.contains(git2::STATUS_WT_MODIFIED) => true,
            s if s.contains(git2::STATUS_WT_DELETED) => true,
            s if s.contains(git2::STATUS_WT_RENAMED) => true,
            s if s.contains(git2::STATUS_WT_TYPECHANGE) => true,
            _ => false,
        });

    let dirty_status = if dirty { "*" } else { "" };

    Some(format!("{}{}", shorthand, dirty_status))
}

pub fn render() {
    let current_path: String = env::current_dir()
        .map(|p| p.to_str().unwrap().into())
        .unwrap_or(INVALID_PATH.into());
    // TODO: Add additional shortening?
    let display_path: String = if let Some(home) = env::home_dir() {
        if let Some(home) = home.to_str() {
            current_path.clone().replace(home, "~")
        } else {
            current_path.clone()
        }
    } else {
        current_path.clone()
    };


    let git_status = Repository::discover(current_path)
        .ok()
        .and_then(|repo| render_git_status(&repo))
        .unwrap_or("".into());

    println!();
    println!("{} {}", display_path.blue(), git_status);
}
