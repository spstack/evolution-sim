
/** ===============================================================================
 * File: main_led.rs
 * Author: Scott Stack
 * Description: main application entry point for LED matrix target version of the program
 * This version interfaces specifically with HUB75 matrix LED wall panels
 * ===============================================================================*/
mod hub75_led_driver;
mod librgbmatrix_defines;

use std::io;
use std::process;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use rand::Rng;
use core_lib::environment::*;
use core_lib::creature::*;
use hub75_led_driver::*;
use librgbmatrix_defines::Color;

const FOOD_SPACE_COLOR : Color = Color{r: 0, g: 200, b: 0};
const WALL_SPACE_COLOR : Color = Color{r: 200, g: 200, b: 200};
const FIGHT_SPACE_COLOR : Color = Color{r: 20, g: 0, b: 0};

const MAX_TIME_STEPS_PER_SIM : usize = 10000;
const STEP_TIME_DELAY : u64 = 250;

// Default parameters that the LED simulation visualization will start with
const DEFAULT_PARAMS : EnvironmentParams = EnvironmentParams {
    env_x_size : 64, // THIS MUST BE SIZE OF PANEL!
    env_y_size : 64, // THIS MUST BE SIZE OF PANEL!
    num_start_creatures : 100,  
    num_start_food : 150,
    num_start_walls : 250,
    energy_per_food_piece : DEFAULT_ENERGY_PER_FOOD_PIECE,
    energy_per_kill : DEFAULT_ENERGY_PER_KILL,
    max_offspring_per_reproduce : 2,
    mutation_prob : DEFAULT_MUTATION_PROB,
    avg_new_food_per_day : 1.0,
    creature_repro_energy_cost : DEFAULT_REPRODUCE_ENERGY_COST,
    creature_starting_energy : DEFAULT_ENERGY_LEVEL,
};


/// Main function for command line sim visualization version
fn main() {
    // Initialize the driver
    let mut driver = RGBLedMatrixDriver::new();

    // initialize the rng
    let mut rng = rand::thread_rng();

    // Setup a Ctrl-C handler to close the driver before quitting
    let ctrlc_triggered_flag = Arc::new(AtomicBool::new(false));
    let ctrlc_handler_flag = ctrlc_triggered_flag.clone();
    ctrlc::set_handler(move || {
        // All this just to set a flag...
        let _ = ctrlc_handler_flag.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |_| return Some(true));
        println!("Crtl-C received...exiting");
        // process::exit(0);
    }).expect("Error setting Ctrl-C handler");

    // Load a random default environment. This API allows selecting from a numbered list
    // of predefined environments that are at least a little interesting. Number zero
    // is always just totally random layout
    let starting_env_num = rng.gen_range(0..NUM_DEFAULT_ENVS);
    let mut env = EnvironmentV1::new_rand_from_default(&DEFAULT_PARAMS, Some(starting_env_num));

    // Run visualizations forever
    loop {
        // TODO: Figure out why we need to run this twice in a row right after en env is created...
        env.advance_step();

        // Run the simulation until there are no more creatures left!
        while env.num_creatures > 0 && env.time_step < MAX_TIME_STEPS_PER_SIM {
            // Advance one day
            env.advance_step();

            // Update LED panel
            display_env_on_led_panel(&env, &mut driver);

            // Wait a bit
            thread::sleep(core::time::Duration::from_millis(STEP_TIME_DELAY));

            // Check whether we should stop
            if ctrlc_triggered_flag.load(Ordering::Relaxed) {
                break;
            }
        }

        // Check again to break out of the outer loop
        if ctrlc_triggered_flag.load(Ordering::Relaxed) {
            break;
        }

        // Ok, so the sim has ended. Display a fading animation
        display_fade_out_animation(&mut driver, &env);

        // Ok, now start a new random simulation
        // This block here really only serves to generate a random environment, but with the chance
        // to generate a completely random environment by sending `None` as the default
        let tmp_env_num : usize = rng.gen_range(0..=NUM_DEFAULT_ENVS);
        let env_num : Option<usize>;
        if tmp_env_num >= NUM_DEFAULT_ENVS {
            env_num = None;
        } else {
            env_num = Some(tmp_env_num);
        }
        env = EnvironmentV1::new_rand_from_default(&DEFAULT_PARAMS, env_num);
        display_env_on_led_panel(&env, &mut driver);

        // Once the display is updated, slowly fade in
        display_fade_in_animation(&mut driver, &env);
    }
  
    // Make sure to close the driver and reset the hardware before quitting!
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



/// Display a fading animation that simply dims the display slowly
fn display_fade_out_animation(driver : &mut RGBLedMatrixDriver, env : &EnvironmentV1) {
    let initial_brightness = driver.get_matrix_brightness();
    if initial_brightness == 0 {
        println!("Error: can't perform fade out if display is already off...");
        return;
    }
    let num_steps: u8 = 50;
    let step_delay_ms : u64 = 100;
    let bright_decrease_per_step = initial_brightness / num_steps;

    // Decrease brightness step by step until we've done the proper number of steps
    let mut brightness = initial_brightness;
    for _loops in 0..num_steps {
        // Apply new brightness
        driver.set_matrix_brightness(brightness);

        // The way the driver works, brightness is only updated for new pixels, so we have to redraw the image
        display_env_on_led_panel(env, driver);

        // Drop brightness down
        brightness = brightness.saturating_sub(bright_decrease_per_step);
        thread::sleep(core::time::Duration::from_millis(step_delay_ms));
    }
}


/// Display a fade in animation that simply turns up the display brightness slowly
fn display_fade_in_animation(driver : &mut RGBLedMatrixDriver, env : &EnvironmentV1) {
    const TARGET_BRIGHTNESS : u8 = 100; // brightness is specified in percent
    let initial_brightness = driver.get_matrix_brightness();
    let num_steps: u8 = 50;
    let step_delay_ms : u64 = 100;
    let bright_increase_per_step = (TARGET_BRIGHTNESS - initial_brightness) / num_steps;

    // Decrease brightness step by step until we've done the proper number of steps
    let mut brightness = initial_brightness;
    for _loops in 0..num_steps {
        // Apply new brightness
        driver.set_matrix_brightness(brightness);

        // The way the driver works, brightness is only updated for new pixels, so we have to redraw the image
        display_env_on_led_panel(env, driver);

        // Increase brightness
        brightness = brightness.saturating_add(bright_increase_per_step);
        thread::sleep(core::time::Duration::from_millis(step_delay_ms));
    }
}