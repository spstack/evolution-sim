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
// pub const NUM_TOTAL_SPACES : usize = ENV_X_SIZE * ENV_Y_SIZE;

pub const MAX_NUM_CREATURES : usize = 50;

pub const NUM_START_CREATURES : usize = 30;
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

    // Number of total creatures created
    num_total_creatures : usize,
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
            num_total_creatures : NUM_START_CREATURES,
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
                    SpaceStates::CreatureSpace(id) => print!("{:2} ", id),
                    SpaceStates::FoodSpace => print!(" # "),
                }
            }
            print!("|");
            println!();
        }
        println!("-----------------------------------------------------------------------------");
        println!("Key:");
        println!("Creature = <id num>\nFood = #");
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

            let pos = creature.position.clone();

            match self.positions[next_position.x][next_position.y] {
                // If next space is blank, perform the move
                SpaceStates::BlankSpace => {
                    self.positions[pos.x][pos.y] = SpaceStates::BlankSpace;
                    self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                }

                // If next space is food, then eat it!
                SpaceStates::FoodSpace => {
                    self.positions[pos.x][pos.y] = SpaceStates::BlankSpace;
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

        // Create a temporary variable to hold new creatures that will spawn
        let mut temp_new_creatures : Vec<CreatureV1> = Vec::new();

        // Evaluate the next action for each creature
        for creature_idx in 0..self.creatures.len() {
            let creature = &mut self.creatures[creature_idx];

            // First update the 'senses' of the creature
            creature.sense_surroundings();

            // Then actually evaluate the brain net to get the next action it'll take
            let action : CreatureActions = creature.perform_next_action();

            // if the creature is dead, don't bother handling the next action. Will be removed
            if creature.is_dead() {
                if DEBUG_LEVEL > 2 {
                    println!("Creature {} is ded... :( | age = {}", creature.id, creature.age);
                }
                continue;
            }

            if DEBUG_LEVEL > 0 {
                println!("Next Action for creature {} is {:?} | age = {} | energy = {}", creature.id, action, creature.age, creature.energy);
            }

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

                CreatureActions::Reproduce => {
                    println!("Creature {} is reproducing!", creature.id);
                    let mut new_offspring = CreatureV1::new_offspring(self.num_total_creatures, &creature);
                    self.num_total_creatures += 1;
                    temp_new_creatures.push(new_offspring);
                },

                CreatureActions::Stay => {}

                _ => {
                    println!("Unhandled action {:?}", action);
                    next_position = creature.position.clone();
                }
            }

            // If there was an update to the position, check for collisions, food, etc...
            if next_position != creature.position {
                println!("Creature {} is moving to {}.{}", creature.id, next_position.x, next_position.y);

                let pos = creature.position.clone();

                // Detect collisions in next space
                match self.positions[next_position.x][next_position.y] {
                    // If next space is blank, perform the move
                    SpaceStates::BlankSpace => {
                        self.positions[pos.x][pos.y] = SpaceStates::BlankSpace;
                        self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                        creature.set_position(next_position.x, next_position.y);
                    }

                    // If next space is food, then eat it!
                    SpaceStates::FoodSpace => {
                        self.positions[pos.x][pos.y] = SpaceStates::BlankSpace;
                        self.positions[next_position.x][next_position.y] = SpaceStates::CreatureSpace(creature.id);
                        creature.eat_food(ENERGY_PER_FOOD_PIECE);
                        creature.set_position(next_position.x, next_position.y);
                    }

                    // Otherwise, do nothing...
                    _ => {}
                }
            }

        } // end loop updating creatures


        // Remove dead creatures from the environment
        self.remove_dead_creatures();

        // Add new spawned creatures
        for new_creature in temp_new_creatures {
            self.positions[new_creature.position.x][new_creature.position.y] = SpaceStates::CreatureSpace(new_creature.id);
            self.creatures.push(new_creature);
        }
        
        // If proper debug level show the env after each step
        if DEBUG_LEVEL > 1 {
            self.show();
        }

        // Increment the time step counter
        self.time_step += 1;

    }

    /// Go through list of creatures and remove the ones that have died from the environment
    fn remove_dead_creatures(&mut self) {
        let mut to_remove : Vec<usize> = Vec::new(); // vector if indices to remove

        // Loop through each creature in the environment
        for creature_idx in 0..self.creatures.len() {
            let creature = &self.creatures[creature_idx];
            if creature.is_dead() {
                let pos = creature.position.clone();
  
                // Update the position map to remove this creature
                self.positions[pos.x][pos.y] = SpaceStates::BlankSpace;

                // Mark this dude for removal
                to_remove.push(creature.id);
            }
        }
        
        // Remove the specified IDs from the list
        for remove_id in to_remove {
            for x in 0..self.creatures.len() {
                if self.creatures[x].id == remove_id {
                    self.creatures.remove(x);
                    break;
                }
            }
        }
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

