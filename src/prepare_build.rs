use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use crate::utils::fs_recursive_copy::copy;
use crate::utils::config::parse_config;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project(manifest: String) {
    let _ = fs::create_dir("build");
    let _ = fs::create_dir("build/tmp");
    let _ = fs::create_dir("build/tmp/source");
    let _ = fs::create_dir("build/tmp/resources");

    println!("{}", "Copying source code...".bold());

    let _ = fs::File::create(PathBuf::from("build/tmp/manifest.xml"));
    let _ = fs::write(PathBuf::from("build/tmp/manifest.xml"), manifest);

    let _ = copy(PathBuf::from("src"), PathBuf::from("build/tmp/source"));
    println!("{}", "Preparing resources...".bold());
    let mut device_specific_res: Vec<String> = Vec::new();

    // Here we get all device-specific resources
    for resource in vec!["resources/drawables", "resources/layouts", "resources/fonts", "resources/menus", "resources/settings"] {
        for entry in fs::read_dir(PathBuf::from(resource)) {
            for entry in entry {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_dir() {
                    if !device_specific_res.contains(&entry.file_name().to_str().unwrap().to_string()) {
                        device_specific_res.push(entry.file_name().to_str().unwrap().to_string());
                    }
                }
            }
        }
    }

    // And create directories
    for dir in device_specific_res {
        let mut end_dir = PathBuf::from("build/tmp");
        let mut end_dirname: String = "resources-".parse().unwrap();
        end_dirname.push_str(&*dir);
        end_dir.push(end_dirname);
        let _ = fs::create_dir(end_dir);
    }

    // Then create directories for language resources
    for language in parse_config(fs::read_to_string("kumitateru.toml").unwrap()).package_meta.languages {
        let mut end_dir = PathBuf::from("build/tmp");
        let mut end_dirname: String = "resources-".parse().unwrap();
        end_dirname.push_str(&*language);
        end_dir.push(end_dirname);
        let _ = fs::create_dir(end_dir);
    }
}
