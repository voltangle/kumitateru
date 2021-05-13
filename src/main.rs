mod prepare_build;
mod utils;

use switch_statement;
use prepare_build::construct_connectiq_project;
use colored::Colorize;

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
            construct_connectiq_project();
            println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project");
        },
        _ => println!("No command found."),
    }
}

