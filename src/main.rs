
/** ===============================================================================
 * File: main.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: main application entry point
 * ===============================================================================*/

mod creature;
mod environment;
use environment::*;
use std::io;



/// Main application entry point
fn main() {

    test_env_v1();

}


// Help string for the command line interface
const HELP_TEXT : &str = "
h = help
q = quit
d = display the current state of the environment
p <creature_id> = print stats for the given creature id
n = next step. Run one simulation step
";


/// Test of the V1 2d environment simulation
fn test_env_v1() {

    // Allocate the env
    let mut env = EnvironmentV1::new();

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
            "p" => env.print_creature(0), 
            "d" => env.show(),
            "n" => env.advance_step(),
            "q" => break,
            _ => println!("Invalid input {}", choice_str),
        }
    }
}
