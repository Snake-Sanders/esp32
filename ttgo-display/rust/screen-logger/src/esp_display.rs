use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    mono_font::{MonoTextStyle, ascii::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::{RgbColor, *},
    primitives::{PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::gpio::{AnyPin, Output};
use esp_hal::spi::{
    Mode,
    master::{AnySpi, Config as SpiConfig, Spi},
};
use esp_hal::time::Rate;
use esp_hal::{Blocking, delay::Delay};
use esp_println as _;
use mipidsi::models;
use mipidsi::options::ColorInversion;
use mipidsi::{self, options::Orientation};
use mipidsi::{interface::SpiInterface, options::Rotation};
use static_cell::StaticCell;

const SPI_BIFFER_SIZE: usize = 8192;
static SPI_BUFFER: StaticCell<[u8; SPI_BIFFER_SIZE]> = StaticCell::new();

pub struct EspDisplayGpio<'a> {
    pub mosi: AnyPin<'a>,
    pub sclk: AnyPin<'a>,
    pub spi: AnySpi<'a>,
    pub cs: Output<'a>,
    pub dc: Output<'a>,
    pub rst: Output<'a>,
    pub bl: Output<'a>,
}

type DisplayModel = models::ST7789;
type DisplayReset<'a> = Output<'a>;
type SpiBus<'a> = Spi<'a, Blocking>;
type DisplayInterface<'a> =
    SpiInterface<'a, ExclusiveDevice<SpiBus<'a>, Output<'a>, NoDelay>, Output<'a>>;
type MipiDisplay<'a> = mipidsi::Display<DisplayInterface<'a>, DisplayModel, DisplayReset<'a>>;

pub struct EspDisplay<'a> {
    text_style: MonoTextStyle<'a, Rgb565>,
    mipi_display: MipiDisplay<'a>,
}

// TODO: implement display logs with defmt
impl<'a> EspDisplay<'a> {
    pub fn new(mut gpio: EspDisplayGpio<'a>) -> Self {
        let spi_conf = SpiConfig::default()
            .with_frequency(Rate::from_hz(40_000_000))
            .with_mode(Mode::_3);

        let spi_bus = Spi::new(gpio.spi, spi_conf)
            .expect("fail creating spi")
            .with_sck(gpio.sclk)
            .with_mosi(gpio.mosi);

        let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
        let spi_device =
            ExclusiveDevice::new_no_delay(spi_bus, gpio.cs).expect("fail creating spi device");

        let buffer: &'static mut [u8] = SPI_BUFFER.init([0_u8; SPI_BIFFER_SIZE]);
        let di = SpiInterface::new(spi_device, gpio.dc, buffer);

        let orientation_portrait = Orientation::new().rotate(Rotation::Deg270);
        let mut delay = Delay::new();

        let mut display = mipidsi::Builder::new(models::ST7789, di)
            .invert_colors(ColorInversion::Inverted)
            .reset_pin(gpio.rst)
            .orientation(orientation_portrait)
            .display_size(135, 240)
            .display_offset(52, 40)
            .init(&mut delay)
            .expect("fail creating display");

        gpio.bl.set_high();
        display.clear(Rgb565::BLACK).expect("fail clear display");

        Rectangle::new(Point::new(0, 0), Size::new(240, 135))
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 1))
            .draw(&mut display)
            .unwrap();

        Self {
            text_style,
            mipi_display: display,
        }
    }

    /// renders the test starting at the position x y
    ///
    /// ("Tl", Point::new(0, 20)),
    // ("Ml", Point::new(0, 70)),
    // ("Bl", Point::new(0, 125)),
    // ("C", Point::new(110, 70)),
    // ("Tr", Point::new(220, 20)),
    // ("Mr", Point::new(220, 70)),
    // ("Br", Point::new(220, 125)),

    pub fn render(&mut self, text: &str, position: Point) {
        Text::new(text, position, self.text_style)
            .draw(&mut self.mipi_display)
            .unwrap();
    }
}
