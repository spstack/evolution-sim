/** ===============================================================================
 * File: creature.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: Includes all code that describes a single creature in the 2D sim
 * 
 * ===============================================================================*/

// Define external crates to use in this module
use std::fmt::Debug;
use rand::Rng;
use serde::{Deserialize, Serialize};

//===============================================================================
// CONSTANTS
//===============================================================================

// Creature default params
pub const DEFAULT_ENERGY_LEVEL : usize = 40;                // Starting amount of energy a creature is "born" with
pub const MAX_POSSIBLE_ENERGY : usize = 200;                // Max possible energy a creature can have
pub const MAX_POSSIBLE_AGE : usize = 200;                   // Max possible age a creature can be
pub const DEFAULT_REPRODUCE_AGE : usize = 22;               // Default age at which creature will reproduce
pub const DEFAULT_CREATURE_COLOR : [u8; 3] = [0, 75, 255];  // Default color each creature will be (blue)
pub const DEFAULT_ORIENTATION : CreatureOrientation = CreatureOrientation::Up; // Which way creature will face by default
pub const DEFAULT_MIN_REPRODUCE_ENERGY : usize = DEFAULT_ENERGY_LEVEL + 1;  // Minimum energy a creature should have to trigger a reproduce event.
pub const DEFAULT_REPRODUCE_ENERGY_COST : usize = DEFAULT_ENERGY_LEVEL;     // Default amount of energy it takes to reproduce
                                                                            // Intent is that this should be greater than the starting energy, so that creatures only reproduce when they find food
// Energy cost params
pub const DEFAULT_MOVE_ENERGY_COST : usize = 1;             // Default amount of energy it takes to move one space
pub const DEFAULT_ROTATE_ENERGY_COST : usize = 1;           // Default amount of energy it takes to rotate
pub const DEFAULT_KILL_ENERGY_COST : usize = 1;             // Default amount of energy it takes to perform a kill action

// Mist constants
pub const VISION_NEURON_INVALID_VAL : f32 = -1e6;           // Value that should be applied to a vision input neuron if there's nothing in view

const COLOR_MODE_VIOLENCE : bool = false;                   // Set this switch to `true` to make creature color depend on how violent the creature is rather than just inherited color with random mutation
const MIN_COLOR_DEVIATION : i8 = -100;                      // Minimum amount each color can change by when reproducing
const MAX_COLOR_DEVIATION : i8 = 100;                       // Maximum amount each color can change by when reproducing

const DEBUG_LEVEL : usize = 0;                              // Debug print level (higher number = more detail)


/// Defines the possible actions that a creature of any type can take
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CreatureActions {
    MoveForwards,
    MoveBackwards,
    MoveLeft,
    MoveRight,
    RotateCW,   // rotate clockwise
    RotateCCW,  // rotate counter-clockwise
    Stay,       // Do nothing
    Reproduce,
    Kill,
}
const ENABLED_CREATURE_ACTIONS : [CreatureActions; 9] = [Stay, MoveForwards, MoveBackwards, MoveLeft, MoveRight, RotateCCW, RotateCW, Reproduce, Kill];

/// Defines input neuron types to a creature. Each one of these has to directly translate into
/// a single neuron input in the "brain" of the creature. I.e. the number of entries here
/// defines how many input nodes are in the network.
#[derive(Copy, Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum CreatureInputs {
    Unused,             // Unused
    Age,                // Represents the age of the creature
    Energy,             // Current energy level
    VisionDistance,     // Distance (in spaces) of the object the creature can see
    VisionColorRed,     // Red component of the color of the object the creature can see [0, 255]
    VisionColorGreen,   // Green component of the color of the object the creature can see [0, 255]
    VisionColorBlue,    // Blue component of the color of the object the creature can see [0, 255]
    LastAction,         // The last action that the creature took
    Orientation,        // Which way th creature is facing
}
const ENABLED_CREATURE_INPUTS : [CreatureInputs; 8] = [Age, Energy, VisionDistance, VisionColorRed, VisionColorGreen, VisionColorBlue, Orientation, LastAction];

