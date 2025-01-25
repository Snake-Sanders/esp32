use display_interface_spi::SPIInterface;
use embedded_graphics::pixelcolor::Rgb565;
use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::spi::config::MODE_3;
use esp_idf_svc::hal::spi::Dma;
use esp_idf_svc::hal::spi::SpiConfig;
use esp_idf_svc::hal::spi::SpiDeviceDriver;
use esp_idf_svc::hal::spi::SpiDriver;
use esp_idf_svc::hal::spi::SpiDriverConfig;
use esp_idf_svc::hal::spi::SPI2;
use esp_idf_svc::hal::units::Hertz;
use mipidsi::options::ColorInversion;
use mipidsi::{models::ST7789, Builder};
use mipidsi::error::Error as DisplayError;
use std::error::Error;

pub struct Display<'a> {
    pub(crate) display: mipidsi::Display<
        SPIInterface<
            SpiDeviceDriver<'static>,
            PinDriver<'a, AnyIOPin, esp_idf_svc::hal::gpio::Output>,
        >,
        ST7789,
    >,
    pub(crate) backlight: PinDriver<'a, AnyIOPin, esp_idf_svc::hal::gpio::Output>,
}

impl<'a> Display<'a> {
    pub fn clear(&mut self, color: Rgb565) -> Result<(), DisplayError> {
        self.display.clear(color)
    }
}

pub fn init(peripherals: Peripherals) -> Result<Display<'static>, Box<dyn Error>> {
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

    Ok(Display {
        display,
        backlight: bl,
    })
}
