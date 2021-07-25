use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use crate::utils::fs_recursive_copy::{recursive_copy, recursive_delete};
use crate::ser_de::parse_config::parse_config;
use toml::value::Table;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_app_project(manifest: String, dependencies: Table) {
    if PathBuf::from("build/tmp").exists() {
        recursive_delete("build/tmp");
    }
    let _ = fs::create_dir("build");
    let _ = fs::create_dir("build/tmp");
    let _ = fs::create_dir("build/tmp/source");
    let _ = fs::create_dir("build/tmp/resources");

    println!("{}", "Copying source code...".bold());

    let _ = fs::File::create(PathBuf::from("build/tmp/manifest.xml"));
    let _ = fs::write(PathBuf::from("build/tmp/manifest.xml"), manifest);

    let _ = recursive_copy(PathBuf::from("src"), PathBuf::from("build/tmp/source"));
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
    for dir in &device_specific_res {
        let mut end_dir = PathBuf::from("build/tmp");
        let mut end_dirname: String = "resources-".parse().unwrap();
        end_dirname.push_str(&*dir);
        end_dir.push(end_dirname);
        let _ = fs::create_dir(end_dir);
    }

    // Then create directories for language resources and transfer them
    for language in parse_config(fs::read_to_string("package.toml").unwrap()).package_meta.languages {
        if language == "eng" {
            let mut end_dir = PathBuf::from("build/tmp");
            let end_dirname: String = "resources".parse().unwrap();
            end_dir.push(end_dirname);
            end_dir.push("strings");
            let _ = fs::create_dir(&end_dir);

            let mut start_directory = PathBuf::from("resources/strings");
            start_directory.push("main");

            recursive_copy(start_directory, end_dir);
        } else {
            let mut end_dir = PathBuf::from("build/tmp");
            let mut end_dirname: String = "resources-".parse().unwrap();
            end_dirname.push_str(&*language);
            end_dir.push(end_dirname);
            let _ = fs::create_dir(&end_dir);

            let mut start_directory = PathBuf::from("resources/strings");
            start_directory.push(language);

            recursive_copy(start_directory, end_dir);
        }
    }

    // And here we will transfer other resources
    for resource in vec!["drawables", "layouts", "fonts", "menus", "settings"] {
        transfer_device_resources(resource.to_string(), device_specific_res.clone());
        let mut dir = PathBuf::from("build/tmp/resources");
        dir.push(resource);
        fs::create_dir(&dir);
        transfer_main_resources(resource.to_string());
    }

    // At last we transfer dependencies...
    fs::create_dir(PathBuf::from("build/tmp/dependencies/"));
    for (_, value) in dependencies.clone() {
        let output = PathBuf::from(format!("{}{}","build/tmp/dependencies/", value[1].as_str().unwrap()));
        fs::copy(format!("{}/{}", "dependencies", value[1].as_str().unwrap()), output);
    }

    // ...and generate monkey.jungle with all needed data
    let _ = fs::File::create(PathBuf::from("build/tmp/monkey.jungle"));
    let mut monkey_jungle_data = String::new();
    monkey_jungle_data.push_str("project.manifest = manifest.xml\n\n"); // First we will write the base line which specifies the manifest location
    for (entry, value) in dependencies {
        monkey_jungle_data.push_str(&*format!("{} = \"{}\"\n", entry, format!("dependencies/{}", value[1].as_str().unwrap())));
        monkey_jungle_data.push_str(&*format!("base.barrelPath = $(base.barrelPath);$({})\n", entry));
    }
    fs::write(PathBuf::from("build/tmp/monkey.jungle"), monkey_jungle_data);
}

fn transfer_main_resources(resource: String) {
    let mut search_path = PathBuf::from("resources");
    search_path.push(&resource);
    for entry in fs::read_dir(search_path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_type().unwrap().is_file() {
            let mut end_dir = PathBuf::from("build/tmp");
            end_dir.push("resources");
            end_dir.push(&resource);
            end_dir.push(entry.file_name());

            fs::copy(entry.path(), end_dir);
        }
    }
}

fn transfer_device_resources(resource: String, device_specific_res: Vec<String>) {
    for res_entry in device_specific_res {
        let mut res_dir = PathBuf::new();
        res_dir.push("resources");
        res_dir.push(&resource);
        res_dir.push(&res_entry);
        if res_dir.exists() {
            let mut end_dir = PathBuf::from("build/tmp");
            let mut end_dirname = String::from("resources-");
            end_dirname.push_str(&*res_entry);
            end_dir.push(end_dirname);
            end_dir.push(&resource);

            recursive_copy(res_dir, &end_dir);
        }
    }

}