#[derive(Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct Position {
    pub x : usize, // x position of the creature
    pub y : usize, // y position of the creature
}

/// Possible orientations that creature can point
#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum CreatureOrientation {
    Up,
    Down,
    Left,
    Right,
}
pub const NUM_ORIENTATION_STATES : usize = 4;

// This represents the state of a creatures vision in one direction
#[derive(Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatureVisionState {
    pub obj_in_view : bool,     // specifies whether there is anything in view. If not, other values should be ignored
    pub dist : usize,           // distance to object (if obj_in_view)
    pub color : CreatureColor,  // color of object (if obj_in_view)
    pub space_type : SpaceStates, // State of the object
}

/// Represents the color of a creature
#[derive(Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreatureColor {
    pub red : u8,
    pub green : u8,
    pub blue : u8,
}
impl CreatureColor {
    /// Create a new creature color from input RGB vector
    pub fn new_from_vec(color_vec : [u8; 3]) -> CreatureColor {
        return CreatureColor {
            red : color_vec[0],
            green : color_vec[1],
            blue : color_vec[2],
        }
    }
    
    /// Return vector form of creature color
    pub fn get_as_vec(&self) -> [u8; 3] {
        return [self.red, self.green, self.blue];
    }
}

// Ensure we don't have to re-type the actions and inputs constantly
use CreatureActions::*;
use CreatureInputs::*;

use crate::{neural_net::NeuralNet, environment::SpaceStates};

#[derive(Serialize, Deserialize, Clone)]
pub struct CreatureParams {
    pub reproduce_energy_cost : usize,
    pub move_energy_cost : usize,
    pub rotate_energy_cost : usize,
    pub kill_energy_cost : usize,
    pub starting_energy : usize,
}
impl CreatureParams {
    pub fn new() -> CreatureParams {
        return CreatureParams {
            reproduce_energy_cost : DEFAULT_REPRODUCE_ENERGY_COST,
            move_energy_cost : DEFAULT_MOVE_ENERGY_COST,
            rotate_energy_cost : DEFAULT_ROTATE_ENERGY_COST,
            kill_energy_cost : DEFAULT_KILL_ENERGY_COST,
            starting_energy : DEFAULT_ENERGY_LEVEL,
        }
    }
}

/// Version 1 of a simple creature. It contains all of the state information about a creature
/// The environment that creates the creature should call ``
#[derive(Serialize, Deserialize, Clone)]
pub struct Creature {
    /// Input parameters to the creature
    pub params : CreatureParams,

    /// Creatures brain represented as a neural network
    pub brain : Brain,

    /// ID number of this creature
    pub id : usize,

    /// Indicates whether the creature is alive
    pub is_alive : bool,
    killed : bool,

    /// Current position in 2d coordinates x, y
    pub position : Position,

    /// Current orientation that creature is pointing
    pub orientation : CreatureOrientation,

    /// Current energy level
    pub energy : usize,

    /// Vision state of the creature (abstract info about what it can "see" in front of it)
    pub vision_state : CreatureVisionState,

    /// Current age of the creature in time-steps 
    pub age : usize,

    /// Color of the creature
    pub color : CreatureColor,

    /// Age at which creature will reproduce
    pub reproduction_age : usize,

    /// Last action that the creature took
    pub last_action : CreatureActions,

    /// Mapping from input neuron number (idx into this vec) to type.
    input_neuron_types : Vec<CreatureInputs>,

    /// Mapping from output neuron number (idx into this vec) to action type.
    output_neuron_types : Vec<CreatureActions>,
}

impl Creature {

    // ============= CONSTRUCTORS ================

