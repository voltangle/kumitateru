mod prepare_build;
mod utils;
mod verify_project;
pub mod compile_project;

use colored::Colorize;
use std::{fs, thread, time};
use crate::verify_project::verify_project;
use crate::prepare_build::construct_connectiq_project;
use crate::utils::manifest_utils::generate_ciq_manifest;
use crate::compile_project::compile_project;
use clap::{Arg, SubCommand, App};
use std::path::PathBuf;
use std::process::Command;
use crate::utils::config::parse_config;

fn main() {
    let matches = App::new("Kumitateru")
        .version("1.0.0")
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
                    println!("Building project...");
                    println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure");
                    verify_project();
                    println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
                    construct_connectiq_project(
                        generate_ciq_manifest(fs::read_to_string("kumitateru.toml").unwrap())
                    );
                    println!("{}", "Successfully assembled!".bold().bright_green());
                    println!("{} {}", "Step 3:".bold().bright_green(), "Compile the app");
                    compile_project(
                        PathBuf::from("build/tmp"),
                        PathBuf::from("build/output"),
                        matches.subcommand_matches("build").unwrap().value_of("target").unwrap());
                }
                "run" => {
                    println!("Building project...");
                    println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure");
                    verify_project();
                    println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
                    construct_connectiq_project(
                        generate_ciq_manifest(fs::read_to_string("kumitateru.toml").unwrap())
                    );
                    println!("{}", "Successfully assembled!".bold().bright_green());
                    println!("{} {}", "Step 3:".bold().bright_green(), "Compile the app");
                    compile_project(
                        PathBuf::from("build/tmp"),
                        PathBuf::from("build/output"),
                        matches.subcommand_matches("run").unwrap().value_of("target").unwrap());
                    println!("{} {}", "Step 4:".bold().bright_green(), "Run");
                    let _ = Command::new("connectiq").status().unwrap();
                    thread::sleep(time::Duration::from_millis(2000));
                    let _ = Command::new("monkeydo")
                        .args(&[
                            format!("{}{}.prg","build/output/", parse_config(fs::read_to_string("kumitateru.toml").unwrap()).package_meta.name),
                            matches.subcommand_matches("run").unwrap().value_of("target").unwrap().to_string()
                        ]).status().unwrap();
                }
                &_ => {}
            }
        }
        None => {
            println!("{}", matches.usage());
        }
    }
}

