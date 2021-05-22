use std::path::PathBuf;
use std::process::Command;
use crate::utils::config::parse_config;
use std::fs;

pub fn compile_project(project: PathBuf, output: PathBuf) {
    let mut jungle_path = project;
    jungle_path.push("monkey.jungle");
    let mut output_path = output;
    let parsed_config = parse_config(fs::read_to_string("kumitateru.toml").unwrap());
    output_path.push(parsed_config.package.name);
    println!("{:?}", ["monkeyc", "--jungles", jungle_path.to_str().unwrap(),
        "--package-app",
        "--release",
        "--output", output_path.to_str().unwrap(),
        "--private-key", &parsed_config.build.signing_key,
        "--warn"]);
    Command::new("monkeyc")
        .args(&[
            "--jungles", jungle_path.to_str().unwrap(),
            "--package-app",
            "--release",
            "--output", output_path.to_str().unwrap(),
            "--private-key", &parsed_config.build.signing_key,
            "--warn"
        ])
        .output()
        .expect("Failed to run monkeyc.");
}
