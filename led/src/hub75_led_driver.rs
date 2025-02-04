/**
 * @file
 * @author - Scott Stack
 * @description - This file implements a driver for the HUB75 type LED display boards. Really it's just
 * a wrapper to the real raspberry pi driver here - https://github.com/hzeller/rpi-rgb-led-matrix
 * 
 * This depends on having access to the already built version of the library which will be statically linked
 * into the final executable version of this application
 */
use crate::librgbmatrix_defines::*;

// Set LED matrix panel properties
const NUM_LED_ROWS : ::std::os::raw::c_int = 64;
const NUM_LED_COLS : ::std::os::raw::c_int = 64;

/// Implement a driver for the HUB75 RGB LED matrix
pub struct RGBLedMatrixDriver {
    matrix : *mut RGBLedMatrix,         // Main matrix object pointer used by underlying C library
    offscreen_canvas : *mut LedCanvas,  // The screen is double buffered, and this canvas is what we'll draw on before applying changes
}


/// Implementation for RGB Matrix Driver
impl RGBLedMatrixDriver {

    /// Constructor that uses the default options for this project
    pub fn new() -> RGBLedMatrixDriver {
        // Using the adafruit-hat GPIO mapping: https://www.adafruit.com/product/3211
        let hardware_mapping = std::ffi::CString::new("adafruit-hat").unwrap();

        // First instantiate options
        let mut options = RGBLedMatrixOptions {
            hardware_mapping : hardware_mapping.as_ptr(),
            rows : NUM_LED_ROWS,
            cols : NUM_LED_COLS,
            chain_length : 1,
            parallel : 0,
            pwm_bits : 0,
            pwm_lsb_nanoseconds : 0,
            pwm_dither_bits : 0,
            brightness : 0,
            scan_mode : 0,
            row_address_type : 0,
            multiplexing : 0,
            disable_hardware_pulsing : false, 
            show_refresh_rate : false,
            inverse_colors : false, 
            led_rgb_sequence : std::ptr::null(),
            pixel_mapper_config : std::ptr::null(),
            panel_type : std::ptr::null(),
            limit_refresh_rate_hz : 0,
            disable_busy_waiting : false
        };

        // Next instantiate runtime options
        let mut rt_options  = RGBLedRuntimeOptions {
            gpio_slowdown : 0,
            daemon : 0,
            drop_privileges : 0,
            do_gpio_init : false,
            drop_priv_user : std::ptr::null(),
            drop_priv_group : std::ptr::null(),
        };

        // Call the C function to actually initialize the matrix. This will start a thread in the background
        // that continuously updates the LED matrix
        let tmp_matrix : *mut RGBLedMatrix;
        unsafe {
            tmp_matrix = led_matrix_create_from_options_and_rt_options(&mut options, &mut rt_options);
        }

        // Make sure driver returned ok
        if tmp_matrix == std::ptr::null_mut() {
            panic!("Error! led matrix driver returned NULL!");
        }

        let tmp_offscreen_canvas : *mut LedCanvas;
        unsafe {
            tmp_offscreen_canvas = led_matrix_create_offscreen_canvas(tmp_matrix);
            if tmp_offscreen_canvas == std::ptr::null_mut() {
                panic!("Error: Could not create double buffered offscreen canvas for LED display");
            }
        }

        // Return the driver object initialized with matrix
        return RGBLedMatrixDriver { matrix: tmp_matrix, offscreen_canvas : tmp_offscreen_canvas };

    }


    /// Set value of an individual pixel in the buffered version of the image (next frame to be displayed)
    pub fn set_buffered_pixel(&self, x : i32, y : i32, color : Color) {
        unsafe {
            // let canvas = led_matrix_get_canvas(self.matrix);
            led_canvas_set_pixel(self.offscreen_canvas, x, y, color.r, color.g, color.b);
        }
    }

    /// Clear the entire buffered version of the screen (next frame)
    pub fn clear_buffered_screen(&self) {
        unsafe {
            led_canvas_clear(self.offscreen_canvas);
        }
    }

    /// Actually display the buffered frame that's been built up and swap the offscreen buffer
    pub fn apply_buffered_frame(&mut self) {
        unsafe {
            self.offscreen_canvas = led_matrix_swap_on_vsync(self.matrix, self.offscreen_canvas);
        } 
    }

    /// Get current brightness setting on the matrix
    pub fn get_matrix_brightness(&self) -> u8 {
        let brightness : u8;
        unsafe {
            brightness = led_matrix_get_brightness(self.matrix);
        }
        return brightness;
    }

    /// Set brightness of the matrix
    pub fn set_matrix_brightness(&mut self, brightness : u8) {
        unsafe {
            led_matrix_set_brightness(self.matrix, brightness);
        }
    }

    /// Close the matrix object to stop background process and free up memory.
    /// Properly resets all of the hardware to default state
    /// THIS MUST BE CALLED BEFORE ENDING THE PROGRAM!!
    pub fn close(&self) {
        unsafe {
            led_matrix_delete(self.matrix);
        }
    } 

}

