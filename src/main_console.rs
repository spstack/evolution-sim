
/** ===============================================================================
 * File: main_console.rs
 * Author: Scott Stack
 * Description: main application entry point for console target version of the program
 * ===============================================================================*/
mod linalg;
mod neural_net;
mod creature;
mod environment;
mod env_console;
use crate::environment::*;

/// Main function for command line sim visualization version
fn main() {
    env_console::run_console_demo_mode();
}

