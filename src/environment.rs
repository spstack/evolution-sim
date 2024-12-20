/** ===============================================================================
 * File: environment.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements environment features that the creature inhabits
 * ===============================================================================*/
use crate::creature::*;
use serde::{Deserialize, Serialize};
use rand::Rng;

//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEBUG_LEVEL : usize = 0;

// Env parameters
pub const DEFAULT_ENERGY_PER_FOOD_PIECE : usize = 20;   // How much energy each piece of food will give a creature
pub const DEFAULT_MUTATION_PROB : f32 = 0.02;           // Default probability that each weight/bias in a creature's DNA will mutate upon reproduction
pub const NEW_FOOD_PIECES_PER_STEP : f32 = 0.8;         // Average number of new food pieces that should appear in the environment per step (can be less than 1)

// Reproduction params
pub const DEFAULT_OFFSPRING_PER_REPRODUCE : usize = 4; // Number of offspring that each creature will have upon each reproduction event
pub const REPRODUCTION_AGE : usize = 21;                // Default age at which a creature will reproduce
pub const MAX_OFFSPRING_SPAWN_DIST : isize = 4;         // Max distance (in spaces) that a creatures offspring will spawn from the parent

// Vision params
pub const MAX_CREATURE_VIEW_DISTANCE : isize = 5;       // Defines max number of spaces a creature can "see"
pub const FOOD_SPACE_COLOR : [u8; 3] = [40, 255, 40];     // color of food space (orange-ish)
pub const WALL_SPACE_COLOR : [u8; 3] = [0, 0, 0];       // color of wall space (black)


//===============================================================================
// Environment V1 Declarations
//===============================================================================

/// Defines all possible error codes for the environment
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum EnvErrors {
    EarlyExitErr,     // Simulation could not run all steps requested because all creatures died
}

/// Enumeration that defines the possible states 
#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpaceStates {
    BlankSpace,                 // Space is blank
    CreatureSpace(usize),       // Space has a creature in it. The single argument represents the ID of the creature
    FoodSpace,                  // Space has a food in it
    WallSpace,                  // Space that contains a wall
}


/// Structure that defines all input parameters to a new environment
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EnvironmentParams {
    pub env_x_size : usize,                 // X size of the sim in "spaces"
    pub env_y_size : usize,                 // Y size of the sim in "spaces"
    pub num_start_creatures : usize,        // Number of creatures to start the sim with
    pub num_start_food : usize,             // Number of starting food spaces
    pub num_start_walls : usize,            // Number of wall spaces on the board
    pub energy_per_food_piece : usize,      // Number of energy units that will be given per food consumed 
    pub max_offspring_per_reproduce : usize,// Maximum number of offspring that will be produced by one reproduction event
    pub mutation_prob : f32,                // Probability that a single value in the creatures DNA will randomly mutate upon reproduction
    pub avg_new_food_per_day : f32,         // Average number of new food pieces added to the environment per day
}
impl EnvironmentParams {
    /// Return a default version of the parameters
    pub fn new() -> EnvironmentParams {
        EnvironmentParams {
            env_x_size : 50,
            env_y_size : 50,
            num_start_creatures : 100,  
            num_start_food : 150,
            num_start_walls : 200,
            energy_per_food_piece : DEFAULT_ENERGY_PER_FOOD_PIECE,
            max_offspring_per_reproduce : DEFAULT_OFFSPRING_PER_REPRODUCE,
            mutation_prob : DEFAULT_MUTATION_PROB,
            avg_new_food_per_day : NEW_FOOD_PIECES_PER_STEP, 
        }
    }
}

/// Structure representing a very simple 2-D environment
#[derive(Serialize, Deserialize, Clone)]
pub struct EnvironmentV1 {
    // Parameters
    pub params : EnvironmentParams,     // All parameters that can be specified when creating a new env 

    // Current state
    pub creatures : Vec<CreatureV1>,    // Vector containing all creature instances
    pub positions : Vec<Vec<SpaceStates>>, // Contains the states of each space.
    pub time_step : usize,              // Represents the current time step in the sim
    pub num_food : usize,               // Number of current food pieces on the board
    pub num_creatures : usize,          // Number of living creatures on the board
    pub num_blank : usize,              // Number of blank spaces on the board
    pub num_walls : usize,              // Number of wall spaces on the board (should be the same as the start parameter, but used for sanity check)
    pub num_total_creatures : usize,    // Number of total creatures created throughout sim
}


/// Implementation of EnvironmentV1
impl EnvironmentV1 {

