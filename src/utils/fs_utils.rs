use std::fs;
use std::path::{PathBuf, Path};
use anyhow::Result;
use std::env;

/// Big thanks to https://stackoverflow.com/a/60406693
pub struct FsUtils {  }

impl FsUtils {
    pub fn recursive_copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<()> {
        let mut stack = Vec::new();
        stack.push(PathBuf::from(from.as_ref()));

        let output_root = PathBuf::from(to.as_ref());
        let input_root = PathBuf::from(from.as_ref()).components().count();

        while let Some(working_path) = stack.pop() {
            // Generate a relative path
            let src: PathBuf = working_path.components().skip(input_root).collect();

            // Create a destination if missing
            let dest = if src.components().count() == 0 {
                output_root.clone()
            } else {
                output_root.join(&src)
            };
            if fs::metadata(&dest).is_err() {
                fs::create_dir_all(&dest)?;
            }

            for entry in fs::read_dir(working_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    match path.file_name() {
                        Some(filename) => {
                            let dest_path = dest.join(filename);
                            fs::copy(&path, &dest_path)?;
                        }
                        None => {
                            eprintln!("failed: {:?}", path);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn recursive_delete<U: AsRef<Path>>(dir: U) -> Result<()> {
        let mut stack = Vec::new();
        stack.push(PathBuf::from(dir.as_ref()));

        while let Some(working_path) = stack.pop() {
            for entry in fs::read_dir(working_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    match path.file_name() {
                        Some(_) => {
                            if path.is_file() {
                                fs::remove_file(&path)?;
                            } else {
                                fs::remove_dir(&path)?;
                            }
                        }
                        None => {
                            eprintln!("failed: {:?}", path);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn workdir(end: Option<PathBuf>) -> Result<PathBuf> {
        let mut result = env::current_dir()?;
        if end != None {
            result.push(end.unwrap())
        }
        Ok(result)
    }
}
