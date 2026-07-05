# SPI Display

## Stack

```
embedded-graphics
        │
mipidsi (ST7789 driver)
        │
display-interface-spi
        │
embedded-hal traits
        │
esp-hal SPI
        │
ESP32
```

## Implementation steps

Incrementally:

- Initialize SPI with esp-hal.
- Turn on the display backlight.
- Initialize the ST7789 with mipidsi.
- Fill the screen red.
- Fill it blue.
- Draw "Hello, Rust!" with embedded-graphics.
