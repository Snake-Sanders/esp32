# Rust Setup

reference `https://lilymara.xyz/posts/2023/01/images-esp32/`

Install cross compiler target

For ESP32 there are two micros, Xtensa and RISCV. The newer variants come with
RISCV. To compile for these target a cross compiler is needed, this is `Espup`.

## Espup

Espup requirements

check if the following packages are installed with `cargo install --list`

```sh
cargo install ldproxy
```

Esp-IDF is also a dependency but this gets installed automatically when building
`std`.

### Update

If the tool was not previously installed then skip this section and read
**Install**.

If you are now updating the tool version then it is probable this might prompt
you some errors.

Make sure you are not in the ESP project directory. These directories have
some environment variables overwrite. Best is to work in $HOME.

An indication of this case is the error 17.

The error **"File exists (os error 17)"** during the installation of the Xtensa
LLVM toolchain indicates that a symlink already exists at the target location,
preventing the creation of a new one.

To solve this delete the link from the previous installation:

`rm ~/.espup/esp-clang`

Then install

```sh
espup install

[info]: Installing the Espressif Rust ecosystem
[info]: Checking Rust installation
[info]: Installing RISC-V Rust targets ('riscv32imc-unknown-none-elf', 'riscv32imac-unknown-none-elf' and 'riscv32imafc-unknown-none-elf') for 'stable' toolchain
[warn]: Previous installation of LLVM exists in: '/Users/mac/.rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-19.1.2_20250225'. Reusing this installation
[info]: Installing GCC (xtensa-esp-elf)
[info]: Creating symlink between '/Users/mac/.rustup/toolchains/esp/xtensa-esp32-elf-clang/esp-19.1.2_20250225/esp-clang/lib' and '/Users/mac/.espup/esp-clang'
[warn]: Previous installation of GCC exists in: '/Users/mac/.rustup/toolchains/esp/xtensa-esp-elf/esp-14.2.0_20240906'. Reusing this installation
[warn]: Failed to detect version of Xtensa Rust, reinstalling it
[info]: Uninstalling Xtensa Rust toolchain
[info]: Installing Xtensa Rust 1.88.0.0 toolchain
[info]: All downloads complete
...

To get started, you need to set up some environment variables by running: 
  '. /Users/mac/export-esp.sh'
This step must be done every time you open a new terminal.
See other methods for setting the environment in 
https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables
```

- Using a docker image can same some time troubleshooting incompatible packages:

### Install

(installing this target might no longer be needed)

```sh
rustup target install riscv32imc-unknown-none-elf
```

```sh
cargo install espup
espup install

# at this moment setup the environment variables

cargo install cargo-espflash
cargo install cargo-generate
```

About [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)

- checkpoint

```sh
xtensa-esp32-elf-gcc --version

xtensa-esp-elf-gcc (crosstool-NG esp-14.2.0_20240906) 14.2.0
```

**Environment variables**

`espup` requires environment variables from `~/export-esp.sh`
to avoid polluting `.zshrc`, the variables are set in `.env`

 '. /Users/mac/export-esp.sh'
This step must be done every time you open a new terminal.
See other methods for setting the environment in:

<https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables>

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
