use std::fs;
use std::process::Command;
use clap::Parser;

/// Simple program to scaffold next project
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Should we open the browser
    #[arg(short, long, default_value_t = false)]
    silent: bool
}

fn main() {
    let dir_path = "./rs";
    let mut dirs: Vec<_> = fs::read_dir(dir_path).unwrap()
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| {
            entry.file_name().to_str().and_then(|name| {
                if name.starts_with('p') {
                    name[1..].parse::<u32>().ok().map(|num| (num, name.to_string()))
                } else {
                    None
                }
            })
        })
        .collect();

    dirs.sort_by_key(|(num, _)| *num);

    let next_number = dirs.last().map_or(1, |(last_num, _)| last_num + 1);
    let next_name = format!("p{:04}", next_number);
    let problem_url = format!("https://projecteuler.net/problem={}", next_number);

    Command::new("cargo")
        .args(["new", "--bin", next_name.as_str()])
        .current_dir(dir_path)
        .status()
        .expect("failed to execute cargo");

    println!("cd {dir_path}/{next_name}");
    println!("Problem URL: {problem_url}");
}