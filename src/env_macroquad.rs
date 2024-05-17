/** ===============================================================================
 * File: env_piston.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements graphical 2D environment using `macroquad` graphics lib
 * ===============================================================================*/
use crate::creature::*;
use macroquad::prelude::*;


//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEBUG_LEVEL : usize = 1;

const ENV_X_SIZE : f32 = 640.0;
const ENV_Y_SIZE : f32 = 480.0;

const NUM_GRID_SQUARES_X : f32 = 60.0;
const NUM_GRID_SQUARES_Y : f32 = 40.0;

const GRID_X_SIZE : f32 = ENV_X_SIZE / NUM_GRID_SQUARES_X;
const GRID_Y_SIZE : f32 = ENV_Y_SIZE / NUM_GRID_SQUARES_Y;

//===============================================================================
// DATA
//===============================================================================

/// Constant parameters for this simulation that are passed into the main function
struct SimParameters {
    env_x_size : usize,     // X size of the environment in pixels
    env_y_size : usize,     // Y Size of the environment in pixels
    
}

/// Enumeration that defines the possible space states
#[derive(Copy, Clone)]
pub enum SpaceStates {
    BlankSpace,                 // Space is blank
    CreatureSpace(usize),       // Space has a creature in it. The single argument represents the ID of the creature
    FoodSpace,                  // Space has a food in it
}

/// Placeholder for piston library environment
struct EnvMacroquad {
    params : SimParameters,     // Constant values that sim is initialized with

}


//===============================================================================
// FUNCTIONS
//===============================================================================

pub fn run_sim() {
    clear_background(GRAY);

    // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    draw_single_square(0, 0);
    draw_single_square(5, 5);
    draw_single_square(6, 5);
    draw_single_square(7, 5);
    draw_single_square(8, 5);
    draw_single_square(7, 5);
    draw_single_square(7, 6);
    draw_single_square(7, 5);
    draw_single_square(7, 4);

    draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

}


/// Draw a single square to the 
pub fn draw_single_square(x_pos : usize, y_pos : usize) {
    draw_rectangle((x_pos as f32) * GRID_X_SIZE, (y_pos as f32) * GRID_Y_SIZE, GRID_X_SIZE, GRID_Y_SIZE, BLUE);
}

