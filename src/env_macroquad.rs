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
pub const DEBUG_LEVEL : usize = 0;

const SCREEN_SIZE_X : f32 = 800.0;
const SCREEN_SIZE_Y : f32 = 800.0;

const NUM_GRID_SQUARES_X : usize = 100;
const NUM_GRID_SQUARES_Y : usize = 100;

const STATS_PANEL_WIDTH : f32 = 400.0;

//===============================================================================
// DATA
//===============================================================================

/// Constant parameters for this simulation that are passed into the main function
struct SimParameters {
    grid_x_size : f32,              // X size of a single grid square in pixels
    grid_y_size : f32,              // Y size of a single grid square in pixels
}


/// Environment
pub struct EnvMacroquad {
    pub params : SimParameters,     // Constant values that sim is initialized with
    pub env : EnvironmentV1,        // Contains the whole environment

    stats_panel_x_pos : f32,
    stats_panel_y_pos : f32,

    screen_width_pixels : f32,      // X Size of the environment in pixels
    screen_height_pixels : f32,     // Y size of the environment in pixels

}


//===============================================================================
// FUNCTIONS
//===============================================================================

impl EnvMacroquad {

    /// Get a new instance of the Macroquad environment
    pub fn new() -> EnvMacroquad {
        let temp_screen_size_x : f32 = SCREEN_SIZE_X + STATS_PANEL_WIDTH;

        // First set the screen size to default. Include the size of the stats panel
        request_new_screen_size(temp_screen_size_x, SCREEN_SIZE_Y);

        return EnvMacroquad {
            params : SimParameters {
                grid_x_size : SCREEN_SIZE_X / NUM_GRID_SQUARES_X as f32,
                grid_y_size : SCREEN_SIZE_Y / NUM_GRID_SQUARES_Y as f32,
            },
            env : EnvironmentV1::new_rand(
                NUM_GRID_SQUARES_X, // env_x_size
                NUM_GRID_SQUARES_Y, // env_y_size
                100, // num_start_creatures
                100, // num_start_food
                20, // num_walls
            ),

            // Set position of stats panel
            stats_panel_x_pos : SCREEN_SIZE_X + 10.0,
            stats_panel_y_pos : 0.0,

            // Set total size of the window for internal tracking
            screen_width_pixels : temp_screen_size_x,
            screen_height_pixels : SCREEN_SIZE_Y,
        }
    }

    /// Run and display the next step of the simulation
    pub fn run_next_step(&mut self) {
        self.env.advance_step();

        // Print out status of creatures per step
        if DEBUG_LEVEL > 0 {
            self.env.show_all_creature_info()
        }
    }

    /// Update the simulation env board
    fn update_sim_display(&self) {

        // For each simulation space on the board, update with proper piece
        for x in 0..self.env.env_x_size {
            for y in 0..self.env.env_y_size {
                match self.env.positions[x][y] {
                    SpaceStates::CreatureSpace(id) => self.draw_creature_square(x, y),
                    SpaceStates::FoodSpace => self.draw_food_space(x, y),
                    SpaceStates::WallSpace => self.draw_wall_space(x, y),
                    _ => (),
                }
            }
        }
    }

    /// Update the statistics panel
    fn update_stats_panel(&self) {
        const header_font_size_px : f32 = 14.0;
        const main_font_size_px : f32 = 10.0;
        let mut cur_y_pos_px = self.stats_panel_y_pos + header_font_size_px;

        // Write the header
        let header_str = format!("{:12} {:12} {:12} {:15} ", "Creature Id", "Age", "Energy", "Last Action");
        draw_text(&header_str, self.stats_panel_x_pos, cur_y_pos_px, header_font_size_px, BLACK);
        cur_y_pos_px += header_font_size_px;

        for creature_idx in 0..self.env.creatures.len() {
            let creature = &self.env.creatures[creature_idx];
            let creature_str = format!("{:<12} {:<12} {:<12} {:<15?} ", creature.id, creature.age, creature.energy, creature.last_action);
            draw_text(&creature_str, self.stats_panel_x_pos, cur_y_pos_px, header_font_size_px, DARKGRAY);
            cur_y_pos_px += main_font_size_px;
        }
    }
        

    /// Update the display
    pub fn update_display(&self) {
        clear_background(GRAY);

        // Update the main board
        self.update_sim_display();

        // Update statistics on the side
        self.update_stats_panel(); 

    }

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
        draw_rectangle((x_pos as f32) * self.params.grid_x_size, (y_pos as f32) * self.params.grid_y_size, self.params.grid_x_size, self.params.grid_y_size, BLACK);
    }


}


/// Test function to draw various shapes
pub fn run_test() {
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


