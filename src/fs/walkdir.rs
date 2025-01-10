use crate::{
    fs,
    path::{Path, PathBuf},
};
use alloc::vec;
use alloc::vec::Vec;

pub fn walkdir<P: AsRef<Path>>(path: P) -> Vec<PathBuf> {
    let mut result = Vec::new();
    let path = path.as_ref();

    // Check if the provided path exists and is a directory
    if path.is_dir() {
        // Use a stack for iterative recursion
        let mut stack = vec![path.to_path_buf()];

        while let Some(current) = stack.pop() {
            if let Ok(entries) = fs::read_dir(&current) {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    result.push(entry_path.clone());

                    // If the entry is a directory, add it to the stack
                    if entry_path.is_dir() {
                        stack.push(entry_path);
                    }
                }
            }
        }
    }

    result
}
