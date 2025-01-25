mod tt_display;

use esp_idf_svc::hal::peripherals::Peripherals;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
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
    let mut display = tt_display::init(peripherals)?;

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
}
