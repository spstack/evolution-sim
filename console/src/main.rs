
/** ===============================================================================
 * File: main_console.rs
 * Author: Scott Stack
 * Description: main application entry point for console target version of the program
 * ===============================================================================*/
mod env_console;
use std::process;

/// Main function for command line sim visualization version
fn main() {
    ctrlc::set_handler(move || {
        println!("Crtl-C received...exiting");
        process::exit(0);
    }).expect("Error setting Ctrl-C handler");


    env_console::run_console_demo_mode();
}

