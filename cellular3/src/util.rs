use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn collect_filenames<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| {
            e.ok().and_then(|e| {
                if e.file_type().is_file() {
                    Some(e.path().to_owned())
                } else {
                    None
                }
            })
        })
        .collect()
}