    /// Constructor for new environment instance that's randomly populated
    pub fn new_rand(in_params : &EnvironmentParams) -> EnvironmentV1 {
        let mut rng = rand::thread_rng();

        // Initialize all positions to be blank at first
        let temp_positions = vec![vec![SpaceStates::BlankSpace; in_params.env_y_size]; in_params.env_x_size];

        // Initialize creature vector
        let temp_creature_vec = Vec::<CreatureV1>::with_capacity(in_params.num_start_creatures);
        let num_spaces = in_params.env_x_size * in_params.env_y_size;

        // Create temporary environment, transferring ownership of vectors
        let mut temp_env = EnvironmentV1 {
            params: EnvironmentParams {
                env_x_size : in_params.env_x_size,
                env_y_size : in_params.env_y_size,
                num_start_creatures : in_params.num_start_creatures,  
                num_start_food : in_params.num_start_food,
                num_start_walls : in_params.num_start_walls,
                energy_per_food_piece : in_params.energy_per_food_piece,
                max_offspring_per_reproduce : in_params.max_offspring_per_reproduce,
                mutation_prob : in_params.mutation_prob,
                avg_new_food_per_day : in_params.avg_new_food_per_day,
            },
            creatures : temp_creature_vec,
            positions : temp_positions,
            time_step : 0,
            num_food : 0, // set to zero as these will be added later
            num_creatures : 0,
            num_walls : 0,
            num_blank : num_spaces,
            num_total_creatures : in_params.num_start_creatures,
        };

        // Fill in random spaces with food
        for _food_num in 0..in_params.num_start_food {
            let pos = temp_env.get_rand_blank_space();
            temp_env.add_food_space(pos);
        }

        // Fill in random spaces with creatures
        for creature_num in 0..in_params.num_start_creatures {
            // Create creature
            let mut creature = CreatureV1::new(creature_num);

            // Set few parameters of the new creature
            let pos = temp_env.get_rand_blank_space();
            creature.set_position(pos.x, pos.y);
            creature.set_reproduction_age(REPRODUCTION_AGE);

            // Set random initial orientation
            let orient = rng.gen_range(0..NUM_ORIENTATION_STATES);
            let orientation = match orient {
                0 => CreatureOrientation::Up,
                1 => CreatureOrientation::Right,
                2 => CreatureOrientation::Down,
                3 => CreatureOrientation::Left,
                _ => panic!("Invalid initial random orientation! Update the number of states"),
            };
            creature.set_orientation(orientation);

            // Add it to the board
            temp_env.add_creature(creature);
        }

        // Fill random wall spaces
        for _wall_num in 0..in_params.num_start_walls {
            let pos = temp_env.get_rand_blank_space();
            temp_env.add_wall_space(pos);
        }

        return temp_env;

    }

    /// Convert this environment to JSON representation for saving/loading
    pub fn to_json(&self) -> String {
        let json_string = serde_json::to_string_pretty(&self).unwrap();
        return json_string;
    }

    /// Main interface to run a certain number of simulation steps
    pub fn run_n_steps(&mut self, num_steps : usize) -> Result<(), EnvErrors> {
        for n in 0..num_steps {

            // Run the step
            self.advance_step();

            // Check whether there's any creatures left
            if self.num_creatures == 0 {
                if DEBUG_LEVEL > 0 {
                    println!("Stopping simulation after {} steps because there are no creatures left", n);
                }
                return Err(EnvErrors::EarlyExitErr);
            }
        }
        return Ok(());
    }


