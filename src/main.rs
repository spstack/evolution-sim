
/** ===============================================================================
 * File: main.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: main application entry point
 * ===============================================================================*/
mod linalg;
mod neural_net;
mod creature;
mod environment;
mod env_macroquad;
use environment::*;
use std::io;
use std::{thread, time::Duration};
use macroquad::prelude::*;


/// Main application entry point for macroquad testing. Unfortunately, this has to be done here
#[macroquad::main("Evolution Sim!")]
async fn main() {
    let mut env = env_macroquad::EnvMacroquad::new();

    // Start the visualization
    loop {
        env.main_loop();
        next_frame().await
    }
}

/// Main function for command line sim visualization
// fn main() {
//     test_env_v1();
// }

// Help string for the command line interface
const HELP_TEXT : &str = "
h = help
q = quit
d = display the current state of the environment
p = print stats for all creatures that are alive
n = next step. Run one simulation step
r = run until no creatures left
";


/// Test of the V1 2d environment simulation
fn test_env_v1() {

    // Allocate the env
    let mut env = EnvironmentV1::new_rand(40, 40, 20, 20, 5);

    // Show initial state
    env.show();

    loop {

        // Prompt for what next action should be
        println!("Action (h for help): ");
        let mut choice = String::new();
        let res = io::stdin().read_line(&mut choice);
        match res {
            Err(e) => {
                println!("Error getting input...{}", e);
                continue;
            }
            Ok(_num_chars) => {},
        }

        // Successfully read a line, handle input!
        let choice_str = choice.trim();
        match choice_str {
            "h" => println!("{}", HELP_TEXT),
            "p" => env.show_all_creature_info(), 
            "d" => env.show(),
            "n" => env.advance_step(),
            "r" => run_full_sim(&mut env),
            "q" => break,
            _ => println!("Invalid input {}", choice_str),
        }
    }
}


/// Run full simulation until there's no more creatures left
fn run_full_sim(env : &mut EnvironmentV1) {

    while env.creatures.len() > 0 {
        // Run a sim step
        env.advance_step();

        // wait a bit
        thread::sleep(Duration::from_millis(500));

    }
}

