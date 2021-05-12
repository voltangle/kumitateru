mod prepare_build;

use switch_statement;
use prepare_build::construct_connectiq_project;

const BUILD_COMMAND: &str = "build";

fn main() {
    let command = std::env::args().nth(1).expect("command");

    // Check what command was in here
    switch_statement::switch! { command;
        BUILD_COMMAND => {
            println!("Building project...");
            println!("Step 1: Assemble a Connect IQ Project");
            construct_connectiq_project();
        },
        _ => println!("No command found."),
    }
}