    /// Constructor returns creature instance w/ default values
    pub fn new(id : usize, inparams : &CreatureParams) -> Creature {
        let input_neuron_types = ENABLED_CREATURE_INPUTS.to_vec();
        let output_neuron_types = ENABLED_CREATURE_ACTIONS.to_vec();

        let temp_creature = Creature {
            params : inparams.clone(),
            brain : Brain::new(&input_neuron_types,&output_neuron_types),
            id : id,
            is_alive : true,
            killed : false,
            position : Position {x : 0, y : 0},
            orientation : DEFAULT_ORIENTATION,
            energy : DEFAULT_ENERGY_LEVEL,
            vision_state : CreatureVisionState {obj_in_view : false, dist : 0, color : CreatureColor::new_from_vec([0,0,0]), space_type : SpaceStates::BlankSpace},
            age : 0,
            last_action : CreatureActions::Stay,
            color : CreatureColor::new_from_vec(DEFAULT_CREATURE_COLOR),
            reproduction_age : DEFAULT_REPRODUCE_AGE,
            input_neuron_types : input_neuron_types,
            output_neuron_types : output_neuron_types,
        };

        return temp_creature;
    }

    /// Constructor to create a new creature from a provided parent (genes copied with optional mutations)
    pub fn new_offspring(id : usize, parent : &Creature, mutation_prob : f32) -> Creature {

        let mut temp_creature = Creature {
            params : parent.params.clone(),
            brain : Brain::new_copy(&parent.brain, mutation_prob), // this creatures a copy of the brain while randomly mutating each weight/bias
            id : id,
            is_alive : true,
            killed : false,
            position : Position {x : parent.position.x, y : parent.position.y},
            orientation : parent.orientation,
            energy : parent.params.starting_energy,
            vision_state : CreatureVisionState {obj_in_view : false, dist : 0, color : CreatureColor::new_from_vec([0,0,0]), space_type : SpaceStates::BlankSpace},
            age : 0,
            last_action : CreatureActions::Stay,
            color : parent.color.clone(),
            reproduction_age : DEFAULT_REPRODUCE_AGE,
            input_neuron_types : parent.input_neuron_types.clone(),
            output_neuron_types : parent.output_neuron_types.clone(),
        };

        if COLOR_MODE_VIOLENCE {
            // Since this creature hasn't killed yet, change it's color a bit to indicate it's more docile (until proven otherwise)
            temp_creature.unset_killer();
        } else {
            // randomly apply a color change with probability `mutation_prob`
            temp_creature.apply_random_color_mutation(mutation_prob);
        }

        return temp_creature;
    }


    /// Constructor to create a new creature from a JSON string
    #[allow(dead_code)]
    pub fn new_from_json(id : usize, json_in : &str) -> serde_json::Result<Creature> {
        // This is pretty neat we can basically just tell serde_json to copy all
        // fields into a given structure.
        let mut temp_creature : Creature = serde_json::from_str(json_in)?;

        // Set the ID to the new value
        temp_creature.id = id;

        return Ok(temp_creature);
    }

    /// Apply a random mutation to color of the creature (used when reproducing)
    /// The mutation prob defines how likely it is that each of the red/greed/blue components
    /// are mutated at all. The amount of mutation is bounded by `[MIN|MAX]_COLOR_DEVIATION` params
    pub fn apply_random_color_mutation(&mut self, mutation_prob : f32) {
        let mut rng = rand::thread_rng();

        // Randomly mutate each color by a random amount
        if rng.gen::<f32>() <= mutation_prob {
            let deviation_val = rng.gen_range(MIN_COLOR_DEVIATION..=MAX_COLOR_DEVIATION);
            if deviation_val < 0 {
                self.color.red = self.color.red.saturating_sub(-deviation_val as u8);
            } else {
                self.color.red = self.color.red.saturating_add(deviation_val as u8);
            }
        }
        if rng.gen::<f32>() <= mutation_prob {
            let deviation_val = rng.gen_range(MIN_COLOR_DEVIATION..=MAX_COLOR_DEVIATION);
            if deviation_val < 0 {
                self.color.green = self.color.green.saturating_sub(-deviation_val as u8);
            } else {
                self.color.green = self.color.green.saturating_add(deviation_val as u8);
            }
        }
        if rng.gen::<f32>() <= mutation_prob {
            let deviation_val = rng.gen_range(MIN_COLOR_DEVIATION..=MAX_COLOR_DEVIATION);
            if deviation_val < 0 {
                self.color.blue = self.color.blue.saturating_sub(-deviation_val as u8);
            } else {
                self.color.blue = self.color.blue.saturating_add(deviation_val as u8);
            }
        }

        // Scale the colors so that they at least reach a minimum brightness
        // This protects against the creatures becoming invisible (all black)
        let color_sum = self.color.red as usize + self.color.blue as usize + self.color.green as usize;
        if color_sum < 255 {
            let brightness_diff = (255 - color_sum) / 3;
            self.color.red += brightness_diff as u8;
            self.color.green += brightness_diff as u8;
            self.color.blue += brightness_diff as u8;
        }
    }

