# ESP32 Rust Development Guide with Docker

This guide explains how to develop ESP32 Rust applications using Docker and Neovim.

## Prerequisites

1. **Docker Desktop** installed and running
2. **Neovim** installed
3. **ESP32 device** (TTGO Display in your case)
4. **USB cable** for connecting ESP32

Download the docker image, this will be downloaded into docker installation
directory. [other images](https://hub.docker.com/r/espressif/idf-rust/tags)

```sh
docker pull espressif/idf-rust:all_1.88.0.0
```

## Quick Start

### 1. Make the script executable

```bash
chmod +x esp-rust.sh
```

### 2. Build your project

```bash
./esp-rust.sh cargo build
```

### 3. Flash to device

```bash
./esp-rust.sh cargo flash
```

## Development Workflow

### Option 1: Interactive Development (Recommended)

1. **Enter interactive shell:**

   ```bash
   ./esp-rust.sh bash
   ```

2. **Inside the container, you can:**
   - Edit files with Neovim: `nvim src/main.rs`
   - Build: `cargo build`
   - Flash: `cargo flash`
   - Monitor: `idf.py monitor`

### Option 2: Direct Commands

Run commands directly without entering the container:

```bash
# Build
./esp-rust.sh cargo build

# Flash
./esp-rust.sh cargo flash

# Monitor serial output
./esp-rust.sh idf.py monitor

# Build with IDF tools
./esp-rust.sh idf.py build
```

## Neovim Setup for ESP32 Development

### 1. Install Rust Language Server

Inside the container or on your host:

```bash
rustup component add rust-analyzer
```

### 2. Neovim Configuration

Create or update your `init.vim` or `init.lua`:

```lua
-- Basic Rust development setup
require('lspconfig').rust_analyzer.setup({
    settings = {
        ['rust-analyzer'] = {
            cargo = {
                loadOutDirsFromCheck = true,
            },
            procMacro = {
                enable = true,
            },
        },
    },
})

-- Key mappings for LSP
vim.keymap.set('n', 'gd', vim.lsp.buf.definition)
vim.keymap.set('n', 'gr', vim.lsp.buf.references)
vim.keymap.set('n', 'K', vim.lsp.buf.hover)
```

### 3. Essential Neovim Plugins

- `nvim-lspconfig` - LSP configuration
- `rust.vim` - Rust syntax highlighting
- `nvim-treesitter` - Better syntax parsing
- `telescope.nvim` - Fuzzy finder

## Project Structure

Your project follows the standard ESP-IDF Rust template:

```
demo_docker/
├── Cargo.toml          # Rust dependencies and configuration
├── build.rs            # Build script for ESP-IDF integration
├── sdkconfig.defaults  # ESP-IDF configuration
├── esp-rust.sh         # Docker development script
└── src/
    └── main.rs         # Main application code
```

## Common Commands

### Building

```bash
# Development build
./esp-rust.sh cargo build

# Release build
./esp-rust.sh cargo build --release
```

### Flashing

```bash
# Flash to ESP32
./esp-rust.sh cargo flash

# Flash with specific port (if multiple devices)
./esp-rust.sh cargo flash -- --port /dev/ttyUSB1
```

### Monitoring

```bash
# Monitor serial output
./esp-rust.sh idf.py monitor

# Monitor with specific baud rate
./esp-rust.sh idf.py monitor --baud 115200
```

### Debugging

```bash
# Build with debug symbols
./esp-rust.sh cargo build

# Run with logging
./esp-rust.sh cargo run
```

## Troubleshooting

### USB Permission Issues

If you get permission errors when flashing:

```bash
# Add your user to the dialout group
sudo usermod -a -G dialout $USER

# Or use sudo for the script
sudo ./esp-rust.sh cargo flash
```

### Port Not Found

If the script can't find your ESP32:

```bash
# List USB devices
ls -la /dev/ttyUSB*

# Check if device is recognized
dmesg | grep tty
```

### Docker Image Issues

If the image fails to pull:

```bash
# Pull manually
docker pull espressif/idf-rust:all_1.88.0.0

# Or use a different tag
# Update IMAGE variable in esp-rust.sh
```

## Advanced Usage

### Custom Docker Image

You can modify the script to use a different image:

```bash
# Edit esp-rust.sh and change IMAGE variable
IMAGE="espressif/idf-rust:latest"
```

### Persistent Development Environment

For longer development sessions, you can create a persistent container:

```bash
# Create a named container
docker run -it --name esp32-dev \
    -v "$(pwd)":/project \
    -w /project \
    espressif/idf-rust:all_1.88.0.0 bash

# Reuse the container
docker start -i esp32-dev
```

### Multiple ESP32 Devices

If you have multiple ESP32 devices:

```bash
# List all USB devices
ls /dev/ttyUSB*

# Use specific device
./esp-rust.sh cargo flash -- --port /dev/ttyUSB1
```

## Next Steps

1. **Customize your application** in `src/main.rs`
2. **Add dependencies** in `Cargo.toml`
3. **Configure ESP-IDF settings** in `sdkconfig.defaults`
4. **Set up your Neovim environment** for optimal development
5. **Test with your TTGO Display hardware**

## Resources

- [ESP-IDF Rust Guide](https://github.com/esp-rs/esp-idf-template)
- [Rust on ESP32 Book](https://esp-rs.github.io/book/)
- [ESP-IDF Documentation](https://docs.espressif.com/projects/esp-idf/)
- [Neovim LSP Setup](https://neovim.io/doc/user/lsp.html)

Happy coding! 🦀✨
