use colored::Colorize;
use std::fs;
use std::path::PathBuf;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project() {
    fs::create_dir("build");
    fs::create_dir("build/tmp");
    fs::create_dir("build/tmp/source");
    fs::create_dir("build/tmp/resources");

    println!("{}", "Copying basic files and source code...".bold());
    match fs::copy("manifest.xml", "build/tmp/manifest.xml") {
        Ok(_) => {}
        Err(_) => {
            eprintln!("{}", "Failed to copy manifest.xml.".red().bold());
        }
    }

    println!("first");
    let source_files = list_sources(PathBuf::from("src")).0;
    println!("second");
    let source_dirs = list_sources(PathBuf::from("src")).1;
    println!("{:?}", source_files);
    println!("{:?}", source_dirs);

    for dir in source_dirs.clone() {
        let mut new_destination: PathBuf = ["build", "tmp", "source"].iter().collect();
        let mut new_entry = String::from(dir.clone().to_str().unwrap());
        new_entry.replace_range(0..4, "");
        new_destination.push(new_entry.clone());
        fs::create_dir(new_destination);
    }

    for entry in source_files.clone() {
        let mut start_destination = entry.clone();
        let mut new_destination: PathBuf = ["build", "tmp", "source"].iter().collect();
        let mut new_entry = String::from(entry.clone().to_str().unwrap());
        new_entry.replace_range(0..4, "");
        new_destination.push(new_entry.clone());
        println!("{:?}", start_destination);
        println!("{:?}", new_destination);

        match fs::copy(start_destination, new_destination) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Failed");
            }
        }
    }
}

fn list_sources(path: PathBuf) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let mut source_files: Vec<PathBuf> = Vec::new();
    let mut source_dirs: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            source_files.push(entry.path());
            println!("is file");
        } else {
            source_dirs.push(entry.path());
            println!("is dir");
            source_files.append(&mut list_sources(PathBuf::from(entry.path())).0);
        }
    }
    return (source_files, source_dirs)
}