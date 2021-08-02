![](https://github.com/ggoraa/kumitateru/actions/workflows/build.yaml/badge.svg)
# Kumitateru
Kumitateru is a build system for Garmin ConnectIQ, written in Rust.

It is ALPHA software, be aware

## Project structure

The basic directory structure looks like this:
```
├── id_rsa_garmin.der
├── kumitateru
├── package.toml
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
Build config sits in the root of the project, and named `package.toml`. It looks like this:

```toml
[package]
name_res = "@Strings.AppName"
icon_resource = "@Drawables.LauncherIcon"
main_class = "MyApp"
# Currently here for the future, exactly for supporting libraries(barrels) as projects, not only apps.
package_type = "app"
app_type = "watch-app"
min_sdk = "1.2.0"
# This property is for setting the SDK, which will be used to build the app
target_sdk = "4.0.4"

[package_meta]
name = "MyApp"
id = "app-id"
version = "1.0.0"
devices = ["fenix6xpro"]
permissions = ["Background"]
languages = ["eng"]

[build]
signing_key = "id_garmin_sign.der"
# This will be, again, for future. Will be used for code analysis, like possible places of crash, bad design, and much more
code_analysis_on_build = false
type_check_level = 0 # Type checking, which was introduced in CIQ 4.0.0. Levels: 0: disable, 1: gradual, 2: informative, 3: strict
compiler_args = "" # If you want some custom parameters, place them here

[dependencies]
"simple-barrel" = ["0.1.0", "simple-barrel-0.1.0.barrel"] # The second entry is a path to the barrel inside of dependencies folder.
```

For now Kumitateru does not support external dependencies(or barrels, as Garmin calls them),
but I will figure out later how to add them, specifically when i will learn the full structure
of Connect IQ manifest and monkey.jungle. 

## Using the build system
Basic build command is `./kumitateru build`, which compiles the app in .iq format. To build
for a specific device, run `./kumitateru build --target <device>`. To build for all devices
in .prg, you run `./kumitateru build --target all`.
