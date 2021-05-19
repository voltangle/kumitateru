use colored::Colorize;
use std::fs;
use std::path::PathBuf;

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

    let source_files = list_sources(PathBuf::from("src")).0;
    let source_dirs = list_sources(PathBuf::from("src")).1;

    // Creating directory structure
    for dir in source_dirs.clone() {
        let mut new_destination: PathBuf = ["build", "tmp", "source"].iter().collect();
        let mut new_entry = String::from(dir.clone().to_str().unwrap());
        new_entry.replace_range(0..4, "");
        new_destination.push(new_entry.clone());
        fs::create_dir(new_destination);
    }

    // And then copying files
    for entry in source_files.clone() {
        let start_destination = entry.clone();
        let mut new_destination: PathBuf = ["build", "tmp", "source"].iter().collect();
        let mut new_entry = String::from(entry.clone().to_str().unwrap());
        new_entry.replace_range(0..4, "");
        new_destination.push(new_entry.clone());

        fs::copy(start_destination, new_destination);
    }
    println!("{}", "Preparing language resources...".bold());
}

fn list_sources(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut source_files: Vec<PathBuf> = Vec::new();
    let mut source_dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            source_files.push(entry.path());
        } else {
            source_dirs.push(entry.path());
            source_files.append(&mut list_sources(PathBuf::from(entry.path())).0);
        }
    }
    return (source_files, source_dirs)
}