    /// Print the current state of the environment board
    pub fn show(&self) {
        println!();
        let num_dashes = self.params.env_x_size * 3 + 1;
        println!("{:-<width$}", " ", width = num_dashes); // print horizontal dashes
        for y in 0..self.params.env_y_size {
            print!("|");
            for x in 0..self.params.env_x_size {
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


    /// Audit the counters of each space type vs. the position matrix to make sure everything is in sync
    fn update_space_counters(&mut self) {
        let mut temp_food : usize = 0; 
        let mut temp_walls : usize = 0; 
        let mut temp_creatures : usize = 0; 
        let mut temp_blank : usize = 0; 
        for x in 0..self.params.env_x_size {
            for y in 0..self.params.env_y_size {
                match self.positions[x][y] {
                    SpaceStates::BlankSpace => temp_blank += 1,
                    SpaceStates::FoodSpace => temp_food += 1,
                    SpaceStates::CreatureSpace(_id) => temp_creatures += 1,
                    SpaceStates::WallSpace => temp_walls += 1,
                }
            }
        }

        self.num_blank = temp_blank;
        self.num_creatures = temp_creatures;
        self.num_walls = temp_walls;
        self.num_food = temp_food;
    }



    /// Advance one "day"!
    pub fn advance_step(&mut self) {

        // Print some info about the env
        if DEBUG_LEVEL > 0 {
            println!("===================== STEP {} ===============", self.time_step);
            println!("Creatures: {}", self.creatures.len());
            println!("");
        }

        // Audit the board on every step
        self.update_space_counters();

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
                    } else {
                        next_position.y = self.params.env_y_size - 1;
                    }
                },

                CreatureActions::MoveDown => {
                    // Check if move would go beyond the bounds of this board
                    if next_position.y < self.positions.len() - 1 {
                        next_position.y += 1;
                    } else {
                        next_position.y = 0;
                    }
                },

                CreatureActions::MoveLeft => {
                    if next_position.x > 0 {
                        next_position.x -= 1;
                    } else {
                        next_position.x = self.params.env_x_size - 1;
                    }
                },

                CreatureActions::MoveRight => {
                    // Check if move would go beyond the bounds of this board
                    if next_position.x < self.positions[0].len() - 1 {
                        next_position.x += 1;
                    } else {
                        next_position.x = 0;
                    }
                },

                CreatureActions::Reproduce => {
                    // Randomly determine how many offspring this creature will have
                    let num_offspring = rng.gen_range(1..=self.params.max_offspring_per_reproduce);
                    if DEBUG_LEVEL > 1 {
                        println!("Creature {} is reproducing with {} offspring!", creature.id, num_offspring);
                    }
                    for _offspring_num in 0..num_offspring {
                        let new_offspring = CreatureV1::new_offspring(self.num_total_creatures, &creature, self.params.mutation_prob);
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
                        creature.eat_food(self.params.energy_per_food_piece);
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

        // Add new spawned creatures in random locations around their parents
        for mut new_creature in temp_new_creatures {
            let pos = self.get_blank_space_at_point(new_creature.position);
            match pos {
                Some(new_pos) => {
                    new_creature.set_position(new_pos.x, new_pos.y);
                    self.positions[new_creature.position.x][new_creature.position.y] = SpaceStates::CreatureSpace(new_creature.id);
                    self.creatures.push(new_creature);
                },
                None => {
                    // Just don't spawn the creature cause there's no space
                    continue;
                }
            }
        }

        // Evaluate the vision of each of the creatures now that everything is updated
        self.update_creature_vision();

        // Add food pieces according to settings
        self.add_new_food_pieces();

        // If proper debug level show the env after each step
        if DEBUG_LEVEL > 0 {
            self.show();
        }

        // Increment the time step counter
        self.time_step += 1;

    }

    /// Add random number of new food pieces to the board in random locations according to 
    /// `avg_new_food_per_day` value.
    fn add_new_food_pieces(&mut self) {
        let mut rng = rand::thread_rng();

        if self.params.avg_new_food_per_day < 1.0 {
            // If the number of new food is less than 1, then decide whether to add
            // a single food piece treating `avg_new_food_per_day` as a probability
            if rng.gen::<f32>() < self.params.avg_new_food_per_day {
                self.add_food_space(self.get_rand_blank_space());
            }
        } else {
            // If avg the number of food pieces is greater than 1, then randomly sample from
            // range where `avg_new_food_per_day` is the center of the distribution. Result
            // will represent how many food pieces to add
            let max_food = self.params.avg_new_food_per_day * 2.0;
            let num_food = rng.gen_range(0.0..max_food).round() as usize;
            for _ in 0..num_food {
                self.add_food_space(self.get_rand_blank_space());
            }
        }
    }

    /// Add a single food space to the specified location
    pub fn add_food_space(&mut self, position : CreaturePosition) {
        if self.positions[position.x][position.y] != SpaceStates::BlankSpace {
            panic!("Tried to add a food space to a non-blank space!");
        }
        self.positions[position.x][position.y] = SpaceStates::FoodSpace;
    }

    /// Add single creature to the environment at position specified by creature itself
    pub fn add_creature(&mut self, new_creature : CreatureV1) {
        if self.positions[new_creature.position.x][new_creature.position.y] != SpaceStates::BlankSpace {
            panic!("Tried to add a creature to a non-blank space!");
        }
        self.positions[new_creature.position.x][new_creature.position.y] = SpaceStates::CreatureSpace(new_creature.id);
        self.creatures.push(new_creature);
        self.num_total_creatures += 1;
    }

    /// Add a wall space to the specified location
    pub fn add_wall_space(&mut self, position : CreaturePosition) {
        if self.positions[position.x][position.y] != SpaceStates::BlankSpace {
            panic!("Tried to add a wall space to a non-blank space!");
        }
        self.positions[position.x][position.y] = SpaceStates::WallSpace;
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
                    self.num_creatures -= 1;
                    break;
                }
            }
        }
    }

