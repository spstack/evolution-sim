
/** ===============================================================================
 * File: main_led.rs
 * Author: Scott Stack
 * Description: main application entry point for LED matrix target version of the program
 * This version interfaces specifically with HUB75 matrix LED wall panels
 * ===============================================================================*/
mod hub75_led_driver;
mod librgbmatrix_defines;

use std::io;
use std::thread;
use core_lib::environment::*;
use core_lib::creature::*;
use hub75_led_driver::*;
use librgbmatrix_defines::Color;

const FOOD_SPACE_COLOR : Color = Color{r: 0, g: 200, b: 0};
const WALL_SPACE_COLOR : Color = Color{r: 200, g: 200, b: 200};
const FIGHT_SPACE_COLOR : Color = Color{r: 20, g: 0, b: 0};

const STEP_TIME_DELAY : u64 = 250;

// Default parameters that the LED simulation visualization will start with
const DEFAULT_CONSOLE_PARAMS : EnvironmentParams = EnvironmentParams {
    env_x_size : 64, // THIS MUST BE SIZE OF PANEL!
    env_y_size : 64, // THIS MUST BE SIZE OF PANEL!
    num_start_creatures : 100,  
    num_start_food : 150,
    num_start_walls : 200,
    energy_per_food_piece : DEFAULT_ENERGY_PER_FOOD_PIECE,
    energy_per_kill : DEFAULT_ENERGY_PER_KILL,
    max_offspring_per_reproduce : DEFAULT_OFFSPRING_PER_REPRODUCE,
    mutation_prob : DEFAULT_MUTATION_PROB,
    avg_new_food_per_day : NEW_FOOD_PIECES_PER_STEP, 
    creature_repro_energy_cost : DEFAULT_REPRODUCE_ENERGY_COST,
    creature_starting_energy : DEFAULT_ENERGY_LEVEL,
};

const DEFAULT_JSON_FILE : &str = "data/default_env1.json";
const DEFAULT_INITIAL_JSON_LOAD_OPTS : JsonEnvLoadParams = JsonEnvLoadParams {
    load_all : false,
    load_parameters : false,
    load_creatures : false,
    load_walls : true,  // just load walls
    load_food : false,
};


/// Main function for command line sim visualization version
fn main() {
    // Initialize the driver
    let mut driver = RGBLedMatrixDriver::new();

    // Only do a set number of sim steps for now
    let mut env = EnvironmentV1::new_rand_from_default(&DEFAULT_CONSOLE_PARAMS, 4);

    // Load a default wall configuration to make it more interesting
    env.load_from_json(DEFAULT_JSON_FILE, &DEFAULT_INITIAL_JSON_LOAD_OPTS);

    // Run one initial step
    env.advance_step();

    for _step in 0..50 {
        // Advance one day
        env.advance_step();

        // Update LED panel
        display_env_on_led_panel(&env, &mut driver);

        // Wait a bit
        thread::sleep(core::time::Duration::from_millis(STEP_TIME_DELAY));

    }


    // Load another default image
    env = EnvironmentV1::new_rand_from_default(&DEFAULT_CONSOLE_PARAMS, 2);
    for _step in 0..50 {
        // Advance one day
        env.advance_step();

        // Update LED panel
        display_env_on_led_panel(&env, &mut driver);

        // Wait a bit
        thread::sleep(core::time::Duration::from_millis(STEP_TIME_DELAY));
    }
    // println!("Press enter to exit...");
    // let mut choice = String::new();
    // let res = io::stdin().read_line(&mut choice).unwrap();

    driver.close();
}



/// Function to take an environment and display it's existing state
fn display_env_on_led_panel(env : &EnvironmentV1, driver : &mut RGBLedMatrixDriver) {
    // clear everything first
    driver.clear_buffered_screen();

    // Loop through the env and set each pixel to proper color
    for y_usize in 0..env.params.env_y_size {
        for x_usize in 0..env.params.env_x_size {
            let x = x_usize as i32;
            let y = y_usize as i32;
            match env.positions[x_usize][y_usize] {
                SpaceStates::BlankSpace => (), // no need to update this pixel
                SpaceStates::CreatureSpace(id) => {
                    let idx = env.get_creature_idx_from_id(id).unwrap();
                    let creature_color_env = env.creatures[idx].color;
                    let creature_color = Color {
                        r: creature_color_env.red,
                        g: creature_color_env.green,
                        b: creature_color_env.blue,
                    };
                    driver.set_buffered_pixel(x, y, creature_color);
                }
                SpaceStates::FoodSpace => driver.set_buffered_pixel(x, y, FOOD_SPACE_COLOR),
                SpaceStates::WallSpace => driver.set_buffered_pixel(x, y, WALL_SPACE_COLOR),
                SpaceStates::FightSpace(_ttl) => driver.set_buffered_pixel(x, y, FIGHT_SPACE_COLOR),
            }
        }
    }

    // Actually perform the buffer swap and display the image
    driver.apply_buffered_frame();
}