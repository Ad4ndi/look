use std::{fs, io, path::{Path, PathBuf}};

pub fn find_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            files.extend(find_files(&path)?);
        } else {
            files.push(path);
        }
    }
    Ok(files)
}
