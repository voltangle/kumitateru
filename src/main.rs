mod prepare_build;
mod utils;
mod verify_project;
pub mod compile_project;
mod ser_de;
mod ciq_sdk;

use colored::Colorize;
use std::{fs, thread, time, process, env};
use verify_project::verify_app_project;
use prepare_build::construct_connectiq_app_project;
use compile_project::compile_app_project;
use clap::{Arg, SubCommand, App};
use std::path::PathBuf;
use std::process::Command;
use std::io;
use serde::Deserialize;
use crate::ser_de::manifest::manifest_utils::generate_ciq_manifest;
use crate::ciq_sdk::CIQSdk;
use anyhow::Context;
use anyhow::Result;
use crate::ser_de::config::app_config::AppConfig;
use regex::Regex;
use crate::utils::arrow_selection::construct_arrow_selection;
use std::str;
use crossterm::{
    event::{
        read,
        Event,
        KeyEvent,
        KeyCode,
        KeyModifiers
    },
    terminal,
    terminal::*,
    ExecutableCommand,
    cursor
};

// These are for checking package type, is it a library or an app
#[derive(Deserialize)]
#[derive(Clone)]
struct AppBarrelCheck {
    package: AppBarrelCheckPackage,
}

#[derive(Deserialize)]
#[derive(Clone)]
struct AppBarrelCheckPackage {
    package_type: String,
}

