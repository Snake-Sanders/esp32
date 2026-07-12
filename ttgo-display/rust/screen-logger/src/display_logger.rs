use core::fmt;

use embedded_graphics::{
    Drawable,
    draw_target::DrawTarget,
    mono_font::{MonoTextStyle, iso_8859_10::FONT_10X20},
    pixelcolor::Rgb565,
    prelude::{Point, RgbColor},
    text::Text,
};

// Depends on font size
const FONT_HEIGHT: i32 = 20;

pub struct DisplayLogger<'a, D> {
    cursor_y: i32,
    display: D,
    font_style: MonoTextStyle<'a, Rgb565>,
}

impl<D> DisplayLogger<'_, D>
where
    D: DrawTarget<Color = Rgb565>,
{
    pub fn new(display: D) -> Self {
        Self {
            cursor_y: 0,
            display,
            font_style: MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE),
        }
    }
}

impl<D> fmt::Write for DisplayLogger<'_, D>
where
    D: DrawTarget<Color = Rgb565>,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let position = Point::new(0, self.cursor_y);
        let _ = Text::new(s, position, self.font_style).draw(&mut self.display);

        self.cursor_y += FONT_HEIGHT;

        Ok(())
    }
}
