use std::path::{PathBuf};
use colored::Colorize;
use crate::fs::{create_zip, gather_files_in};
use crate::rt::Context;
use crate::utils::pretty_path;

pub fn load_manifest_in(mut path: PathBuf) {
    let file_str = std::fs::read_to_string(&path);
    if file_str.is_err() {
        println!("{}", format!("Failed to load fxmainfest.lua at {}, due to {}", pretty_path(&path), file_str.err().unwrap().to_string()).red().bold());
    } else {
        path.pop();
        std::env::set_current_dir(&path).unwrap();
        println!("{}", format!("fxmanifest.lua loaded from {}", pretty_path(&path).yellow()).green());
        let ctx = Context::new();
        ctx.load_directives();
        ctx.load_str(file_str.unwrap());
        let found_files = ctx.get_files();
        println!("{}", format!("Found {} file entries (globs are not yet expanded)", found_files.len()).green());
        let mut gathered = gather_files_in(path.clone(), found_files);
        println!("{}", format!("Found {} actually existing file(s) after glob expansion", gathered.len()).green());
        gathered.push("fxmanifest.lua".into());
        let mut disp_path = path.clone();
        let out = create_zip(gathered, path);
        disp_path.push(out);
        println!("{}", format!("Resource archive created at: {}", pretty_path(&disp_path)).green());
    }
}