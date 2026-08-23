// generator version: 1.3.0
// generator parameters: --chip esp32 -o unstable-hal -o stack-smashing-protection -o defmt -o neovim -o esp

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::{error, info};
use embedded_graphics::geometry::Point;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_println as _;
use screen_logger::esp_display::{EspDisplay, EspDisplayGpio};

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

fn setup() -> EspDisplay<'static> {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let per = esp_hal::init(config);

    info!("Setup");

    let display_gpio = EspDisplayGpio {
        mosi: per.GPIO19.into(),
        sclk: per.GPIO18.into(),
        spi: per.SPI2.into(),
        cs: Output::new(per.GPIO5, Level::High, OutputConfig::default()),
        dc: Output::new(per.GPIO16, Level::High, OutputConfig::default()),
        rst: Output::new(per.GPIO23, Level::High, OutputConfig::default()),
        bl: Output::new(per.GPIO4, Level::High, OutputConfig::default()),
    };

    EspDisplay::new(display_gpio)
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    info!("Starting ESP");
    let mut display = setup();
    let delay = Delay::new();

    loop {
        display.render("hello", Point::new(0, 20));
        delay.delay_millis(5000);
    }
}
