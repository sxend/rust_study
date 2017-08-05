use std::path::{Path, PathBuf};

fn main() {
    let include_line_path: PathBuf = Path::new("./src/bin/path_sample.rs").to_owned();
    let include_line_path: PathBuf = if include_line_path.is_absolute() {
        include_line_path
    } else if include_line_path.is_relative() {
        let current_path = Path::new("./src/bin/path_sample.rs");
        let mut path_buf = current_path.parent().unwrap().to_path_buf();
        path_buf.push(&include_line_path);
        path_buf
    } else {
        panic!("unexpected path: {:?}", include_line_path);
    };
    println!("include_line_file: {:?}, exists: {}", include_line_path, include_line_path.exists());
}