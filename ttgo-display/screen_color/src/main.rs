use display_interface_spi::SPIInterface;

use embedded_graphics::prelude::RgbColor;
use embedded_graphics::{pixelcolor::Rgb565, prelude::*};

use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::spi::config::MODE_3;
use esp_idf_svc::hal::spi::Dma;
// SpiConfig was before spi::config::Config;
use esp_idf_svc::hal::spi::SpiConfig;
// SpiDriverConfig was before spi::config::DriverConfig
use esp_idf_svc::hal::spi::SpiDeviceDriver;
use esp_idf_svc::hal::spi::SpiDriver;
use esp_idf_svc::hal::spi::SpiDriverConfig;
use esp_idf_svc::hal::spi::SPI2;
use esp_idf_svc::hal::units::Hertz;

use mipidsi::options::ColorInversion;
use mipidsi::{models::ST7789, Builder};

use std::error::Error;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Starting display...v1");

    let peripherals = Peripherals::take()?;

    /* Define the SPI interface as the variable `spi` */
    let mut delay = Ets;

    // Pin definitions
    let sclk = peripherals.pins.gpio18; // SCLK
    let sdo = peripherals.pins.gpio19; // SDO aka MOSI
    let rst = peripherals.pins.gpio23; // Reset
    let dc = peripherals.pins.gpio16; // Data/Command
    let cs = peripherals.pins.gpio5; // Chip Select
    let bl = peripherals.pins.gpio4; // Backlight

    // Initialize GPIO
    let rst = PinDriver::output(rst)?;
    let dc = PinDriver::output(dc)?;
    let mut bl = PinDriver::output(bl)?;
    let sdi: Option<AnyIOPin> = None;

    // to avoid swipe effect update the SPI frequency
    // 40000000
    // - `240 * 135` – Total number of pixels.
    // - `* 2` – Two bytes per pixel (RGB565).
    // - `+ 8` – Extra space for DMA alignment or control data.
    let dma = Dma::Auto(240 * 135 * 2 + 8);

    // SpiDriver – Represents the raw SPI bus where
    // one SpiDriver per SPI bus, this handles many SPI devices
    // connected to this bus.

    let spi_driver = SpiDriver::new::<SPI2>(
        peripherals.spi2,
        sclk,
        sdo,
        sdi, // no MISO
        &SpiDriverConfig::default().dma(dma),
    )?;

    // SpiDeviceDriver - A higher-level abstraction representing
    // a single device on the SPI bus.
    let spi_config = SpiConfig {
        baudrate: Hertz(20_000_000), // High SPI speed for display
        data_mode: MODE_3,           // SPI Mode 3
        write_only: true,            // No read operations
        polling: false,              // Use DMA/interrupt, not polling
        queue_size: 3,               // Allow multiple operations in queue
        ..Default::default()
    };

    let spi_device = SpiDeviceDriver::new(spi_driver, Some(cs), &spi_config)?;

    // Create a DisplayInterface from SPI and DC pin, with no manual CS control
    let di = SPIInterface::new(spi_device, dc);

    // Create the ILI9486 display driver from the display interface and optional RST pin
    let mut display = Builder::new(ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        //.display_size(240 as u16, 135 as u16)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    // back light on
    bl.set_high()?;

    // Clear the display to red
    display.clear(Rgb565::RED.into()).unwrap();
    log::info!("red");

    log::info!("Display initialized.");

    loop {
        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::BLUE.into()).unwrap();
        log::info!("blue");

        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::MAGENTA.into()).unwrap();
        log::info!("magenta");

        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::GREEN.into()).unwrap();
        log::info!("green");

        thread::sleep(Duration::from_secs(1));
        display.clear(Rgb565::YELLOW.into()).unwrap();
        log::info!("yellow");

        display.clear(Rgb565::RED.into()).unwrap();
        log::info!("red");

        log::info!("repeat");
    }

    //Ok(())
}
