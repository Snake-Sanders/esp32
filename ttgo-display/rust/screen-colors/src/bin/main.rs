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
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::RgbColor};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use esp_hal::spi::{
    Mode,
    master::{Config as SpiConfig, Spi},
};
use esp_hal::time::Rate;
use esp_println as _;
use mipidsi;
use mipidsi::interface::SpiInterface;
use mipidsi::models;
use mipidsi::options::ColorInversion;

#[panic_handler]
fn panic(panic_info: &core::panic::PanicInfo) -> ! {
    error!("{}", panic_info);
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let per = esp_hal::init(config);
    let mut delay = Delay::new();

    let mosi = per.GPIO19;
    let sclk = per.GPIO18;
    let cs = Output::new(per.GPIO5, Level::High, OutputConfig::default());
    let dc = Output::new(per.GPIO16, Level::High, OutputConfig::default());
    let rst = Output::new(per.GPIO23, Level::High, OutputConfig::default());
    let mut bl = Output::new(per.GPIO4, Level::High, OutputConfig::default());

    let spi_conf = SpiConfig::default()
        .with_frequency(Rate::from_hz(40_000_000))
        .with_mode(Mode::_3);

    let spi_bus = Spi::new(per.SPI2, spi_conf)
        .expect("fail creating spi")
        .with_sck(sclk)
        .with_mosi(mosi);

    let spi_device = ExclusiveDevice::new_no_delay(spi_bus, cs).expect("fail creating spi device");

    let mut buffer = [0_u8; 8192];
    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    let mut display = mipidsi::Builder::new(models::ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .init(&mut delay)
        .expect("fail creating display");

    bl.set_high();
    display.clear(Rgb565::BLACK).expect("fail clear display");

    let pause = 3000;
    loop {
        info!("RED");
        display.clear(Rgb565::RED).unwrap();
        delay.delay_millis(pause);

        info!("GREEN");
        display.clear(Rgb565::GREEN).unwrap();
        delay.delay_millis(pause);

        info!("BLUE");
        display.clear(Rgb565::BLUE).unwrap();
        delay.delay_millis(pause);

        info!("BLACK");
        display.clear(Rgb565::BLACK).unwrap();
        delay.delay_millis(pause);

        info!("Off");
        bl.set_low();
        delay.delay_millis(pause);

        info!("On");
        bl.set_high();
    }
}
