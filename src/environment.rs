use std::path::absolute;

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
pub const DEBUG_LEVEL : usize = 0;

pub const DEFAULT_ENERGY_PER_FOOD_PIECE : usize = 20;
pub const DEFAULT_OFFSPRING_PER_REPRODUCE : usize = 10;
pub const DEFAULT_MUTATION_PROB : f32 = 0.05;

pub const MAX_CREATURE_VIEW_DISTANCE : isize = 5;       // Defines max number of spaces a creature can "see"
pub const FOOD_SPACE_COLOR : [u8; 3] = [255, 0, 0];     // color of food space
pub const WALL_SPACE_COLOR : [u8; 3] = [0, 0, 0];       // color of wall space


//===============================================================================
// Environment V1 Declarations
//===============================================================================

/// Enumeration that defines the possible states 
#[derive(Copy, Clone, PartialEq)]
pub enum SpaceStates {
    BlankSpace,                 // Space is blank
    CreatureSpace(usize),       // Space has a creature in it. The single argument represents the ID of the creature
    FoodSpace,                  // Space has a food in it
    WallSpace,                  // Space that contains a wall
}


/// Structure representing a very simple 2-D environment
pub struct EnvironmentV1 {
    // Parameters
    pub env_x_size : usize,                 // X size of the sim in "spaces"
    pub env_y_size : usize,                 // Y size of the sim in "spaces"
    num_start_creatures : usize,            // Number of creatures to start the sim with
    num_start_food : usize,                 // Number of starting food spaces
    energy_per_food_piece : usize,          // Number of energy units that will be given per food consumed 
    max_offspring_per_reproduce : usize,    // Maximum number of offspring that will be produced by one reproduction event
    mutation_prob : f32,                    // Probability that a single value in the creatures DNA will randomly mutate upon reproduction

    // Current state
    pub creatures : Vec<CreatureV1>,// Vector containing all creature instances
    pub positions : Vec<Vec<SpaceStates>>, //[[SpaceStates; MAX_ENV_Y_SIZE]; MAX_ENV_X_SIZE], // Contains the states of each space.
    time_step : usize,              // Represents the current time step in the sim
    num_total_creatures : usize,    // Number of total creatures created throughout sim
}


/// Implementation of EnvironmentV1
impl EnvironmentV1 {

    /// Constructor for new environment instance that's randomly populated
    pub fn new_rand(env_x_size : usize, env_y_size : usize, num_start_creatures: usize, num_start_food : usize, num_walls : usize) -> EnvironmentV1 {
        let mut rng = rand::thread_rng();

        // Initialize all positions to be blank at first
        let mut temp_positions = vec![vec![SpaceStates::BlankSpace; env_y_size]; env_x_size];

        // Initialize creature vector
        let mut temp_creature_vec = Vec::<CreatureV1>::with_capacity(num_start_creatures);

        // Fill in random spaces with food
        for _food_num in 0..num_start_food {
            let x = rng.gen_range(0..env_x_size);
            let y = rng.gen_range(0..env_y_size);
            temp_positions[x][y] = SpaceStates::FoodSpace;
        }

        // Fill in random spaces with creatures (no worries if they overwrite food)
        for creature_num in 0..num_start_creatures {

            // Generate the random location of new creature
            let x = rng.gen_range(0..env_x_size);
            let y = rng.gen_range(0..env_y_size);

            // Set the space to creature space
            temp_positions[x][y] = SpaceStates::CreatureSpace(creature_num);

            // Create new creature!
            temp_creature_vec.push(CreatureV1::new(creature_num));
            temp_creature_vec[creature_num].set_position(x, y);

        }

        // Fill random wall spaces
        for _wall_num in 0..num_walls {
            let mut done : bool = false;
            while !done {
                let x = rng.gen_range(0..env_x_size);
                let y = rng.gen_range(0..env_y_size);

                // Only allow overwriting of blank spaces
                match temp_positions[x][y] {
                    SpaceStates::BlankSpace => {
                        temp_positions[x][y] = SpaceStates::WallSpace;
                        done = true;
                    },
                    _ => {},
                }
            }
        }

        // Return a new instance of the environment
        return EnvironmentV1 {
            env_x_size : env_x_size,
            env_y_size : env_y_size,
            num_start_creatures : num_start_creatures,  
            num_start_food : num_start_food,
            energy_per_food_piece : DEFAULT_ENERGY_PER_FOOD_PIECE,
            max_offspring_per_reproduce : DEFAULT_OFFSPRING_PER_REPRODUCE,
            mutation_prob : DEFAULT_MUTATION_PROB,
            creatures : temp_creature_vec,
            positions : temp_positions,
            time_step : 0,
            num_total_creatures : num_start_creatures,
        }
    }


