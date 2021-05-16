mod prepare_build;
mod utils;
mod verify_project;

use switch_statement;
use colored::Colorize;
use crate::verify_project::verify_project;
use crate::prepare_build::construct_connectiq_project;

const BUILD_COMMAND: &str = "build";

fn main() {
    let command: String;
    match std::env::args().nth(1) {
        None => {
            eprintln!("{}", "No command was passed to GarBuild. Exiting...".bright_red().bold());
            std::process::exit(1);
        }
        Some(_command) => {
            command = _command;
        }
    }

    // Check what command was in here
    switch_statement::switch! { command;
        BUILD_COMMAND => {
            println!("Building project...");
            println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure");
            verify_project();
            println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
            construct_connectiq_project();
        },
        _ => println!("No command found."),
    }
}

