/*
 An example digital clock using a TFT LCD screen to show the time.
 Demonstrates use of the font printing routines. (Time updates but date does not.)

 It uses the time of compile/upload to set the time
 For a more accurate clock, it would be better to use the RTClib library.
 But this is just a demo...

 Make sure all the display driver and pin connections are correct by
 editing the User_Setup.h file in the TFT_eSPI library folder.

 #########################################################################
 ###### DON'T FORGET TO UPDATE THE User_Setup.h FILE IN THE LIBRARY ######
 #########################################################################

 Based on clock sketch by Gilchrist 6/2/2014 1.0

A few colour codes:

code	color
0x0000	Black
0xFFFF	White
0xBDF7	Light Gray
0x7BEF	Dark Gray
0xF800	Red
0xFFE0	Yellow
0xFBE0	Orange
0x79E0	Brown
0x7E0	Green
0x7FF	Cyan
0x1F	Blue
0xF81F	Pink

 */

#include <TFT_eSPI.h>  // Hardware-specific library
#include <SPI.h>

#define TFT_GREY 0x5AEB

TFT_eSPI tft = TFT_eSPI();  // Invoke custom library

void test_display_bounds() {
    tft.fillScreen(TFT_BLACK);  // Clear screen

    // Draw a RED cross at (0,0) - Check if it's visible
    tft.drawFastVLine(0, 0, 4, TFT_RED);  // Vertical line
    tft.drawFastHLine(0, 0, 4, TFT_RED);  // Horizontal line

    // Draw a GREEN rectangle from (0,0) to (135,240) to find the real width/height
    tft.drawRect(0, 0, 135, 240, TFT_GREEN);


    // Print coordinates at edges
    tft.setTextColor(TFT_WHITE, TFT_BLACK);
    tft.setTextSize(2);

    tft.setCursor(5, 5);
    tft.print("(0,0)");

    tft.setCursor(100, 5);
    tft.print("(W,0)");

    tft.setCursor(5, 220);
    tft.print("(0,H)");

    tft.setCursor(100, 220);
    tft.print("(W,H)");

    Serial.println("Test pattern drawn. Check display boundaries.");
}

void setup(void) {
  //Serial.begin(115200);
  tft.init();

  // adjustemts for TTGo-Display
  tft.setRotation(0);
  tft.writecommand(TFT_MADCTL);
  tft.writedata(0x08);
  tft.invertDisplay(false);
}

void loop() {
  
  tft.drawRect(4, 0, tft.width() - 10, tft.height() -10, TFT_GREEN);

  delay(5000);
  tft.fillScreen(TFT_BLACK);
    // Draw boundary lines to detect real edges
    tft.drawFastHLine(0, 120, 135, TFT_BLUE);  // Center horizontal
    tft.drawFastVLine(67, 0, 240, TFT_RED);   // Center vertical

  delay(5000);
  tft.fillScreen(TFT_BLACK);
    tft.setCursor(5, 5, 4);
    tft.print("A");
    tft.setCursor(20, 20, 4);
    tft.print("B");
    tft.setCursor(40, 40, 4);
    tft.print("C");
    delay(5000);
  tft.fillScreen(TFT_BLACK);
}
