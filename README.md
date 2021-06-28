![](https://github.com/ggoraa/kumitateru/actions/workflows/rust.yml/badge.svg)
# Kumitateru
Kumitateru is a build system for Garmin ConnectIQ, written in Rust.

It is ALPHA software, be aware

## Project structure

The basic directory structure looks like this:
```
├── id_rsa_garmin.der
├── kumitateru
├── kumitateru.toml
├── resources
│   ├── drawables
│   │   ├── drawables.xml
│   │   └── launcher_icon.png
│   ├── fonts
│   │   └── fenix5
│   ├── layouts
│   ├── menus
│   ├── settings
│   └── strings
│       └── main
│           └── strings.xml
└── src
```

## Build config
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

## Using the build system
Basic build command is `./kumitateru build`, which compiles the app in .iq format. To build
for a specific device, run `./kumitateru build --target <device>`. To build for all devices
in .prg, you run `./kumitateru build --target all`.
