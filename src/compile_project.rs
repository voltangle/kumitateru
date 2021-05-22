use std::path::PathBuf;
use std::process::Command;
use crate::utils::config::parse_config;
use std::fs;

pub fn compile_project(project: PathBuf, output: PathBuf, target: &str) {
    let mut jungle_path = project;
    jungle_path.push("monkey.jungle");
    let mut output_path = output;
    let parsed_config = parse_config(fs::read_to_string("kumitateru.toml").unwrap());
    output_path.push(parsed_config.package.name);


    if target == "package" {
        let output = Command::new("monkeyc")
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
        println!("{}", String::from_utf8_lossy(&output.stderr))
    } else if target == "all" {
        for device in parsed_config.package_meta.devices {
            let output = Command::new("monkeyc")
                .args(&[
                    "--jungles", jungle_path.to_str().unwrap(),
                    "--device", &device,
                    "--output", output_path.to_str().unwrap(),
                    "--private-key", &parsed_config.build.signing_key,
                    "--warn"
                ])
                .output()
                .expect("Failed to run monkeyc.");
            println!("{}", String::from_utf8_lossy(&output.stderr))
        }
    } else {
        let output = Command::new("monkeyc")
            .args(&[
                "--jungles", jungle_path.to_str().unwrap(),
                "--device", target,
                "--output", output_path.to_str().unwrap(),
                "--private-key", &parsed_config.build.signing_key,
                "--warn"
            ])
            .output()
            .expect("Failed to run monkeyc.");
        println!("{}", String::from_utf8_lossy(&output.stderr))
    }
}

// monkeyc \
// --jungles ./monkey.jungle \
// --device $(DEVICE) \
// --output bin/$(appName).prg \
// --private-key $(PRIVATE_KEY) \
// --warn
