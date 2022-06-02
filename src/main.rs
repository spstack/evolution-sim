
/** ===============================================================================
 * File: main.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: main application entry point
 * ===============================================================================*/

mod creature;
mod environment;
// mod matrix_math;
use creature::creature_v1::*;
use environment::*;



/// Main application entry point
fn main() {

    // Spawn one creature
    let mut creature = Creature::new(1);
    creature.brain.show();
    creature.perform_next_action();
    creature.brain.show();

    // Ok, now test the environment
    let env = EnvironmentV1::new();
    env.show();

}