    /// Update what each of the creatures is currently "seeing"
    fn update_creature_vision(&mut self) {

        // Check vision of all creatures
        for c_idx in 0..self.creatures.len() {

            // Define variables for position we will be looking in
            let mut xpos = self.creatures[c_idx].position.x;
            let mut ypos = self.creatures[c_idx].position.y;

            for _step in 0..MAX_CREATURE_VIEW_DISTANCE {
                // Update the position we're currently looking in by checking the direction creature is facing
                match self.creatures[c_idx].orientation {
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
                if (xpos >= self.params.env_x_size) || (ypos >= self.params.env_y_size) {
                    break;
                }

                // This is super ugly, but just takes the x and y distances and adds them
                let creature_x = self.creatures[c_idx].position.x;
                let creature_y = self.creatures[c_idx].position.y;
                let distance : usize = ((xpos as i32 - creature_x as i32).abs() + (ypos as i32 - creature_y as i32).abs()) as usize;

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
                        self.creatures[c_idx].set_vision(vis);
                    },

                    // Wall space in view
                    SpaceStates::WallSpace => {
                        let vis : CreatureVisionState = CreatureVisionState {
                            obj_in_view : true,
                            dist : distance,
                            color : CreatureColor::new_from_vec(WALL_SPACE_COLOR)
                        };
                        self.creatures[c_idx].set_vision(vis);
                    },

                    // Another creature is in view, update the color with the creature's color
                    SpaceStates::CreatureSpace(c_id) => {
                        let target_cidx = self.get_creature_idx_from_id(c_id).unwrap();
                        let tgt_creature_color = self.creatures[target_cidx].color.get_as_vec();
                        let vis : CreatureVisionState = CreatureVisionState {
                            obj_in_view : true,
                            dist : distance,
                            color : CreatureColor::new_from_vec(tgt_creature_color)
                        };
                        self.creatures[target_cidx].set_vision(vis);
                    }
                }
            }
        }
    }


    /// Get a random blank spot on the board
    fn get_rand_blank_space(&self) -> CreaturePosition {
        let mut rng = rand::thread_rng();
        let mut done : bool = false;
        let mut found_x: usize = 0;
        let mut found_y: usize = 0;
        let mut attempts : usize = 0;

        // Loop until we find a blank space
        while !done {
            let x = rng.gen_range(0..self.params.env_x_size);
            let y = rng.gen_range(0..self.params.env_y_size);

            // Only allow overwriting of blank spaces
            match self.positions[x][y] {
                SpaceStates::BlankSpace => {
                    found_x = x;
                    found_y = y;
                    done = true;
                },
                _ => {
                    attempts += 1;
                },
            }

            // Check loop watchdog
            if attempts > 10_000 {
                panic!("Error! No blank spaces left in the simulation!");
            }
        }

        return CreaturePosition {
            x : found_x,
            y: found_y,
        }
    }

    /// Get a random blank spot centered at the specified position. This is used during creature reproduction
    /// to determine where offspring should be placed
    fn get_blank_space_at_point(&self, target_pos : CreaturePosition) -> Option<CreaturePosition> {
        let mut rng = rand::thread_rng();
        let mut done : bool = false;
        let mut found_x: usize = 0;
        let mut found_y: usize = 0;
        let mut attempts : usize = 0;

        // Loop until we find a blank space
        while !done {
            let x_diff : isize = rng.gen_range(-MAX_OFFSPRING_SPAWN_DIST..MAX_OFFSPRING_SPAWN_DIST);
            let y_diff : isize = rng.gen_range(-MAX_OFFSPRING_SPAWN_DIST..MAX_OFFSPRING_SPAWN_DIST);

            let mut x_isize = (target_pos.x as isize + x_diff);
            let mut y_isize = (target_pos.y as isize + y_diff);

            if x_isize < 0 {
                x_isize = 0;
            }
            if y_isize < 0 {
                y_isize = 0;
            }
            if x_isize >= self.params.env_x_size as isize {
                x_isize = self.params.env_x_size as isize - 1;
            }
            if y_isize >= self.params.env_y_size as isize {
                y_isize = self.params.env_y_size as isize - 1;
            }

            let x: usize = x_isize as usize;
            let y: usize = y_isize as usize;

            // Only allow overwriting of blank spaces
            match self.positions[x][y] {
                SpaceStates::BlankSpace => {
                    found_x = x;
                    found_y = y;
                    done = true;
                },
                _ => {
                    attempts += 1;
                },
            }

            // If we make a number of attempts equal to number of possible spaces, then just
            // give up early and say there are no spaces
            if attempts > (MAX_OFFSPRING_SPAWN_DIST * MAX_OFFSPRING_SPAWN_DIST) as usize {
                return None;
            }
        }

        return Some(CreaturePosition {
            x : found_x,
            y : found_y,
        });
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

