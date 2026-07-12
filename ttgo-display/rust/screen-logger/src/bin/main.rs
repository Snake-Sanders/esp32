// generator version: 1.3.0
// generator parameters: --chip esp32 -o unstable-hal -o stack-smashing-protection -o defmt -o neovim -o esp

#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::{error, info};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::RgbColor,
};
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    main,
    spi::{
        Mode,
        master::{Config as SpiConfig, Spi},
    },
    time::Rate,
};
use esp_println as _;
use mipidsi::{
    self,
    interface::SpiInterface,
    models,
    options::{ColorInversion, Orientation, Rotation},
};
use screen_logger::display_logger;

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
    let pause = 5000;
    info!("Starting...");

    setup();

    let delay = Delay::new();
    loop {
        delay.delay_millis(pause);
    }
}

fn setup() {
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

    let orientation_portrait = Orientation::new().rotate(Rotation::Deg270);
    let mut display = mipidsi::Builder::new(models::ST7789, di)
        .invert_colors(ColorInversion::Inverted)
        .reset_pin(rst)
        .orientation(orientation_portrait)
        .display_size(135, 240)
        .display_offset(52, 40)
        .init(&mut delay)
        .expect("fail creating display");

    bl.set_high();
    display.clear(Rgb565::BLACK).expect("fail clear display");

    let _style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
}
