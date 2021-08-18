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
use crate::ser_de::config::app_config::{AppConfig, AppConfigPackage, AppConfigPackageMeta, AppConfigBuild};
use regex::Regex;
use utils::tui::item_selection::display_cli_item_selection;
use crossterm::terminal::disable_raw_mode;
use uuid::Uuid;
use heck::CamelCase;
use crate::utils::fs_utils::FsUtils;

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
        .subcommand(SubCommand::with_name("clean"))
        .get_matches();

    match matches.subcommand_name() {
        Some(name) => {
            match name {
                "build" => {
                    let config_str = fs::read_to_string(FsUtils::workdir(Some(PathBuf::from("package.toml")))?).with_context(|| "Unable to read package.toml")?;
                    let config_struct = toml::from_str::<AppConfig>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?;
                    let package_type = toml::from_str::<AppBarrelCheck>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?.package.package_type;
                    if package_type == String::from("app")  {
                        let target = matches.subcommand_matches("build").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?;
                        if !config_struct.package_meta.devices.contains(&target.to_string()) {
                            eprintln!("Bad target specified. Please use one from your package.toml");
                            process::exit(13);
                        }
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("Building the app..."); }
                        let bin_loc = pre_compilation_steps(config_struct.clone()).with_context(|| "Unable to execute pre-compilation steps")?;
                        compile_app_project(
                            FsUtils::workdir(Some(PathBuf::from("build/tmp")))?,
                            FsUtils::workdir(Some(PathBuf::from("build/bin")))?,
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
                    let config_str = fs::read_to_string(FsUtils::workdir(Some(PathBuf::from("package.toml")))?).with_context(|| "Unable to read package.toml")?;
                    let config_struct = toml::from_str::<AppConfig>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?;
                    let package_type = toml::from_str::<AppBarrelCheck>(&*config_str.clone()).with_context(|| "Unable to parse package.toml")?.package.package_type;
                    if package_type == String::from("app")  {
                        let target = matches.subcommand_matches("run").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?;
                        if !config_struct.package_meta.devices.contains(&target.to_string()) {
                            eprintln!("Bad target specified. Please use one from your package.toml");
                            process::exit(13);
                        }
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("Running the app..."); }
                        let bin_loc = pre_compilation_steps(config_struct.clone()).with_context(|| "Unable to execute pre-compilation steps")?;
                        compile_app_project(
                            FsUtils::workdir(Some(PathBuf::from("build/tmp")))?,
                            FsUtils::workdir(Some(PathBuf::from("build/bin")))?,
                            matches.subcommand_matches("run").unwrap().value_of("target").with_context(|| "Argument --target/-t was not specified")?,
                            bin_loc,
                            config_struct.clone()).with_context(|| "Failed to build a binary")?;
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("{} {}", "Step 4:".bold().bright_green(), "Run"); }
                        if env::var("KMTR_IDE_SILENT").is_ok() { println!("\n=== RUN LOGS ===\n"); }
                        let _ = Command::new("connectiq").status()?; // start the simulator
                        thread::sleep(time::Duration::from_millis(2000)); // idk how to fix the race issue when monkeydo is unable to connect to the simulator because it has not started at the time other that like this
                        let _ = Command::new("monkeydo")
                            .args(&[
                                format!("{}{}{}.prg", FsUtils::workdir(None)?.display(), "/build/bin/", config_struct.clone().package_meta.name),
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
                    let config_str = fs::read_to_string(FsUtils::workdir(Some(PathBuf::from("package.toml")))?).with_context(|| "Unable to read package.toml")?;
                    let config_struct = toml::from_str::<AppConfig>(&*config_str).with_context(|| "Unable to parse package.toml")?;
                    let package_type = toml::from_str::<AppBarrelCheck>(&*config_str).with_context(|| "Unable to parse package.toml")?.package.package_type;
                    if package_type == "app" {
                        if !env::var("KMTR_IDE_SILENT").is_ok() { println!("Packaging the app..."); }
                        let bin_loc = pre_compilation_steps(config_struct.clone()).with_context(|| "Unable to execute pre-compilation steps")?;
                        compile_app_project(
                            FsUtils::workdir(Some(PathBuf::from("build/tmp")))?,
                            FsUtils::workdir(Some(PathBuf::from("build/bin")))?,
                            "package",
                            bin_loc,
                            config_struct.clone()
                        ).with_context(|| "Failed to build a binary")?;
                    } else {
                        if !env::var("KMTR_IDE_SILENT").is_ok() {
                            eprintln!("{}{}{}{}{}", "Sorry, this project is not an app, it is a".bright_red(), "library".bold().bright_red(), "(barrel). You can't use".bright_red(), "run".bold().bright_red(), "with libraries!".bright_red());
                        }
                        process::exit(12); // Exit code 12 indicates that the project config has bad project type
                    }
                }
                "new" => {
                    let mut proj_name = String::new();
                    let proj_type: String;
                    let proj_min_sdk: String;
                    let proj_target_sdk: String;
                    let mut proj_signing_key: Option<PathBuf> = None; // If none, then a new key should be generated. If some, then it will be imported
                    println!("{}", "Welcome to Kumitateru new project wizard!".bold());
                    println!("What should we call this project?");
                    io::stdin().read_line(&mut proj_name);
                    proj_type = vec!(
                        "watch-app",
                        "watchface",
                        "datafield",
                        "widget",
                        "audio-content-provider"
                    )[display_cli_item_selection(
                        "Now what type is your app?",
                        vec!(
                            "App",
                            "Watchface",
                            "Datafield",
                            "Widget",
                            "Audio content provider"
                        ))? as usize].to_string();

                    proj_min_sdk = get_version(VersionType::MinSDK);

                    proj_target_sdk = get_version(VersionType::TargetSDK);

                    if display_cli_item_selection("Generate a new signing key or import one?", vec!("Generate a new key", "Import an existing key"))? == 1 {
                        println!("Please type the path to your key:");
                        let mut path = String::new();
                        io::stdin().read_line(&mut path)?;
                        proj_signing_key = Some(PathBuf::from(path));
                    }

                    if proj_signing_key == None {
                        Command::new("openssl").args([
                            "genrsa",
                            "-out", "id_rsa_garmin.pem",
                            "4096"
                        ]).status()?;
                        Command::new("openssl").args([
                            "pkcs8", "-topk8",
                            "-inform", "PEM",
                            "-outform", "DER",
                            "-in", "id_rsa_garmin.pem",
                            "-out", "id_rsa_garmin.der",
                            "-nocrypt"
                        ]).status()?;
                        fs::remove_file("id_rsa_garmin.pem")?;
                    }
                    let mut main_class = proj_name.to_string();
                    main_class.push_str("App");
                    println!("{}", main_class);

                    let toml_config = AppConfig {
                        package: AppConfigPackage {
                            icon_resource: "".to_string(),
                            name_res: "".to_string(),
                            main_class: main_class.to_camel_case(),
                            app_type: proj_type,
                            min_sdk: proj_min_sdk[0..proj_min_sdk.len() - 1].to_string(),
                            target_sdk: proj_target_sdk[0..proj_target_sdk.len() - 1].to_string()
                        },
                        package_meta: AppConfigPackageMeta {
                            name: proj_name[0..proj_name.len() - 1].to_string(),
                            id: Uuid::new_v4().to_string(),
                            version: "0.1.0".to_string(),
                            devices: vec![],
                            permissions: vec![],
                            languages: vec!["eng".to_string()]
                        },
                        build: AppConfigBuild {
                            signing_key: "id_rsa_garmin.der".to_string(),
                            type_check_level: 0,
                            compiler_args: "".to_string()
                        },
                        dependencies: Default::default()
                    };
                    println!("{:#?}", toml_config);
                }
                "clean" => {
                    fs::remove_dir_all(FsUtils::workdir(Some(PathBuf::from("build")))?).with_context(|| "Unable to clear build directory")?;
                }
                &_ => {}
            }
        }
        None => {
            println!("{}", matches.usage());
        }
    }
    disable_raw_mode()?; // Just in case
    Ok(())
}

fn get_version(ver_type: VersionType) -> String {
    match ver_type {
        VersionType::MinSDK => {
            println!("\nWhat minimum SDK will your app support?");
        }
        VersionType::TargetSDK => {
            println!("\nWhat SDK will your app target?");
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
