
/** ===============================================================================
 * File: main.rs
 * Author: Scott Stack
 * Description: main application entry point
 * 
 * Features want to implement
 *  - generate walls in more interesting way (connected walls)
 *  - allow saving individual creatures
 * ===============================================================================*/
mod linalg;
mod neural_net;
mod creature;
mod environment;
use environment::*;

// Includes depending on which visualization type we choose in the cargo.toml file
#[cfg(feature = "macroquad_vis")]
mod env_macroquad;
#[cfg(feature = "macroquad_vis")]
use macroquad::prelude::next_frame;
#[cfg(feature = "console_vis")]
mod env_console;



/// Main application entry point for Macroquad GUI interface
#[cfg(feature = "macroquad_vis")]
#[macroquad::main("Evolution Sim!")]
async fn main() {
    let mut m_env = env_macroquad::EnvMacroquad::new();

    // Start the visualization
    loop {
        m_env.main_loop_interactive_mode();
        next_frame().await
    }
}

/// Main function for command line sim visualization version
#[cfg(feature = "console_vis")]
fn main() {
    env_console::run_console_demo_mode();
}