fn main() -> Result<()> {
    let matches = App::new("Kumitateru")
        .version("0.4.0")
        .author("GGorAA <yegor_yakovenko@icloud.com>")
        .about("A build system for Garmin ConnectIQ.")
        .subcommand(SubCommand::with_name("build")
            .arg(Arg::with_name("target")
                .long("target")
                .value_name("TARGET")
                .help("Specifies custom target.")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("run")
            .arg(Arg::with_name("target")
                .long("target")
                .value_name("TARGET")
                .help("Specifies custom target.")
                .takes_value(true))
        )
        .subcommand(SubCommand::with_name("package"))
        .subcommand(SubCommand::with_name("new"))
        .get_matches();

    match matches.subcommand_name() {
        Some(name) => {
            match name {
                "build" => {
                    let config_str = fs::read_to_string("package.toml").with_context(|| "Unable to read package.toml")?;
                    let config_struct = toml::from_str::<AppConfig>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?;
                    let package_type = toml::from_str::<AppBarrelCheck>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?.package.package_type;
                    if package_type == String::from("app")  {
                        let target = matches.subcommand_matches("run").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?;
                        if !config_struct.package_meta.devices.contains(&target.to_string()) {
                            eprintln!("Bad target specified. Please use one from your package.toml");
                            process::exit(13);
                        }
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("Building the app..."); }
                        let bin_loc = pre_compilation_steps(config_struct.clone()).with_context(|| "Unable to execute pre-compilation steps")?;
                        compile_app_project(
                            PathBuf::from("build/tmp"),
                            PathBuf::from("build/output"),
                            matches.subcommand_matches("build").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?,
                            bin_loc,
                            config_struct).with_context(|| "Failed to build a binary")?;
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{}", "Successfully built!".bold().bright_green()); }
                    } else if package_type == String::from("lib") {
                        if !env::var("KMTR_IDE_SILENT").is_ok() { eprintln!("Kumitateru does not support building libraries(barrels) at the time. Please, replace project_type value with \"app\"."); }
                        process::exit(12); // Exit code 12 indicates that the project config has bad project type
                    } else {
                        if !env::var("KMTR_IDE_SILENT").is_ok() { eprintln!("Bad project type specified. Please, set it to \"app\" and leave it alone."); }
                        process::exit(12); // Exit code 12 indicates that the project config has bad project type
                    }
                }
                "run" => {
                    let config_str = fs::read_to_string("package.toml").with_context(|| "Unable to read package.toml")?;
                    let config_struct = toml::from_str::<AppConfig>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?;
                    let package_type = toml::from_str::<AppBarrelCheck>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?.package.package_type;
                    if package_type == "app" {
                        let target = matches.subcommand_matches("run").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?;
                        if !config_struct.package_meta.devices.contains(&target.to_string()) || target != "all" {
                            eprintln!("Bad target specified. Please use one from your package.toml");
                            process::exit(13);
                        }
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("Running the app..."); }
                        let bin_loc = pre_compilation_steps(config_struct.clone()).with_context(|| "Unable to execute pre-compilation steps")?;
                        compile_app_project(
                            PathBuf::from("build/tmp"),
                            PathBuf::from("build/output"),
                            matches.subcommand_matches("run").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?,
                            bin_loc,
                            config_struct.clone()).with_context(|| "Failed to build a binary")?;
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{} {}", "Step 4:".bold().bright_green(), "Run"); }
                        if env::var("KMTR_IDE_SILENT").is_ok() { println!("\n=== RUN LOGS ===\n"); }
                        let _ = Command::new("connectiq").status()?; // start the simulator
                        thread::sleep(time::Duration::from_millis(2000)); // idk how to fix the race issue when monkeydo is unable to connect to the simulator because it has not started at the time other that like this
                        let _ = Command::new("monkeydo")
                            .args(&[
                                format!("{}{}.prg", "build/output/", config_struct.clone().package_meta.name),
                                matches.subcommand_matches("run").unwrap().value_of("target").unwrap().to_string()
                            ]).status()?;
                    } else {
                        if !env::var("KMTR_IDE_SILENT").is_ok() {
                            eprintln!("{}{}{}{}{}", "Sorry, this project is not an app, it is a".bright_red(), "library".bold().bright_red(), "(barrel). You can't use".bright_red(), "run".bold().bright_red(), "with libraries!".bright_red());
                        }
                        process::exit(12); // Exit code 12 indicates that the project config has bad project type
                    }
                }
                "package" => {
                    let config_str = fs::read_to_string("package.toml").with_context(|| "Unable to read package.toml")?;
                    let config_struct = toml::from_str::<AppConfig>(&*config_str).with_context(|| "Unable to parse package.toml")?;
                    let package_type = toml::from_str::<AppBarrelCheck>(&*config_str).with_context(|| "Unable to parse package.toml")?.package.package_type;
                    if package_type == "app" {
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("Packaging the app..."); }
                        let bin_loc = pre_compilation_steps(config_struct.clone()).with_context(|| "Unable to execute pre-compilation steps")?;
                        compile_app_project(
                            PathBuf::from("build/tmp"),
                            PathBuf::from("build/bin"),
                            "package",
                            bin_loc,
                            config_struct.clone()).with_context(|| "Failed to build a binary")?;
                    } else {
                        if !env::var("KMTR_IDE_SILENT").is_ok() {
                            eprintln!("{}{}{}{}{}", "Sorry, this project is not an app, it is a".bright_red(), "library".bold().bright_red(), "(barrel). You can't use".bright_red(), "run".bold().bright_red(), "with libraries!".bright_red());
                        }
                        process::exit(12); // Exit code 12 indicates that the project config has bad project type
                    }
                }
                "new" => {
                    let mut proj_name = String::new();
                    let mut proj_type: i8;
                    let mut proj_min_sdk = String::new();
                    let mut proj_target_sdk = String::new();
                    println!("{}", "Welcome to Kumitateru new project wizard!".bold());
                    println!("What should we call this project?");
                    io::stdin().read_line(&mut proj_name);
                    {
                        let mut highlighted = 0;
                        // This is a thing to fix issues with resizing of the terminal window.
                        // When the window resizes, a print of arrow selection is done again,
                        // so we need some sort of protection against it. This variable will be
                        // false if the window was just resized, because of that continue; statement
                        // at that piece of code that handles resizing. If no resizing was done,
                        // then it would pass to the end of the loop code and make this variable
                        // true again, making selection text to be show again. I hope this clarifies
                        // what this variable does :D
                        let mut selection_to_show = true;
                        let mut exiting_state = false;
                        loop {
                            if selection_to_show {
                                print!("{}", construct_arrow_selection("Now what type is your app?", vec!(
                                    "App",
                                    "Watchface",
                                    "Datafield",
                                    "Widget",
                                    "Audio content provider"
                                ), highlighted, if exiting_state { true } else { false }));
                                if exiting_state { break }
                            }
                            selection_to_show = false;

                            enable_raw_mode();
                            let event = read()?;
                            match event {
                                Event::Resize(w, h) => {
                                    disable_raw_mode();
                                    continue;
                                }
                                _ => {}
                            }

                            if event == Event::Key(KeyCode::Up.into()) {
                                disable_raw_mode();
                                if highlighted == 0 {
                                    highlighted = 4;
                                } else {
                                    highlighted -= 1;
                                }
                                for _ in 0..6 {
                                    io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine));
                                    io::stdout().execute(cursor::MoveUp(1));
                                }
                            }

                            if event == Event::Key(KeyCode::Down.into()) {
                                disable_raw_mode();
                                if highlighted == 4 {
                                    highlighted = 0;
                                } else {
                                    highlighted += 1;
                                }
                                for _ in 0..6 {
                                    io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine));
                                    io::stdout().execute(cursor::MoveUp(1));
                                }
                            }

                            if event == Event::Key(KeyEvent { modifiers: KeyModifiers::CONTROL, code: KeyCode::Char('c') }) {
                                disable_raw_mode();
                                process::exit(1);
                            }

                            if event == Event::Key(KeyCode::Enter.into()) {
                                disable_raw_mode();
                                proj_type = highlighted;
                                exiting_state = true;
                                for _ in 0..6 {
                                    io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine));
                                    io::stdout().execute(cursor::MoveUp(1));
                                }
                            }
                            selection_to_show = true;
                        }
                    }
                    proj_min_sdk = get_version(VersionType::MinSDK);
                    proj_target_sdk = get_version(VersionType::TargetSDK);
                }
                &_ => {}
            }
        }
        None => {
            println!("{}", matches.usage());
        }
    }
    disable_raw_mode();
    Ok(())
}

