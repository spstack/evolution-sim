/** ===============================================================================
 * File: main_macroquad.rs
 * Author: Scott Stack
 * Description: main application entry point for macroquad target version of the program
 * 
 * Features want to implement
 *  - generate walls in more interesting way (connected walls)
 *  - allow saving individual creatures
 * ===============================================================================*/
mod env_macroquad;
use macroquad::prelude::next_frame;


/// Main application entry point for Macroquad GUI interface
#[macroquad::main("Evolution Sim!")]
async fn main() {
    let mut m_env = env_macroquad::EnvMacroquad::new();

    // Start the visualization
    loop {
        m_env.main_loop_interactive_mode();
        next_frame().await
    }
}
