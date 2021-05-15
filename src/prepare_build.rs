use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Error;

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

    let mut source_files:Vec<PathBuf> = Vec::new();

    for file in fs::read_dir("src").unwrap() {
        println!("{:?}", file);
        let file = file.unwrap();
        if file.file_type().unwrap().is_file() {
            source_files.push(file.path());
        } else {
            eprintln!("{}", "Currently nested files in src are not supported. Please move everything to the root.".red().bold());
            std::process::exit(1);
        }
    }

    for file in source_files {
        let file = file.to_str().unwrap().rsplit("/").next().unwrap();
        let mut start_destination = PathBuf::from("src");
        start_destination.push(file.clone());
        let mut new_destination: PathBuf = ["build", "tmp", "source"].iter().collect();
        new_destination.push(file.clone());
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