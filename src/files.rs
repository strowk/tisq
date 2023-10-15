use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn open_tisq_root() -> eyre::Result<PathBuf> {
    // get user directory or current
    let parent_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => Path::new("./").to_path_buf(),
    };
    let tisq_folder = parent_dir.join(".tisq");
    if !tisq_folder.exists() {
        fs::create_dir_all(&tisq_folder)?;
    }
    Ok(tisq_folder)
}
