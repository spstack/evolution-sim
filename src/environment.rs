/** ===============================================================================
 * File: environment.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements environment features that the creature inhabits
 * ===============================================================================*/
use crate::creature::creature_v1;
use rand::Rng;

//===============================================================================
// CONSTANTS
//===============================================================================
pub const ENV_X_SIZE : usize = 25;
pub const ENV_Y_SIZE : usize = 25;
pub const NUM_TOTAL_SPACES : usize = ENV_X_SIZE * ENV_Y_SIZE;


//===============================================================================
// Environment V1 Declarations
//===============================================================================

/// Enumeration that defines the possible states 
#[derive(Copy, Clone)]
pub enum SpaceStates {
    BlankSpace,                 // Space is blank
    CreatureSpace(usize),       // Space has a creature in it
    FoodSpace,                  // Space has a food in it
}

pub struct EnvironmentV1 {

    // Allocate an array with enough space to hold all possible creatures
    // pub creatures : [creature_v1::Creature; NUM_TOTAL_SPACES],

    // Contains the states of each space.
    pub positions : [[SpaceStates; ENV_Y_SIZE]; ENV_X_SIZE],

}


impl EnvironmentV1 {

    /// Constructor
    pub fn new() -> EnvironmentV1 {
        let mut rng = rand::thread_rng();

        // let mut temp_creatures : [creature_v1::Creature; NUM_TOTAL_SPACES];
        let mut temp_positions = [[SpaceStates::BlankSpace; ENV_Y_SIZE]; ENV_X_SIZE];

        // Fill in random spaces with food
        for _food_num in 0..10 {
            let x = rng.gen_range(0..ENV_X_SIZE);
            let y = rng.gen_range(0..ENV_Y_SIZE);
            temp_positions[x][y] = SpaceStates::FoodSpace;
        }

        // Fill in random spaces with creatures (no worries if they overwrite food)
        for creature_num in 0..10 {
            let x = rng.gen_range(0..ENV_X_SIZE);
            let y = rng.gen_range(0..ENV_Y_SIZE);
            temp_positions[x][y] = SpaceStates::CreatureSpace(creature_num);
        }


        // Return a new instance of the environment
        return EnvironmentV1 {
            // creatures : temp_creatures,
            positions : temp_positions,
        }
    }


    /// Print the current state of the environment board
    pub fn show(&self) {
        println!();
        println!("-----------------------------------------------------------------------------");
        for y in 0..ENV_Y_SIZE {
            print!("|");
            for x in 0..ENV_X_SIZE {
                match self.positions[x][y] {
                    SpaceStates::BlankSpace => print!("   "),
                    SpaceStates::CreatureSpace(_id) => print!(" X "),
                    SpaceStates::FoodSpace => print!(" # "),
                }
            }
            print!("|");
            println!();
        }
        println!("-----------------------------------------------------------------------------");
    }
} 

