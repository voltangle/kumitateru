![](https://github.com/ggoraa/kumitateru/actions/workflows/rust.yml/badge.svg)
# Kumitateru
Kumitateru is a build system for Garmin ConnectIQ, written in Rust.

Currently in development, so you can't use it.

## Setup build config
Build config sits in the root of the project, and named `kumitateru.toml`. It looks like this:

```toml
[package]
name = "MyApp"
name_res = "@Strings.AppName"
main_class = "MyAppMainClass"
app_type = "watch-app"
min_sdk = "1.2.0"

[package-meta]
id = "app-id"
devices = [
    "fenix6x"
]
permissions = [
    "Communications"
]
languages = [
    "eng"
]

[build]
version = "1.0.0"
icon_resource = "@Drawables.LauncherIcon"
signing_key = "id_garmin_sign.der"

# This block does not affect anything, it is just there for the future
[dependencies]
"simple-barrel" = "0.1.0"
```

For now Kumitateru does not support external dependencies(or barrels, as Garmin calls them),
but I will figure out later how to add them, specifically when i will learn the full structure
of Connect IQ manifest and monkey.jungle. 
