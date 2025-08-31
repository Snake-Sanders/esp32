#!/usr/bin/env bash
# esp-rust.sh — Run Espressif's Rust toolchain inside Docker
#
# requires exec permissions
# chmod +x esp-rust.sh
#
# Usage examples:
# ./esp-rust.sh bash          # Enter interactive shell
# ./esp-rust.sh cargo build   # Build the project
# ./esp-rust.sh cargo flash   # Flash to device
# ./esp-rust.sh idf.py monitor # Monitor serial output

# Docker Image to use
IMAGE="espressif/idf-rust:all_1.88.0.0"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "Error: Docker is not running. Please start Docker first."
    exit 1
fi

# Check if image exists, pull if not
if ! docker image inspect "$IMAGE" > /dev/null 2>&1; then
    echo "Pulling Docker image $IMAGE..."
    docker pull "$IMAGE"
fi

# Function to show usage
show_usage() {
    echo "ESP32 Rust Development Docker Script"
    echo "===================================="
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  bash              - Enter interactive shell in container"
    echo "  cargo build       - Build the project"
    echo "  cargo flash       - Flash to ESP32 device"
    echo "  idf.py monitor    - Monitor serial output"
    echo "  idf.py flash      - Flash using IDF tools"
    echo "  idf.py build      - Build using IDF tools"
    echo ""
    echo "Examples:"
    echo "  $0 bash           # Interactive development"
    echo "  $0 cargo build    # Build project"
    echo "  $0 cargo flash    # Flash to device"
    echo ""
    echo "Note: Make sure your ESP32 is connected via USB"
}

# If no arguments, show usage
if [ $# -eq 0 ]; then
    show_usage
    exit 0
fi

#  -it: Interactive terminal (keeps stdin/stdout connected)
# --rm : Automatically remove container when it exits
# -v "$(pwd)":/project : Mount the current directory into /project in the container
# -w /project : Start inside /project (your mounted folder)
# --device=/dev/ttyUSB0:/dev/ttyUSB0 : Mount USB device for flashing (if exists)
# $IMAGE "$@" : Run the specified command

# Check if USB device exists for flashing
USB_DEVICE=""
if [ -e "/dev/ttyUSB0" ]; then
    USB_DEVICE="--device=/dev/ttyUSB0:/dev/ttyUSB0"
    echo "USB device detected: /dev/ttyUSB0"
fi

# Run the container:
echo "Starting ESP32 Rust development environment..."
docker run -it --rm \
    -v "$(pwd)":/project \
    -w /project \
    $USB_DEVICE \
    "$IMAGE" "$@"

# Notes:
# - "$(pwd)" ensures your current working dir is accessible in the container
# - "/project" is arbitrary; you just need to remember that's where your files are
# - "$@" forwards any arguments passed to this script into the container
# - USB device mounting enables direct flashing to ESP32
