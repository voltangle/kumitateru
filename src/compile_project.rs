use std::path::PathBuf;
use std::process::Command;
use crate::ser_de::parse_config::parse_config;
use std::{fs, process, env};
use anyhow::{Result, Context};

pub fn compile_app_project(project: PathBuf, output: PathBuf, target: &str, compiler: PathBuf) -> Result<()> {
    let mut jungle_path = project;
    jungle_path.push("monkey.jungle");
    let mut output_path = output;
    let parsed_config = parse_config(fs::read_to_string("package.toml").with_context(|| "Unable to read package.toml")?);

    match target {
        "package" => {
            output_path.push(parsed_config.package_meta.name);
            output_path.set_extension("iq");
            let compiler = compiler.to_str().unwrap();
            let monkeybrains_path: &str = &format!("{}/{}", compiler, "monkeybrains.jar");
            let mut command = Command::new("java");
            command.args([
                    "-Xms768m",
                    "-Dfile.encoding=UTF-8",
                    "-Dapple.awt.UIElement=true",
                    "-jar", monkeybrains_path,
                    "-o",output_path.to_str().unwrap(),
                    "--package-app",
                    "--release",
                    "-f", jungle_path.to_str().unwrap(),
                    "-y", &parsed_config.build.signing_key,
                    "-l", &parsed_config.build.type_check_level.to_string(),
                    "--warn"
                ]);
            println!("{}", String::from_utf8_lossy(&*command.output().expect("Failed to run Monkey C compiler.").stderr));
            if !command.status().unwrap().success() {
                if !env::var("KMTR_IDE_SILENT").is_ok() { eprintln!("Build failed."); }
                process::exit(30); // code 30 signifies that the compiler was not able to build the project
            }
        }
        "all" => {
            for device in parsed_config.package_meta.devices {
                let mut output_path = output_path.clone();
                let mut app_name = String::from(parsed_config.package_meta.name.clone());
                app_name.push_str("-");
                app_name.push_str(&*device);
                output_path.push(app_name);
                output_path.set_extension("prg");
                let compiler = compiler.to_str().unwrap();
                let monkeybrains_path: &str = &format!("{}/{}", compiler, "monkeybrains.jar");
                println!("{}", String::from_utf8_lossy(&Command::new("java")
                    .args([
                        "-Xms768m",
                        "-Dfile.encoding=UTF-8",
                        "-Dapple.awt.UIElement=true",
                        "-jar", monkeybrains_path,
                        "-o",output_path.to_str().unwrap(),
                        "--device", target,
                        "-f", jungle_path.to_str().unwrap(),
                        "-y", &parsed_config.build.signing_key,
                        "-l", &parsed_config.build.type_check_level.to_string(),
                        "--warn"
                    ]).output().expect("Failed to run Monkey C compiler.").stderr))
            }
        }
        _ => {
            output_path.push(parsed_config.package_meta.name);
            output_path.set_extension("prg");
            let compiler = compiler.to_str().unwrap();
            let monkeybrains_path: &str = &format!("{}/{}", compiler, "monkeybrains.jar");
            let mut command = Command::new("java").args([
                "-Xms768m",
                "-Dfile.encoding=UTF-8",
                "-Dapple.awt.UIElement=true",
                "-jar", monkeybrains_path,
                "-o",output_path.to_str().unwrap(),
                "--device", target,
                "-f", jungle_path.to_str().unwrap(),
                "-y", &parsed_config.build.signing_key,
                "-l", &parsed_config.build.type_check_level.to_string(),
                "--warn"
            ]).spawn().expect("Failed to run Monkey C compiler.");
            if !command.wait().unwrap().success() {
                if !env::var("KMTR_IDE_SILENT").is_ok() { eprintln!("Build failed."); }
                process::exit(30); // code 30 signifies that the compiler was not able to build the project
            }
        }
    }
    Ok(())
}

// java -jar monkeybrains.jar -o wl.prg -f /monkey.jungle -y /id_rsa_garmin.der -l 1

