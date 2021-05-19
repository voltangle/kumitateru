use colored::Colorize;
use std::fs;
use std::path::{PathBuf, Path};

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project(manifest: String) {
    fs::create_dir("build");
    fs::create_dir("build/tmp");
    fs::create_dir("build/tmp/source");
    fs::create_dir("build/tmp/resources");

    println!("{}", "Copying source code...".bold());

    fs::File::create(PathBuf::from("build/tmp/manifest.xml"));
    fs::write(PathBuf::from("build/tmp/manifest.xml"), manifest);

    copy(PathBuf::from("src"), PathBuf::from("build/tmp/source"));
    println!("{}", "Preparing language resources...".bold());

}


// Big thanks to https://stackoverflow.com/a/60406693
fn copy<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
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
