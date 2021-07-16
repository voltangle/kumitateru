use std::path::PathBuf;
use std::process::Command;
use crate::ser_de::parse_config::parse_config;
use std::fs;

pub fn compile_app_project(project: PathBuf, output: PathBuf, target: &str) {
    let mut jungle_path = project;
    jungle_path.push("monkey.jungle");
    let mut output_path = output;
    let parsed_config = parse_config(fs::read_to_string("kumitateru.toml").unwrap());

    if target == "package" {
        output_path.push(parsed_config.package_meta.name);
        output_path.set_extension("iq");
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
            let mut output_path = output_path.clone();
            let mut app_name = String::from(parsed_config.package_meta.name.clone());
            app_name.push_str("-");
            app_name.push_str(&*device);
            output_path.push(app_name);
            output_path.set_extension("prg");
            let _ = Command::new("monkeyc")
                .args(&[
                    "--jungles", jungle_path.to_str().unwrap(),
                    "--device", &device,
                    "--output", output_path.to_str().unwrap(),
                    "--private-key", &parsed_config.build.signing_key,
                    "--warn"
                ]).spawn().unwrap();
        }
    } else {
        output_path.push(parsed_config.package_meta.name);
        output_path.set_extension("prg");
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
