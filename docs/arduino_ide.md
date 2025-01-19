# Arduino IDE 

In Arduino IDE, go to File > Preferences add the extra sources

`https://dl.espressif.com/dl/package_esp32_index.json`

## Selecting the board

Select the Correct Board:

Go to Tools > Board > ESP32 Arduino > ESP32 Dev Module.
This is the closest and most compatible option for the TTGO T-Display.

## changing the TFT library

Open the Arduino IDE do the following:

1. download the `TFT_eSPI` library into your Arduino IDE 
2. select ESP32 Dev Module. 
3. go to `/Users/<USER>/Documents/Arduino/libraries/TFT_eSPI`
4. edit the `User_Setup_Select.h` file in the `TFT_eSPI` library 
5. select the Setup file for ESP32 and TTGO T-Display ST7789V SPI bus TFT 

This is a clone of the TTGO T-Display module, so select the 
setup file for ESP32 and TTGO T-Display ST7789V SPI bus TFT

```c
//...
//#include <User_Setups/Setup24_ST7789.h>            
#include <User_Setups/Setup25_TTGO_T_Display.h>    
//#include <User_Setups/Setup26_TTGO_T_Wristband.h>  
//...
```

## Linux Mint

On linux mint might complain about it can't  `import serial`

`pip install pyserial`

also there might be an `gpio` compile error

Find the libraries `~/Arduino/libraries/TFT_eSPI/`.
Then add to `processors/TFT_eSPI_ESP32.h` the following

```c
// add these after the processor specific header
#include "driver/gpio.h"
#include <rom/ets_sys.h>
```

