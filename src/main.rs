mod prepare_build;
mod utils;
mod verify_project;

use colored::Colorize;
use std::fs;
use crate::verify_project::verify_project;
use crate::prepare_build::construct_connectiq_project;
use crate::utils::manifest_utils::generate_ciq_manifest;
use clap::{Arg, SubCommand, App};


fn main() {
    let matches = App::new("Kumitateru")
        .version("0.1.0")
        .author("GGorAA <yegor_yakovenko@icloud.com>")
        .about("A build system for Garmin ConnectIQ.")
        .arg(Arg::with_name("version")
            .value_name("version")
            .help("Get version")
            .takes_value(false))
        .subcommand(SubCommand::with_name("build"))
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
                }
                &_ => {}
            }
        }
        None => {
            println!("{}", matches.usage());
        }
    }
}

