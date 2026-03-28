# Rust Setup

The setup is nicely explained in the [Rust on ESP book](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html) chapter **getting-started**.

Install cross compiler target

For ESP32 there are two micros, Xtensa and RISCV. The newer variants come with
RISCV. To compile for these target a cross compiler is needed, this is `Espup`.
About [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html).

> The old ESP32-TTGO uses Xtensa.

Some packages can be installed with `cargo-binstall`

```sh
brew install cargo-binstall
```

## Espup

Check if there are changes in the installation:
Visit <https://github.com/esp-rs/espup>

check if the following packages are installed with `cargo install --list`

```sh
cargo install ldproxy
```

### Installation

```sh
cargo install espup --locked
espup install
```

**Environment variables**

`espup` requires environment variables from `~/export-esp.sh`

Copy the content of `HOME/export-esp.sh` to the terminal configuration,
example `.zshrc`

```sh
cargo binstall esp-generate
cargo binstall esp-flash
cargo binstall probe-rs-tools
```

Read more about [Probe-rs](https://probe.rs/docs/library/quickstart/).

- checkpoint

```sh
xtensa-esp32-elf-gcc --version
xtensa-esp-elf-gcc (crosstool-NG esp-15.2.0_20250920) 15.2.0
```

## Crates

Repositories naming conventions

- `esp-*` are focused on `no_std` approach
- `esp-idf-*` are focused on `std` approach

`esp-idf-sys` access to drivers, Wi-Fi, etc.
`embedded-svc` abstraction for embedded services (WiFi, Network, Httpd, Logging, etc.).
`esp-idf-hal` implementation of the `embedded-hal`.
`esp-idf-svc` implementation of `embedded-svc`.

### Stack

1. `esp-idf-svc` (calls all below)
2. `esp-idf-hal` (calls all below)
3. `esp-idf-sys` & `embedded-svc`

[reference](https://docs.esp-rs.org/book/overview/using-the-standard-library.html#relevant-esp-rs-crates)

### Graphic library

[embedded-graphics] (<https://github.com/embedded-graphics/embedded-graphics>)
[mipidsi](https://github.com/almindor/mipidsi)
See [Troubleshooting](https://github.com/almindor/mipidsi/blob/master/docs/TROUBLESHOOTING.md)
wrong color and inversion

## Example: Building using the template generator

The template generator will fetch the template project from github and guide
you through the basic options to create your project.

Here is an example to create a project for ESP32 (old HW version) with STD.

```sh
cargo generate esp-rs/esp-idf-template cargo

⚠️   Favorite `esp-rs/esp-idf-template` not found in config, using it as a git
repository: <https://github.com/esp-rs/esp-idf-template.git>
🤷   Project Name: hello_world
🔧   Destination: /Users/mac/src/github/esp32/ttgo-display/rust/hello_world ...
🔧   project-name: hello_world ...
🔧   Generating template ...
✔ 🤷   Which MCU to target? · esp32
✔ 🤷   Configure advanced template options? · true
✔ 🤷   ESP-IDF version (master = UNSTABLE) · v5.3
✔ 🤷   Configure project to use Dev Containers (VS Code and GitHub 
Codespaces)? · false
✔ 🤷   Configure project to support Wokwi simulation with Wokwi VS Code 
extension? · true
✔ 🤷   Add CI files for GitHub Action? · false
[ 1/14]   Done: .cargo/config.toml
[ 2/14]   Done: .cargo
[ 3/14]   Done: .gitignore
[ 4/14]   Done: .vscode/launch.json
[ 5/14]   Done: .vscode
[ 6/14]   Done: Cargo.toml
[ 7/14]   Done: build.rs
[ 8/14]   Done: diagram.json
[ 9/14]   Ignored: pre-script.rhai
[10/14]   Done: rust-toolchain.toml
[11/14]   Done: sdkconfig.defaults
[12/14]   Done: src/main.rs
[13/14]   Done: src
[14/14]   Done: wokwi.toml
🔧   Moving generated files into: `/Users/mac/src/github/esp32/ttgo-display/rust/hello_world`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /Users/mac/src/github/esp32/ttgo-display/rust/hello_world
```

## Example

```sh
cargo generate https://github.com/esp-rs/esp-idf-template cargo
```

if the command `cargo run` does not work,
try build and communicate manually.

- connecting to the board

```sh
cargo build
cargo espflash board-info

Chip type:         esp32 (revision v1.1)
Crystal frequency: 40 MHz
Flash size:        16MB
Features:          WiFi, BT, Dual Core, 240MHz, Coding Scheme None
MAC address:       ac:15:18:dd:aa:ff
```

- flashing the board

```sh
cargo espflash flash

App/part. size:    499,552/1,048,576 bytes, 47.64%
[00:00:01] [========================================]16/16  0 
[00:00:00] [========================================]1/1    0
[00:00:25] [========================================]255/255 0x10000
[2025-01-01T13:32:28Z INFO ] Flashing has completed!
```

- running the app with logs

```sh
cargo espflash monitor

I (434) main_task: Started on CPU0
I (444) main_task: Calling app_main()
I (444) esp_display: Hello, world!
```

## Docker Containers

The tool chaing can be used directly on a Docker container:

<https://hub.docker.com/r/espressif/idf-rust/tags>
