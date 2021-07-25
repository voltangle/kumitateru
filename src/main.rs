mod prepare_build;
mod utils;
mod verify_project;
pub mod compile_project;
mod ser_de;
mod ciq_sdk;

use colored::Colorize;
use std::{fs, thread, time, process};
use verify_project::verify_app_project;
use prepare_build::construct_connectiq_app_project;
use compile_project::compile_app_project;
use clap::{Arg, SubCommand, App};
use std::path::PathBuf;
use std::process::Command;
use ser_de::parse_config::parse_config;
use serde::Deserialize;
use crate::ser_de::manifest::manifest_utils::generate_ciq_manifest;
use crate::ser_de::config::app_config::AppConfig;
use crate::ciq_sdk::CIQSdk;

// These are for checking package type, is it a library or an app
#[derive(Deserialize)]
#[derive(Clone)]
struct AppBarrelCheck {
    package: AppConfigPackage,
}

#[derive(Deserialize)]
#[derive(Clone)]
struct AppConfigPackage {
    package_type: String,
}

fn main() {
    let matches = App::new("Kumitateru")
        .version("0.2.0")
        .author("GGorAA <yegor_yakovenko@icloud.com>")
        .about("A build system for Garmin ConnectIQ.")
        .arg(Arg::with_name("version")
            .value_name("version")
            .help("Get version")
            .takes_value(false))
        .subcommand(SubCommand::with_name("build")
            .arg(Arg::with_name("target")
                .long("target")
                .value_name("TARGET")
                .help("Specifies custom target.")
                .default_value("package")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("run")
            .arg(Arg::with_name("target")
                .long("target")
                .value_name("TARGET")
                .help("Specifies custom target.")
                .default_value("package")
                .takes_value(true))
        )
        .get_matches();


    match matches.subcommand_name() {
        Some(name) => {
            match name {
                "build" => {
                    let package_type = toml::from_str::<AppBarrelCheck>(&*fs::read_to_string("kumitateru.toml").unwrap()).unwrap().package.package_type;
                    if package_type == "app"  {
                        println!("Building the app...");
                        let bin_loc = CIQSdk::bin_location(&*toml::from_str::<AppConfig>(&*fs::read_to_string("kumitateru.toml").unwrap()).unwrap().package.target_sdk);
                        println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure");
                        verify_app_project();
                        println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
                        construct_connectiq_app_project(
                            generate_ciq_manifest(fs::read_to_string("kumitateru.toml").unwrap()),
                            toml::from_str::<AppConfig>(&*fs::read_to_string("kumitateru.toml").unwrap()).unwrap().dependencies
                        );
                        println!("{}", "Successfully assembled!".bold().bright_green());
                        println!("{} {}", "Step 3:".bold().bright_green(), "Compile the app");
                        compile_app_project(
                            PathBuf::from("build/tmp"),
                            PathBuf::from("build/output"),
                            matches.subcommand_matches("build").unwrap().value_of("target").unwrap(),
                            bin_loc);
                        println!("{}", "Successfully built!".bold().bright_green());
                    } else if package_type == "lib" {
                        eprintln!("Kumitateru does not support building libraries(barrels) at the time. Please, replace project_type value with \"app\".");
                        process::exit(12); // Exit code 12 indicates that the project config was badly configured
                    } else {
                        eprintln!("Bad project type specified. Please, set it to \"app\" and leave it alone.");
                        process::exit(12); // Exit code 12 indicates that the project config was badly configured
                    }
                }
                "run" => {
                    if toml::from_str::<AppBarrelCheck>(&*fs::read_to_string("kumitateru.toml").unwrap()).unwrap().package.package_type == "app" {
                        println!("Running the app...");
                        let bin_loc = CIQSdk::bin_location(&*toml::from_str::<AppConfig>(&*fs::read_to_string("kumitateru.toml").unwrap()).unwrap().package.target_sdk);
                        println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure");
                        verify_app_project();
                        println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
                        construct_connectiq_app_project(
                            generate_ciq_manifest(fs::read_to_string("kumitateru.toml").unwrap()),
                            toml::from_str::<AppConfig>(&*fs::read_to_string("kumitateru.toml").unwrap()).unwrap().dependencies
                        );
                        println!("{}", "Successfully assembled!".bold().bright_green());
                        println!("{} {}", "Step 3:".bold().bright_green(), "Compile the app");
                        compile_app_project(
                            PathBuf::from("build/tmp"),
                            PathBuf::from("build/output"),
                            matches.subcommand_matches("run").unwrap().value_of("target").unwrap(),
                            bin_loc);
                        println!("{} {}", "Step 4:".bold().bright_green(), "Run");
                        let _ = Command::new("connectiq").status().unwrap(); // start the simulator
                        thread::sleep(time::Duration::from_millis(2000));
                        let _ = Command::new("monkeydo")
                            .args(&[
                                format!("{}{}.prg", "build/output/", parse_config(fs::read_to_string("kumitateru.toml").unwrap()).package_meta.name),
                                matches.subcommand_matches("run").unwrap().value_of("target").unwrap().to_string()
                            ]).status().unwrap();
                    } else {
                        eprintln!("{}{}{}{}{}", "Sorry, this project is not an app, it is a".bright_red(), "library".bold().bright_red(), "(barrel). You can't use".bright_red(), "run".bold().bright_red(), "with libraries!".bright_red());
                        process::exit(12); // Exit code 12 indicates that the project config was badly configured

                    }
                }
                &_ => {}
            }
        }
        None => {
            println!("{}", matches.usage());
        }
    }
}

