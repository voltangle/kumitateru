# Kumitateru
Kumitateru is a build system for Garmin ConnectIQ, written in Rust.

Currently in development, so you can't use it.

## Setup build config
Build config sits in the root of the project, and named `kumitateru.toml`. It looks like this:

```toml
[package]
name = "MyApp"
main_class = "MyAppMainClass"
app_type = "watch-app"
min_sdk = "1.2.0"

[package-meta]
id = "app-id"
version = "1.0.0"
icon = "@Drawables.LauncherIcon"
devices = [
    "fr945",
    "fenix3",
    "fenix3_hr",
    "fenix5",
    "fenix5plus",
    "fenix5s",
    "fenix5splus",
    "fenix6",
    "fenix6pro",
    "fenix6s",
    "fenix6spro",
    "fenix6xpro",
    "fenixchronos",
    "vivoactive3",
    "vivoactive3d",
    "vivoactive3m",
    "vivoactive3mlte",
    "vivoactive4",
    "vivoactive4s",
]
permissions = [
    "Communications"
]
languages = [
    "eng",
    "rus",
]

```

For now Kumitateru does not support external dependencies(or barrels, as Garmin calls them),
but I will figure out later how to add them, specifically when i will learn the full structure
of Connect IQ manifest and monkey.jungle. 
