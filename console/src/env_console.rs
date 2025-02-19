/**
 * @file 
 * @author - Scott Stack
 * @description - Implements console display functions for the environments.
 */
use std::thread;
use std::time;
use rand::Rng;
use core_lib::environment::*;
use core_lib::creature::*;
use std::io;

// Constant escape sequences for visualization
const PREVIOUS_LINE_ESAPE_SEQ : &str = "\x1B[F";
const CLEAR_SCREEN_ESCAPE_SEQ : &str = "\x1B[1J";

// characters to be printed for each space type
const CREATURE_PRINT_CHAR : &str = "O";
const WALL_PRINT_CHAR : &str = "▮";
const FOOD_PRINT_CHAR : &str = "+";
const FIGHT_PRINT_CHAR : &str = "☠";


// Default parameters that the console simulation visualization will start with
const DEFAULT_CONSOLE_PARAMS : EnvironmentParams = EnvironmentParams {
    env_x_size : 64,
    env_y_size : 64,
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

/// Run an environment simulation that infinitely runs a bunch of simulations for
/// demonstration purposes
pub fn run_console_demo_mode() {
    let mut rng = rand::thread_rng();

    let tmp_env_num : usize = rng.gen_range(0..=NUM_DEFAULT_ENVS);
    let env_num : Option<usize>;
    if tmp_env_num >= NUM_DEFAULT_ENVS {
        env_num = None;
    } else {
        env_num = Some(tmp_env_num);
    }
    let mut env = Environment::new_rand_from_default(&DEFAULT_CONSOLE_PARAMS, env_num);

    // Run one initial step
    env.advance_step();

    while env.num_creatures > 0 {
        env.advance_step();
        show_env(&env);

        // Wait a bit
        thread::sleep(time::Duration::from_millis(500));

        // Go back number of lines to 'clear' the previous version of the board from the display
        let num_extra_print_lines: usize = 8;
        for _nline in 0..(env.params.env_y_size + num_extra_print_lines) {
            print!("{}", PREVIOUS_LINE_ESAPE_SEQ);
        }
        print!("{}", CLEAR_SCREEN_ESCAPE_SEQ);
    }
}


/// Print the current state of the environment board
pub fn show_env(env : &Environment) {
    println!();
    let num_dashes = env.params.env_x_size * 3 + 1;
    println!("{:-<width$}", " ", width = num_dashes); // print horizontal dashes
    for y in 0..env.params.env_y_size {
        print!("|");
        for x in 0..env.params.env_x_size {
            match env.positions[x][y] {
                SpaceStates::BlankSpace => print!("   "),
                SpaceStates::CreatureSpace(_id) => print!(" {} ", CREATURE_PRINT_CHAR),
                SpaceStates::FoodSpace => print!(" {} ", FOOD_PRINT_CHAR),
                SpaceStates::WallSpace => print!(" {} ", WALL_PRINT_CHAR),
                SpaceStates::FightSpace(_ttl) => print!(" {} ", FIGHT_PRINT_CHAR),
            }
        }
        print!("|");
        println!();
    }
    println!("{:-<width$}", " ", width = num_dashes); // print horizontal dashes
    println!("Key:");
    println!("Creature = {}", CREATURE_PRINT_CHAR);
    println!("Food = {}", FOOD_PRINT_CHAR);
    println!("Wall = {}", WALL_PRINT_CHAR);
    println!("FightSpace = {}", FIGHT_PRINT_CHAR);
}



//================= WORK IN PROGRESS BELOW =================


// Help string for the command line interface
#[allow(dead_code)]
const HELP_TEXT : &str = "
h = help
q = quit
d = display the current state of the environment
p = print stats for all creatures that are alive
n = next step. Run one simulation step
r = run until no creatures left
";


/// Run an interactive version of the console program
/// TODO: Not implemented fully...
#[allow(dead_code)]
fn interactive_console_mode() {

    // Allocate the env
    let params = EnvironmentParams::new();
    let mut env = Environment::new_rand(&params);

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
#[allow(dead_code)]
fn run_full_sim(env : &mut Environment) {

    while env.creatures.len() > 0 {
        // Run a sim step
        env.advance_step();

        // wait a bit
        thread::sleep(time::Duration::from_millis(500));

    }
}

