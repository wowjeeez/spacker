mod manif_loader;
mod utils;
mod rt;
mod fs;
use std::time::{Instant};

use std::path::PathBuf;
use colored::Colorize;
use crate::manif_loader::load_manifest_in;

fn main() {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();
    println!("{}", "Finding your fxmanifest.lua file...".green().bold());
    let manif_re_path = args.get(1);
    if manif_re_path.is_some() {
            let pth = PathBuf::from(manif_re_path.unwrap());
            if pth.is_relative() {
                let canon = std::fs::canonicalize(&pth);
                if canon.is_err() {
                    println!("{}", format!("Failed to canonicalize path due to: {}", canon.err().unwrap().to_string()).red().bold());
                    std::process::exit(1);
                } else {
                    load_manifest_in(canon.unwrap());
                }
            } else {
                load_manifest_in(pth);
            }
    } else {
        println!("{}", "No fxmanifest.lua file was found, looking in the current directory...".yellow());
        let mut dir = std::env::current_dir().unwrap();
        dir.push("fxmanifest.lua");
        load_manifest_in(dir);
    }
    let duration = start.elapsed();
    println!("{}", format!("Done in {} ms", duration.as_millis()).green())
}
