# Rust Setup

reference `https://lilymara.xyz/posts/2023/01/images-esp32/`

Install cross compiler target

```sh
rustup target install riscv32imc-unknown-none-elf
```

```sh
cargo install espup
espup install

# at this moment setup the environment variables

cargo install cargo-espflash
cargo install cargo-generate
cargo install ldproxy
```
About [espup](https://docs.esp-rs.org/book/installation/riscv-and-xtensa.html)

- checkpoint

```sh
xtensa-esp32-elf-gcc --version

xtensa-esp-elf-gcc (crosstool-NG esp-14.2.0_20240906) 14.2.0
```

**Environment varaiables**

`espup` requires environment variables from `~/export-esp.sh`
to avoid polluting `.zshrc`, the variables are set in `.env`

 '. /Users/mac/export-esp.sh'
This step must be done every time you open a new terminal.
See other methods for setting the environment in: 

https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables

## Crates 

Repositories naming convetions

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

[embedded-graphics] (https://github.com/embedded-graphics/embedded-graphics)
[mipidsi](https://github.com/almindor/mipidsi)
See [Troubleshooting](https://github.com/almindor/mipidsi/blob/master/docs/TROUBLESHOOTING.md) 
wrong color and inversion

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

https://hub.docker.com/r/espressif/idf-rust/tags


