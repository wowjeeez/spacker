use std::path::{PathBuf};
use colored::{ColoredString, Colorize};
pub fn pretty_path(pth: &PathBuf) -> ColoredString {
    let str = pth.display();
    str.to_string().bold()
}

pub fn get_dirs(pth: &PathBuf, mut root: PathBuf) -> Vec<PathBuf> {
    let mut dirs = vec![];
    for part in pth.iter() {
        root.push(part);
        if root.is_dir() {
            dirs.push(root.clone());
        } else {
            break
        }
    }
    dirs
}