    // ============= ENVIRONMENT INTERFACE FUNCTIONS ================


    /// Set position in the board
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.position.x = x;
        self.position.y = y;
    }

    /// Set the creatures orientation
    pub fn set_orientation(&mut self, orientation : CreatureOrientation) {
        self.orientation = orientation;
    }

    /// Eat a piece of food that gives it the specified amount of energy
    pub fn eat_food(&mut self, food_energy : usize) {
        if self.energy + food_energy > MAX_POSSIBLE_ENERGY {
            self.energy = MAX_POSSIBLE_ENERGY;
        } else {
            self.energy += food_energy;
        }
    }

    /// Set the vision state of the creature based on surroundings
    pub fn set_vision(&mut self, vision : CreatureVisionState) {
        self.vision_state = vision;
    }

    /// Kill this creature (another creature has hunted it)
    pub fn kill(&mut self) {
        self.energy = 0;
        self.is_alive = false;
        self.killed = true;
    }

    /// Mark this creature as a killer (carnivore). This changes it's color a bit to indicate to others that it's dangerous
    pub fn set_killer(&mut self) {
        if COLOR_MODE_VIOLENCE {
            self.color.red = self.color.red.saturating_add(10);
            self.color.blue = self.color.blue.saturating_sub(10);
        }
    }

    pub fn unset_killer(&mut self) {
        if COLOR_MODE_VIOLENCE {
            self.color.red = self.color.red.saturating_sub(10);
            self.color.blue = self.color.blue.saturating_add(10);
        }
    }

    /// Returns true if the creature is dead and false if it is alive
    pub fn is_dead(&self) -> bool {
        if self.energy > 0  && self.age < MAX_POSSIBLE_AGE {
            return false
        } else {
            return true;
        }
    }

    // returns whether this creature has been killed
    pub fn was_killed(&self) -> bool {
        return self.killed;
    }


    /// Sense surroundings by populating the input neurons to reflect current state
    /// This should be called before `perform_next_action`
    pub fn sense_surroundings(&mut self) {
        // Get vision state node values first
        let vis_dist : f32;
        let vis_red : f32;
        let vis_green : f32;
        let vis_blue : f32;

        if self.vision_state.obj_in_view {
            vis_dist = self.vision_state.dist as f32;
            vis_red = self.vision_state.color.red as f32;
            vis_green = self.vision_state.color.green as f32;
            vis_blue = self.vision_state.color.blue as f32;
        } else {
            // Set vision neurons to something way out there to try to communicate long distance
            vis_dist = VISION_NEURON_INVALID_VAL;
            vis_red = VISION_NEURON_INVALID_VAL;
            vis_green = VISION_NEURON_INVALID_VAL;
            vis_blue = VISION_NEURON_INVALID_VAL;
        }


        // Loop through each input neuron and set it depending on the type
        for (input_neuron_idx, neuron_type) in self.input_neuron_types.iter().enumerate() {
            match neuron_type {
                Age => self.brain.set_input(input_neuron_idx, self.age as f32),
                Energy => self.brain.set_input(input_neuron_idx, self.energy as f32),
                VisionDistance => self.brain.set_input(input_neuron_idx, vis_dist),
                VisionColorRed => self.brain.set_input(input_neuron_idx, vis_red),
                VisionColorGreen => self.brain.set_input(input_neuron_idx, vis_green),
                VisionColorBlue => self.brain.set_input(input_neuron_idx, vis_blue),
                LastAction => self.brain.set_input(input_neuron_idx,  self.action_to_f32(self.last_action)),
                Orientation => self.brain.set_input(input_neuron_idx, self.orientation_to_f32(self.orientation)),
                _ => {
                    if DEBUG_LEVEL > 0 {
                        println!("Warning: unused/unpopulated input neuron {:?} at idx {}", neuron_type, input_neuron_idx);
                    }
                }
            }
        } 
    }

    /// Function to tranlate an action into a float that is consumed by the neural net
    fn action_to_f32(&self, action : CreatureActions) -> f32 {

        // These are a  little random, but I guess trying to make similar actions have similar values. Not sure that's the
        // right way to go about it tho
        return match action {
            CreatureActions::Stay => 0.0,
            CreatureActions::MoveForwards => 2.0,
            CreatureActions::MoveBackwards => 3.0,
            CreatureActions::MoveLeft => 4.0,
            CreatureActions::MoveRight => 5.0,
            CreatureActions::RotateCCW => 10.0,
            CreatureActions::RotateCW => 11.0,
            CreatureActions::Reproduce => 15.0,
            CreatureActions::Kill => 20.0,
        }
    }

    /// Function to tranlate the orientation into a float that is consumed by the neural net
    fn orientation_to_f32(&self, orientation : CreatureOrientation) -> f32 {
        return match orientation {
            CreatureOrientation::Up => 0.0,
            CreatureOrientation::Left => 1.0,
            CreatureOrientation::Down => 2.0,
            CreatureOrientation::Right => 3.0,
        }
    }

    // Perform next action (evaluate neural network and decide on next action based on output)
    // 
    pub fn perform_next_action(&mut self) -> CreatureActions {

        if self.is_dead() {
            // Creature is dead, just return stay action
            self.last_action = CreatureActions::Stay;
            return CreatureActions::Stay;
        }

        // Increase age until a max. If we've hit max, then die of old age
        if self.age < MAX_POSSIBLE_AGE {
            self.age += 1;
        } else {
            self.is_alive = false;
            return Stay;
        }

        // Before we even do any action eval, check to see whether creature should reproduce
        if self.energy > DEFAULT_MIN_REPRODUCE_ENERGY {
            self.energy -= self.params.reproduce_energy_cost;
            self.last_action = Reproduce;
            return Reproduce;
        }

        // Otherwise, evaluate the brain network based on the current state of the input neurons
        // To check what our next action will be
        let mut action = self.brain.get_next_action();

        // Get the value of the action to be taken
        // let action = self.brain.get_current_action();
        self.last_action = action;

        // Show the state of the brain if debug level high enough
        if DEBUG_LEVEL > 1 {
            print!("{}", self.to_json());
        }

        // NOTE: the calling environment will have to handle the action depending on the 
        // state of the environment. The only thing that does not depend on the env
        // is rotation which we can simply handle here. Nothing will happen if the action
        // is not a rotation
        self.apply_rotation(action);


        // Calculate new energy based on which action we decide to take. Different actions cost differing amounts
        self.energy = match action {
            Reproduce => self.energy.saturating_sub(self.params.reproduce_energy_cost),
            MoveBackwards | MoveForwards | MoveLeft | MoveRight => self.energy.saturating_sub(self.params.move_energy_cost),
            RotateCCW | RotateCW => self.energy.saturating_sub(self.params.rotate_energy_cost),
            Kill => self.energy.saturating_sub(self.params.kill_energy_cost),
            _ => self.energy,
        };

        // If we're out of energy, mark this creature dead and return Stay action
        if self.energy == 0 {
            self.is_alive = false;
            action = Stay;
        }

        return action; 
    }

    /// Return a JSON string that represents this creature. Allows saving state to
    /// a file for use later
    pub fn to_json(&self) -> String {
        let json_string = serde_json::to_string(&self);
        return json_string.expect("Error converting creature to JSON");
    }

    // ============= INTERNAL FUNCTIONS ================

    /// Apply rotation to creature - there is definitely a better way to do this...
    /// If the action does not specify a rotation, just do nothing
    fn apply_rotation(&mut self, action : CreatureActions) {
        match self.orientation {
            CreatureOrientation::Up => {
                self.orientation = match action {
                    RotateCCW => CreatureOrientation::Left,
                    RotateCW => CreatureOrientation::Right,
                    _=> self.orientation, // do nothing
                }
            }
            CreatureOrientation::Left => {
                self.orientation = match action {
                    RotateCCW => CreatureOrientation::Down,
                    RotateCW => CreatureOrientation::Up,
                    _=> self.orientation, // do nothing
                }
            }
            CreatureOrientation::Down => {
                self.orientation = match action {
                    RotateCCW => CreatureOrientation::Right,
                    RotateCW => CreatureOrientation::Left,
                    _=> self.orientation, // do nothing
                }
            }
            CreatureOrientation::Right => {
                self.orientation = match action {
                    RotateCCW => CreatureOrientation::Up,
                    RotateCW => CreatureOrientation::Down,
                    _=> self.orientation, // do nothing
                }
            }
        } 
    }

}


