use display_interface_spi::SPIInterface;

use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;

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

type Display<'a> = mipidsi::Display<
    SPIInterface<SpiDeviceDriver<'static>, PinDriver<'a, AnyIOPin, esp_idf_svc::hal::gpio::Output>>,
    ST7789,
>;

fn init_display(
    peripherals: Peripherals,
) -> Result<(Display, PinDriver<AnyIOPin, esp_idf_svc::hal::gpio::Output>), Box<dyn Error>> {
    let mut delay = Ets;

    // Pin definitions
    let sclk = peripherals.pins.gpio18;
    let sdo = peripherals.pins.gpio19;
    let rst = peripherals.pins.gpio23;
    let dc = peripherals.pins.gpio16;
    let cs = peripherals.pins.gpio5;
    let bl = peripherals.pins.gpio4;

    // Initialize GPIO
    let rst = PinDriver::output(rst)?;
    let dc = PinDriver::output(dc)?;
    let mut bl = PinDriver::output(bl)?;
    let sdi: Option<AnyIOPin> = None;

    let dma = Dma::Auto(240 * 135 * 2 + 8);

    let spi_driver = SpiDriver::new::<SPI2>(
        peripherals.spi2,
        sclk,
        sdo,
        sdi,
        &SpiDriverConfig::default().dma(dma),
    )?;

    let spi_config = SpiConfig {
        baudrate: Hertz(20_000_000),
        data_mode: MODE_3,
        write_only: true,
        polling: false,
        queue_size: 3,
        ..Default::default()
    };

    let spi_device = SpiDeviceDriver::new(spi_driver, Some(cs), &spi_config)?;
    let di = SPIInterface::new(spi_device, dc);

    let display = Builder::new(ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut delay)
        .unwrap();

    bl.set_high()?;

    Ok((display, bl))
}

fn main() -> Result<(), Box<dyn Error>> {
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("Starting display...v1");

    let peripherals = Peripherals::take()?;
    let (mut display, _bl) = init_display(peripherals)?;

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
