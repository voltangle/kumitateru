use colored::Colorize;
use std::fs;
use std::path::Path;

/// This function gathers all files from resources and
/// src directories, and transfers them in build/proj,
/// where it will be built by monkeyc.
pub fn construct_connectiq_project() -> Result<bool, std::io::Error> {
    fs::create_dir("build");
    fs::create_dir("build/tmp");
    fs::create_dir("build/tmp/source");
    fs::create_dir("build/tmp/resources");

    println!("{}", "Transferring basic files and source code...".bold());

    fs::copy("manifest.xml", "build/tmp/manifest.xml")?;
    Ok(true)
}