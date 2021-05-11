mod prepare_build;

use switch_statement;

const BUILD_COMMAND: &str = "build";

fn main() {
    println!("GarBuild");

    let command = std::env::args().nth(1).expect("command");

    // Check what command was in here
    switch_statement::switch! { command;
        BUILD_COMMAND => {
            println!("Building project...");
        },
        _ => println!("No command found."),
    }
}
