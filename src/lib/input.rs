use std::{env, fs, path::PathBuf};

pub fn load_input(path: &str) -> String {
    let dir = env::current_dir().unwrap();
    let home_dir_path = dir.as_path();
    let joined_path = home_dir_path.join(PathBuf::from(path));
    let contents = fs::read_to_string(joined_path.as_os_str()).unwrap();
    return contents;
}
