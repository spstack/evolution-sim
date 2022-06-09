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
pub const DEBUG_LEVEL : usize = 2;

pub const ENV_X_SIZE : usize = 25;
pub const ENV_Y_SIZE : usize = 25;
pub const NUM_TOTAL_SPACES : usize = ENV_X_SIZE * ENV_Y_SIZE;

pub const MAX_NUM_CREATURES : usize = 20;

pub const NUM_START_CREATURES : usize = 10;
pub const NUM_START_FOOD : usize = 10;
pub const ENERGY_PER_FOOD_PIECE : usize = 20;

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

/// Structure representing a very simple 2-D environment
pub struct EnvironmentV1 {

    // Vector containing all creature instances
    pub creatures : Vec<CreatureV1>,

    // Contains the states of each space.
    pub positions : [[SpaceStates; ENV_Y_SIZE]; ENV_X_SIZE],

    // Represents the current time step in the sim
    time_step : usize,  
}


/// Implementation of EnvironmentV1
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

    fn handle_creature_action(&mut self, creature: &mut CreatureV1, action : CreatureActions) {
        let mut next_position : CreaturePosition = creature.position.clone();

        // Now handle the action
        match action {
            CreatureActions::MoveUp => {
                if next_position.y > 0 {
                    next_position.y -= 1;
                }
            },

            CreatureActions::MoveDown => {
                // Check if move would go beyond the bounds of this board
                if next_position.y < self.positions.len() - 1 {
                    next_position.y += 1;
                }
            },

            CreatureActions::MoveLeft => {
                if next_position.x > 0 {
                    next_position.x -= 1;
                }
            },

            CreatureActions::MoveRight => {
                // Check if move would go beyond the bounds of this board
                if next_position.x < self.positions[0].len() - 1 {
                    next_position.x += 1;
                }
            },

            _ => {
                println!("Unhandled action {:?}", action);
                next_position = creature.position.clone();
            }
        }

        // If there was an update to the position, check for collisions, food, etc...
        if next_position != creature.position {
            println!("Creature {} is moving to {}.{}", creature.id, next_position.x, next_position.y);

            match self.positions[next_position.x][next_position.y] {
                // If next space is blank, perform the move
                SpaceStates::BlankSpace => {
                    self.positions[creature.position.x][creature.position.y] = SpaceStates::BlankSpace;
                    self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                }

                // If next space is food, then eat it!
                SpaceStates::FoodSpace => {
                    self.positions[creature.position.x][creature.position.y] = SpaceStates::BlankSpace;
                    self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                    creature.eat_food(ENERGY_PER_FOOD_PIECE);
                }

                // Otherwise, do nothing...
                _ => {}
            }
            println!("moved...");
        }

    }

    /// Advance one "day"!
    pub fn advance_step(&mut self) {

        // Print some info about the env
        if DEBUG_LEVEL > 0 {
            println!("===================== STEP {} ===============", self.time_step);
            println!("Creatures: {}", self.creatures.len());
            println!("");
        }

        // Evaluate the next action for each creature
        for creature in &mut self.creatures {
            // First update the 'senses' of the creature
            creature.sense_surroundings();

            // Then actually evaluate the brain net to get the next action it'll take
            let action : CreatureActions = creature.perform_next_action();

            // Now handle the action
            let mut next_position : CreaturePosition = creature.position.clone();

            match action {
                CreatureActions::MoveUp => {
                    if next_position.y > 0 {
                        next_position.y -= 1;
                    }
                },

                CreatureActions::MoveDown => {
                    // Check if move would go beyond the bounds of this board
                    if next_position.y < self.positions.len() - 1 {
                        next_position.y += 1;
                    }
                },

                CreatureActions::MoveLeft => {
                    if next_position.x > 0 {
                        next_position.x -= 1;
                    }
                },

                CreatureActions::MoveRight => {
                    // Check if move would go beyond the bounds of this board
                    if next_position.x < self.positions[0].len() - 1 {
                        next_position.x += 1;
                    }
                },

                _ => {
                    println!("Unhandled action {:?}", action);
                    next_position = creature.position.clone();
                }
            }

            // If there was an update to the position, check for collisions, food, etc...
            if next_position != creature.position {
                println!("Creature {} is moving to {}.{}", creature.id, next_position.x, next_position.y);

                // Detect collisions in next space
                match self.positions[next_position.x][next_position.y] {
                    // If next space is blank, perform the move
                    SpaceStates::BlankSpace => {
                        self.positions[creature.position.x][creature.position.y] = SpaceStates::BlankSpace;
                        self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                        creature.set_position(next_position.x, next_position.y);
                    }

                    // If next space is food, then eat it!
                    SpaceStates::FoodSpace => {
                        self.positions[creature.position.x][creature.position.y] = SpaceStates::BlankSpace;
                        self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                        creature.eat_food(ENERGY_PER_FOOD_PIECE);
                        creature.set_position(next_position.x, next_position.y);
                    }

                    // Otherwise, do nothing...
                    _ => {}
                }
            }


        }

        
        // If proper debug level show the env after each step
        if DEBUG_LEVEL > 1 {
            self.show();
        }

        // Increment the time step counter
        self.time_step += 1;

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

