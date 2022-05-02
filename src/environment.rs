/** ===============================================================================
 * File: environment.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements environment features that the creature inhabits
 * ===============================================================================*/
use crate::creature::creature_v1;

const ENV_X_SIZE : usize = 25;
const ENV_Y_SIZE : usize = 25;
const NUM_TOTAL_SPACES : usize = ENV_X_SIZE * ENV_Y_SIZE;

/// Enumeration that defines the possible states 
enum SpaceStates {
    BlankSpace,                 // Space is blank
    // CreatureSpace(&Creature),   // Space has a creature in it
    FoodSpace,                  // Space has a food in it
}

struct EnvironmentV1 {

    creatures : [creature_v1::Creature; NUM_TOTAL_SPACES],
    positions : [SpaceStates; NUM_TOTAL_SPACES],

}

// fn initialize_environment() -> EnvironmentV1 {

// }



