use std::path::PathBuf;
mod file_operations;
mod gui;

fn main() {
    let home_dir = dirs::home_dir().expect("Failed to determine home directory.");
    let home_dir_path = home_dir.to_string_lossy().into_owned();
    let files = file_operations::list_files(&home_dir_path);

    gui::run(&home_dir_path, &files);
}

