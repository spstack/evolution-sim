/** ===============================================================================
 * File: env_piston.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements graphical 2D environment using `macroquad` graphics lib
 * ===============================================================================*/
use crate::creature::*;
use crate::environment::*;
use macroquad::prelude::*;


//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEBUG_LEVEL : usize = 1;

const SCREEN_SIZE_X : f32 = 640.0;
const SCREEN_SIZE_Y : f32 = 480.0;

const NUM_GRID_SQUARES_X : usize = 50;
const NUM_GRID_SQUARES_Y : usize = 50;

// const GRID_X_SIZE : f32 = SCREEN_SIZE_X / NUM_GRID_SQUARES_X as f32;
// const GRID_Y_SIZE : f32 = SCREEN_SIZE_Y / NUM_GRID_SQUARES_Y as f32;

//===============================================================================
// DATA
//===============================================================================

/// Constant parameters for this simulation that are passed into the main function
struct SimParameters {
    screen_width_pixels : f32,      // X Size of the environment in pixels
    screen_height_pixels : f32,     // Y size of the environment in pixels
    grid_x_size : f32,              // X size of a single grid square in pixels
    grid_y_size : f32,              // Y size of a single grid square in pixels
}


/// Environment
pub struct EnvMacroquad {
    pub params : SimParameters,     // Constant values that sim is initialized with
    pub env : EnvironmentV1,        // Contains the whole environment
}


//===============================================================================
// FUNCTIONS
//===============================================================================

impl EnvMacroquad {

    /// Get a new instance of the Macroquad environment
    pub fn new() -> EnvMacroquad {
        return EnvMacroquad {
            params : SimParameters {
                screen_width_pixels : SCREEN_SIZE_X,
                screen_height_pixels : SCREEN_SIZE_Y,
                grid_x_size : SCREEN_SIZE_X / NUM_GRID_SQUARES_X as f32,
                grid_y_size : SCREEN_SIZE_Y / NUM_GRID_SQUARES_Y as f32,
            },
            env : EnvironmentV1::new(
                NUM_GRID_SQUARES_X, // env_x_size
                NUM_GRID_SQUARES_Y, // env_y_size
                50, // num_start_creatures
                20, // num_start_food
            ),
        }
    }

    /// Run and display the next step of the simulation
    pub fn run_next_step(&mut self) {
        self.env.advance_step();
        self.env.show_all_creature_info()
    }

    /// Update the display
    pub fn update_display(&self) {
        clear_background(GRAY);
        // for creature in &self.env.creatures {
        //     draw_single_square(creature.position.x, creature.position.y);
        // }

        for x in 0..self.env.env_x_size {
            for y in 0..self.env.env_y_size {
                match self.env.positions[x][y] {
                    SpaceStates::CreatureSpace(id) => self.draw_creature_square(x, y),
                    SpaceStates::FoodSpace => self.draw_food_space(x, y),
                    _ => (),
                }
            }
        }


    }
        // println!();
        // println!("-----------------------------------------------------------------------------");
        // for y in -1..ENV_Y_SIZE {
        //     print!("|");
        //     for x in -1..ENV_X_SIZE {
        //         match self.positions[x][y] {
        //             SpaceStates::BlankSpace => print!("   "),
        //             SpaceStates::CreatureSpace(id) => print!("{:1} ", id),
        //             SpaceStates::FoodSpace => print!(" # "),
        //         }
        //     }
        //     print!("|");
        //     println!();
        // }
        // println!("-----------------------------------------------------------------------------");
        // println!("Key:");
        // println!("Creature = <id num>\nFood = #");



    /// Draw a single creature square to the specified location on the screen
    fn draw_creature_square(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.params.grid_x_size, (y_pos as f32) * self.params.grid_y_size, self.params.grid_x_size, self.params.grid_y_size, BLUE);
    }
    /// Draw a single food space on the screen
    fn draw_food_space(&self, x_pos : usize, y_pos : usize) {
        draw_rectangle((x_pos as f32) * self.params.grid_x_size, (y_pos as f32) * self.params.grid_y_size, self.params.grid_x_size, self.params.grid_y_size, GREEN);
    }
    /// Draw a wall space on the screen
    fn draw_wall_space(&self, x_pos : usize, y_pos : usize) {
        // draw_rectangle((x_pos as f32) * self.params.grid_x_size, (y_pos as f32) * self.grid_y_size, self.grid_x_size, self.grid_y_size, BLACK);
    }


}

pub fn start_sim() {

}

pub fn run_sim() {
    clear_background(GRAY);

    // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    // draw_single_square(0, 0);
    // draw_single_square(5, 5);
    // draw_single_square(6, 5);
    // draw_single_square(7, 5);
    // draw_single_square(8, 5);
    // draw_single_square(7, 5);
    // draw_single_square(7, 6);
    // draw_single_square(7, 5);
    // draw_single_square(7, 4);

    draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
}


