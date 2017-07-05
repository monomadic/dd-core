use std::path::{Path, PathBuf};
use std::env;

pub fn plugin_dir() -> PathBuf {
    // #[cfg(target_os = "windows")]
    // return fs::get_folder_path().unwrap();
    // #[cfg(target_os = "macos")]
    // return ::std::path::PathBuf::from(".");

    env::current_exe().unwrap()
}
