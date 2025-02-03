
/** ===============================================================================
 * File: main_led.rs
 * Author: Scott Stack
 * Description: main application entry point for LED matrix target version of the program
 * This version interfaces specifically with HUB75 matrix LED wall panels
 * ===============================================================================*/
mod linalg;
mod neural_net;
mod creature;
mod environment;
mod env_console;
mod hub75_led_driver;
mod librgbmatrix_defines;

use std::io;
use hub75_led_driver::*;
use librgbmatrix_defines::Color;

/// Main function for command line sim visualization version
fn main() {
    let driver = RGBLedMatrixDriver::new();

    // Just set the middle pixel to blue-ish
    let pixel_color = Color{r: 100, g:  100, b: 255};
    driver.set_pixel(32, 32, pixel_color);

    // Infinite loop
    println!("Press enter to exit...");
    let mut choice = String::new();
    let res = io::stdin().read_line(&mut choice).unwrap();

    driver.close();
}