/// Second version of a creature brain that uses more generic neural network
#[derive(Deserialize, Serialize, Clone)]
pub struct Brain {
    // Underlying neural network
    net : NeuralNet<f32>,

    // Input types (maps input neuron type to index)
    pub input_node_types : Vec<CreatureInputs>,

    // Input types (maps output neuron index to type)
    pub output_node_types : Vec<CreatureActions>,
}

// Define the number and size of internal layers in the brain neural net
// (input/output will be overwritten by the constructor)
const PLACEHOLDER_NUM_NODES : usize = 0;
const BRAIN_LAYER_SIZES : [usize; 4] = [PLACEHOLDER_NUM_NODES, 10, 10, PLACEHOLDER_NUM_NODES]; // 2 internal layers with 6 neurons each. First/Last layer sizes will be specified by constructor
const BRAIN_MIN_INIT_NODE_VAL : f32 = -50.0;  // Min initial value that a node will take
const BRAIN_MAX_INIT_NODE_VAL : f32 = 50.0;   // Max initial value that a node will take 

impl Brain {

    /// Create a new instance of the Brain with random values for all weights/biases
    /// and explicit mapping of input/output neuron types
    /// * `input_node_types` is a vector mapping the input neuron idx to which type of input it is. It
    /// also determines the number of neurons in the input layer by it's size
    /// * `output_node_types` is a vector mapping the output neuron idx to which type of action it is. It
    /// also determines the number of neurons in the output layer by it's size
    pub fn new(input_node_types : &Vec<CreatureInputs>, output_node_types : &Vec<CreatureActions>) -> Brain {
        let mut layer_sizes  = BRAIN_LAYER_SIZES.to_vec();
        let output_layer_idx = layer_sizes.len() - 1;
        layer_sizes[0] = input_node_types.len();
        layer_sizes[output_layer_idx] = output_node_types.len();
        return Brain {
            net : NeuralNet::new(&layer_sizes, BRAIN_MIN_INIT_NODE_VAL, BRAIN_MAX_INIT_NODE_VAL),
            input_node_types : input_node_types.clone(), 
            output_node_types : output_node_types.clone(),
        };
    }

    /// Create a new brain copy, but randomly mutate some of the weights/biases
    /// with chance of mutation for each of mutation_prob 
    pub fn new_copy(other_brain : &Brain, mutation_prob : f32) -> Brain {

        // Copy the neural net and apply random mutations to it
        let mut nn = other_brain.net.clone();
        nn.apply_rand_mutations(mutation_prob, BRAIN_MIN_INIT_NODE_VAL, BRAIN_MAX_INIT_NODE_VAL);

        return Brain {
            net : nn,
            input_node_types : other_brain.input_node_types.clone(), 
            output_node_types : other_brain.output_node_types.clone(),
        };
    }

    /// Set the value of the input neuron at specified index
    pub fn set_input(&mut self, neuron_idx : usize, value : f32) {
        self.net.set_input_node(neuron_idx, value);
    }

    /// Evaluate the neural network and output the next action the creature will take
    pub fn get_next_action(&mut self) -> CreatureActions {
        let output_idx = self.net.evaluate_network().unwrap();
        return self.output_node_types[output_idx];
    }


}


