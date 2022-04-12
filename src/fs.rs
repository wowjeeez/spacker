use std::io::Write;
use std::path::PathBuf;
use glob::{glob_with, MatchOptions};
use colored::Colorize;
use crate::utils::{get_dirs, pretty_path};

pub fn gather_files_in(mut in_dir: PathBuf, inp_files: Vec<String>) -> Vec<PathBuf> {
    let options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: true,
        require_literal_leading_dot: false,
    };

    let mut files: Vec<PathBuf> = vec![];
    for file in inp_files {
        in_dir.push(&file);
        let gl = glob_with(file.as_str(), options);
        if gl.is_ok() {
            let orig_len = files.len();
            let u_w_gl = gl.unwrap();
            for entry in u_w_gl {
                if entry.is_ok() {
                        let uw = entry.unwrap();
                        if uw.is_file() {
                            files.push(uw);
                        }
                } else {
                    println!("{}", format!("Error reading glob result in'{}, due to: {}", file.bold(), entry.err().unwrap().to_string().bold()).red())
                }
            }
            if orig_len == files.len() {
                println!("{}", format!("Pattern {} returned 0 files", file.bold()).yellow())
            } else {
                println!("{}", format!("Pattern {} returned {} file(s)", file.bold(), files.len() - orig_len).green())
            }
        } else {
            println!("{}", format!("Error reading file(s) in entry: {}, due to: {}", file.bold(), gl.err().unwrap().msg.bold()).red())
        }
        in_dir.pop();
    }
    files
}

pub fn create_zip(files: Vec<PathBuf>, root: PathBuf) -> PathBuf {
    let zip_path = PathBuf::from("resource.zip");
    let entry = std::fs::File::create(&zip_path).unwrap();
    let mut zip = zip::ZipWriter::new(entry);
    let mut written_dirs = vec![];
    for file in files.iter() {
        //weird code to convert absolute path into a relative one
        let dirs = get_dirs(&file, root.clone()).iter().map(|p| PathBuf::from(p.to_str().unwrap().replace(root.to_str().unwrap(), ""))).collect::<Vec<PathBuf>>();
        for dir in dirs {
            let pth = dir.to_str().unwrap().replace("\\", "/");
            if !written_dirs.contains(&pth) {
                written_dirs.push(pth.to_string());
                zip.add_directory(pth, Default::default()).unwrap();
            }
        }
        println!("{}", format!("Adding {} to the archive", pretty_path(&file)).yellow());
        let file_str = std::fs::read_to_string(&file).unwrap();
        zip.start_file(file.to_str().unwrap(), Default::default()).unwrap();
        zip.write_all(file_str.as_bytes()).unwrap();
    }
    zip_path
}