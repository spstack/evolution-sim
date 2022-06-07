/** ===============================================================================
 * File: environment.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements environment features that the creature inhabits
 * ===============================================================================*/
use crate::creature::*;
use rand::Rng;

//===============================================================================
// CONSTANTS
//===============================================================================
pub const ENV_X_SIZE : usize = 25;
pub const ENV_Y_SIZE : usize = 25;
pub const NUM_TOTAL_SPACES : usize = ENV_X_SIZE * ENV_Y_SIZE;

pub const MAX_NUM_CREATURES : usize = 20;

pub const NUM_START_CREATURES : usize = 1;
pub const NUM_START_FOOD : usize = 10;

//===============================================================================
// Environment V1 Declarations
//===============================================================================

/// Enumeration that defines the possible states 
#[derive(Copy, Clone)]
pub enum SpaceStates {
    BlankSpace,                 // Space is blank
    CreatureSpace(usize),       // Space has a creature in it. The single argument represents the ID of the creature
    FoodSpace,                  // Space has a food in it
}

pub struct EnvironmentV1 {

    // Vector containing all creature instances
    pub creatures : Vec<CreatureV1>,

    // Contains the states of each space.
    pub positions : [[SpaceStates; ENV_Y_SIZE]; ENV_X_SIZE],

    time_step : usize,  // Represents the current time step in the sim
}


/// A very simple 2-D Environment
impl EnvironmentV1 {

    /// Constructor for new environment instance
    pub fn new() -> EnvironmentV1 {
        let mut rng = rand::thread_rng();

        // Initialize all positions to be blank at first
        let mut temp_positions = [[SpaceStates::BlankSpace; ENV_Y_SIZE]; ENV_X_SIZE];

        // Initialize creature vector
        let mut temp_creature_vec = Vec::<CreatureV1>::with_capacity(MAX_NUM_CREATURES);

        // Fill in random spaces with food
        for _food_num in 0..NUM_START_FOOD {
            let x = rng.gen_range(0..ENV_X_SIZE);
            let y = rng.gen_range(0..ENV_Y_SIZE);
            temp_positions[x][y] = SpaceStates::FoodSpace;
        }

        // Fill in random spaces with creatures (no worries if they overwrite food)
        for creature_num in 0..NUM_START_CREATURES {

            // Generate the random location of new creature
            let x = rng.gen_range(0..ENV_X_SIZE);
            let y = rng.gen_range(0..ENV_Y_SIZE);

            // Set the space to creature space
            temp_positions[x][y] = SpaceStates::CreatureSpace(creature_num);

            // Create new creature!
            temp_creature_vec.push(CreatureV1::new(creature_num));
            temp_creature_vec[creature_num].set_position(x, y);

        }


        // Return a new instance of the environment
        return EnvironmentV1 {
            creatures : temp_creature_vec,
            positions : temp_positions,
            time_step : 0,
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
        println!("Key:");
        println!("Creature = X\nFood = #");
    }


    /// Advance one "day"!
    pub fn advance_step(&mut self) {

        // Evaluate the next action for each creature
        for creature in &mut self.creatures {
            let action : CreatureActions = creature.perform_next_action();

            println!("Action = {:?}", action);

            // Now handle the action
            match action {
                CreatureActions::MoveUp => {
                    let [x, mut y] = creature.position;
                    self.positions[x][y] = SpaceStates::BlankSpace;
                    if y > 0 {
                        y -= 1;
                        creature.position[1] -= 1;
                    }
                    self.positions[x][y] = SpaceStates::CreatureSpace(creature.id);
                },

                CreatureActions::MoveDown => {
                    let [x, mut y] = creature.position;
                    self.positions[x][y] = SpaceStates::BlankSpace;
                    if y < ENV_Y_SIZE-1 {
                        y += 1;
                        creature.position[1] += 1;
                    }
                    self.positions[x][y] = SpaceStates::CreatureSpace(creature.id);
                },

                CreatureActions::MoveLeft => {
                    let [mut x, y] = creature.position;
                    self.positions[x][y] = SpaceStates::BlankSpace;
                    if x > 0 {
                        x -= 1;
                        creature.position[0] -= 1;
                    }
                    self.positions[x][y] = SpaceStates::CreatureSpace(creature.id);
                },

                CreatureActions::MoveRight => {
                    let [mut x, y] = creature.position;
                    self.positions[x][y] = SpaceStates::BlankSpace;
                    if x < ENV_X_SIZE-1 {
                        x += 1;
                        creature.position[0] += 1;
                    }
                    self.positions[x][y] = SpaceStates::CreatureSpace(creature.id);
                },

                _ => println!("Unhandled action {:?}", action),
            }
        }

        self.show();
    }


    /// Print status on a given creature
    pub fn print_creature(&self, id : usize) {

        // Check the bounds 
        if self.creatures.len() <= id {
            return;
        }

        self.creatures[id].brain.show();
    }
} 

