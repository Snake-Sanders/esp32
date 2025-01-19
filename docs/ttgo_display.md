# ESP32 TTGO Display Board info

Once the port is detected and connected with Arduino IDE, 
try to read the board information.
Go to `Tools/Get board info`:

```
BN: LilyGo T-Display
VID: 0x1A86
PID: 0x55D4
SN: 58AB009719
```

Details: 

This is a LilyGO TTGO T-Display (clone), which is a popular ESP32 development
board with an integrated 1.14" TFT display (driven by the ST7789V controller).

- `LilyGO` is the brand/manufacturer
- `TTGO` is the product line for their ESP32 boards with displays

Features 

- ESP32-D0WDQ6 (dual-core 240MHz processor Xtensa)
  https://www.espressif.com/sites/default/files/documentation/esp32_datasheet_en.pdf
- Integrated Display (`ST7789V`, SPI interface)
  1.14" TFT LCD 135x240 IPS (pixels)
- Two user buttons (can double as reset/boot).
- USB-C for power and programming.

## SPI configuration

|Pin|Function|Notes|
|MOSI|Master Out Slave In|Data sent from ESP32 to display|
|SCLK|Serial Clock|Clock signal to synchronize communication|
|CS|Chip Select|Selects the device (optional, can be tied low)|
|DC|Data/Command|Selects between sending data or command|
|RST|Reset|Resets the display|
|BL|Backlight|Controls the screen backlight|

For SPI, you need to define:

- SPI Bus (MOSI, SCLK)
- Control Pins (DC, RST, BL)