    /// Print the current state of the environment board
    pub fn show(&self) {
        println!();
        let num_dashes = self.env_x_size * 3 + 1;
        println!("{:-<width$}", " ", width = num_dashes); // print horizontal dashes
        for y in 0..self.env_y_size {
            print!("|");
            for x in 0..self.env_x_size {
                match self.positions[x][y] {
                    SpaceStates::BlankSpace => print!("   "),
                    SpaceStates::CreatureSpace(id) => print!("{:2} ", id),
                    SpaceStates::FoodSpace => print!(" # "),
                    SpaceStates::WallSpace => print!("|-|"),
                }
            }
            print!("|");
            println!();
        }
        println!("{:-<width$}", " ", width = num_dashes); // print horizontal dashes
        println!("Key:");
        println!("Creature = <id num>\nFood = #");
    }

    /// Print all creature info in columns to stdout
    pub fn show_all_creature_info(&self) {
        println!("{:12} {:12} {:12} {:15} ", "Creature Id", "Age", "Energy", "Last Action");
        for creature_idx in 0..self.creatures.len() {
            let creature = &self.creatures[creature_idx];
            println!("{:<12} {:<12} {:<12} {:<15?} ", creature.id, creature.age, creature.energy, creature.last_action);
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

        // Initialize the random number generator used in this function
        let mut rng = rand::thread_rng();

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

            if DEBUG_LEVEL > 1 {
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
                    // Randomly determine how many offspring this creature will have
                    let num_offspring = rng.gen_range(1..=self.max_offspring_per_reproduce);
                    if DEBUG_LEVEL > 1 {
                        println!("Creature {} is reproducing with {} offspring!", creature.id, num_offspring);
                    }
                    for _offspring_num in 0..num_offspring {
                        let new_offspring = CreatureV1::new_offspring(self.num_total_creatures, &creature, self.mutation_prob);
                        self.num_total_creatures += 1;
                        temp_new_creatures.push(new_offspring);
                    }
                },

                // Actions that don't require any further processing
                CreatureActions::Stay => {},
                CreatureActions::RotateCCW => {}, // handled inside creature code
                CreatureActions::RotateCW => {}, // handled inside creature code
            }

            // If there was an update to the position, check for collisions, food, etc...
            if next_position != creature.position {
                if DEBUG_LEVEL > 1 {
                    println!("Creature {} is moving to {}.{}", creature.id, next_position.x, next_position.y);
                }

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
                        creature.eat_food(self.energy_per_food_piece);
                        creature.set_position(next_position.x, next_position.y);
                    }

                    // If space is wall, then move is invalid. Stay put
                    SpaceStates::WallSpace => {}

                    // Otherwise, do nothing...
                    _ => {}
                }
            }
        } // end loop updating creatures


        // Remove dead creatures from the environment
        self.remove_dead_creatures();

        // Add new spawned creatures
        for mut new_creature in temp_new_creatures {
            let new_x_pos = rng.gen_range(0..self.env_x_size); //(new_creature.position.x + 1) % self.env_x_size;
            let new_y_pos = rng.gen_range(0..self.env_y_size);// (new_creature.position.y + 1) % self.env_y_size;
            // let new_y_pos = rng.gen_range(0..3) % self.env_y_size;
            new_creature.set_position(new_x_pos, new_y_pos);
            self.positions[new_creature.position.x][new_creature.position.y] = SpaceStates::CreatureSpace(new_creature.id);
            self.creatures.push(new_creature);
        }

        // Evaluate the vision of each of the creatures now that everything is updated
        self.update_creature_vision();
        
        // If proper debug level show the env after each step
        if DEBUG_LEVEL > 0 {
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

    /// Update what each of the creatures is currently "seeing"
    fn update_creature_vision(&mut self) {
        for creature in &mut self.creatures {
            // Define variables for position we will be looking in
            let mut xpos = creature.position.x;
            let mut ypos = creature.position.y;

            for _step in 0..MAX_CREATURE_VIEW_DISTANCE {
                // Update the position we're currently looking in by checking the direction creature is facing
                match creature.orientation {
                    CreatureOrientation::Up => {
                        if ypos == 0 {
                            break;
                        }
                        ypos -= 1;
                    },
                    CreatureOrientation::Down => ypos += 1,
                    CreatureOrientation::Right => xpos += 1,
                    CreatureOrientation::Left => {
                        if xpos == 0 {
                            break;
                        }
                        xpos -= 1;
                    },
                }

                // Check the bounds - if we're at the end or wrapped around, there's nothing to see. Return
                if (xpos >= self.env_x_size) || (ypos >= self.env_y_size) {
                    break;
                }

                // This is super ugly, but just takes the x and y distances and adds them
                let distance : usize = ((xpos as i32 - creature.position.x as i32).abs() + (ypos as i32 - creature.position.y as i32).abs()) as usize;

                // Check what type space is there
                match self.positions[xpos][ypos] {
                    SpaceStates::BlankSpace => {},

                    // Food space is in view
                    SpaceStates::FoodSpace => {
                        let vis : CreatureVisionState = CreatureVisionState {
                            obj_in_view : true,
                            dist : distance,
                            color : CreatureColor::new_from_vec(FOOD_SPACE_COLOR)
                        };
                        creature.set_vision(vis);
                    },

                    // Wall space in view
                    SpaceStates::WallSpace => {
                        let vis : CreatureVisionState = CreatureVisionState {
                            obj_in_view : true,
                            dist : distance,
                            color : CreatureColor::new_from_vec(WALL_SPACE_COLOR)
                        };
                        creature.set_vision(vis);
                    },

                    // Another creature is in view, update the color with the creature's color
                    SpaceStates::CreatureSpace(c_id) => {
                        // let c_idx = self.get_creature_idx_from_id(c_id).unwrap();
                        // let tgt_creature_color = self.creatures[c_idx].color.get_as_vec();
                        let tgt_creature_color = [0,0,255]; // TODO: fix borrow checker error when above is uncommented
                        let vis : CreatureVisionState = CreatureVisionState {
                            obj_in_view : true,
                            dist : distance,
                            color : CreatureColor::new_from_vec(tgt_creature_color)
                        };
                        creature.set_vision(vis);
                    }
                }
            }
        }



    }


    /// Get the index of the creature into the self.creatures array from creature ID
    pub fn get_creature_idx_from_id(&self, creature_id : usize) -> Result<usize, &str> {
        for creature_idx in 0..self.creatures.len() {
            if self.creatures[creature_idx].id == creature_id {
                return Ok(creature_idx);
            }
        }
        return Err("Invalid creature id");
    }

    /// Print status on a given creature
    pub fn print_creature(&self, id : usize) {

        // Check the bounds 
        if self.creatures.len() <= id {
            return;
        }

        // self.creatures[id].brain.show();
    }
} 

