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
    "fenix6x"
]
permissions = [
    "Communications"
]
languages = [
    "eng"
]

# This block does not affect anything, it is just there for the future
[dependencies]
"simple-barrel" = "0.1.0"
```

For now Kumitateru does not support external dependencies(or barrels, as Garmin calls them),
but I will figure out later how to add them, specifically when i will learn the full structure
of Connect IQ manifest and monkey.jungle. 