fn get_version(ver_type: VersionType) -> String {
    match ver_type {
        VersionType::MinSDK => {
            println!("What minimum SDK will your app support?");
        }
        VersionType::TargetSDK => {
            println!("What SDK will your app target?");
        }
    }
    let version_regex = Regex::new(r#"[0-9]+\.[0-9]+\.[0-9]+(\.[0-9a-zA-Z_]+)?"#).unwrap();
    let mut version = String::new();
    io::stdin().read_line(&mut version);
    if !version_regex.is_match(&*version) {
        println!("{}", "Bad version format. Please try again!".bright_red());
        get_version(ver_type);
    }
    return version
}

fn pre_compilation_steps(config: AppConfig) -> Result<PathBuf> {
    let bin_loc = CIQSdk::bin_location(&*config.package.target_sdk)?;
    if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{} {}", "Step 1:".bold().bright_green(), "Verify project structure"); }
    verify_app_project().with_context(|| "Failed to verify project")?;
    if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{} {}", "Step 2:".bold().bright_green(), "Assemble a ConnectIQ Project"); }
    construct_connectiq_app_project(
        generate_ciq_manifest(config.clone()).with_context(|| "Unable to generate manifest.xml")?,
        config.clone().dependencies
    ).with_context(|| "Failed to construct a ConnectIQ project")?;
    if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{}", "Successfully assembled!".bold().bright_green()); }
    if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{} {}", "Step 3:".bold().bright_green(), "Compile the app"); }
    Ok(bin_loc)
}

enum VersionType {
    MinSDK,
    TargetSDK,
}
