/** ===============================================================================
 * File: creature.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: Includes all code that describes a single creature in the 2D sim
 * 
 * Notes:
 * Add orientation to creature state
 * Add color to creature statistics (r,g,b) value
 * Add vision that only looks at current orientation
 * ===============================================================================*/

// Define external crates to use in this module
use std::fmt::Debug;
use serde::{Deserialize, Serialize};

//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEFAULT_ENERGY_LEVEL : usize = 20;
pub const MAX_POSSIBLE_ENERGY : usize = 200;
pub const MAX_POSSIBLE_AGE : usize = 200;

pub const DEFAULT_REPRODUCE_AGE : usize = 21;               // Default age at which creature will reproduce
pub const DEFAULT_CREATURE_COLOR : [u8; 3] = [0, 0, 255];   // Default color each creature will be (blue)
pub const DEFAULT_ORIENTATION : CreatureOrientation = CreatureOrientation::Up; // Which way creature will face by default


pub const VISION_NEURON_INVALID_VAL : f32 = -1e6;           // Value that should be applied to a vision input neuron if there's nothing in view

const DEBUG_LEVEL : usize = 0;  // Debug print level (higher number = more detail)


/// Defines the possible actions that a creature of any type can take
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CreatureActions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateCW,   // rotate clockwise
    RotateCCW,  // rotate counter-clockwise
    Stay,       // Do nothing
    Reproduce,
    // Kill, //....soon
}

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
}

#[derive(Copy, Clone, PartialEq, Deserialize, Serialize)]
pub struct CreaturePosition {
    pub x : usize, // x position of the creature
    pub y : usize, // y position of the creature
}

/// Possible orientations that creature can point
#[derive(Copy, Clone, PartialEq, Deserialize, Serialize)]
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

use crate::neural_net::NeuralNet;

/// Version 1 of a simple creature. It contains all of the state information about a creature
/// The environment that creates the creature should call ``
#[derive(Serialize, Deserialize, Clone)]
pub struct CreatureV1 {

    /// Creatures brain represented as a neural network
    pub brain : Brain,

    /// ID number of this creature
    pub id : usize,

    /// Indicates whether the creature is alive
    pub is_alive : bool,

    /// Current position in 2d coordinates x, y
    pub position : CreaturePosition,

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

impl CreatureV1 {

    // ============= CONSTRUCTORS ================

    /// Constructor returns creature instance w/ default values
    pub fn new(id : usize) -> CreatureV1 {
        let input_neuron_types = vec![Age, Energy, VisionDistance, VisionColorRed, VisionColorGreen, VisionColorBlue];
        let output_neuron_types = vec![Stay, MoveUp, MoveDown, MoveLeft, MoveRight, RotateCCW, RotateCW];

        let temp_creature = CreatureV1 {
            brain : Brain::new(&input_neuron_types,&output_neuron_types),
            id : id,
            is_alive : true,
            position : CreaturePosition {x : 0, y : 0},
            orientation : DEFAULT_ORIENTATION,
            energy : DEFAULT_ENERGY_LEVEL,
            vision_state : CreatureVisionState {obj_in_view : false, dist : 0, color : CreatureColor::new_from_vec([0,0,0])},
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
    pub fn new_offspring(id : usize, parent : &CreatureV1, mutation_prob : f32) -> CreatureV1 {

        let mut temp_creature = CreatureV1 {
            brain : Brain::new_copy(&parent.brain, mutation_prob),
            id : id,
            is_alive : true,
            position : CreaturePosition {x : parent.position.x, y : parent.position.y},
            orientation : parent.orientation,
            energy : DEFAULT_ENERGY_LEVEL,
            vision_state : CreatureVisionState {obj_in_view : false, dist : 0, color : CreatureColor::new_from_vec([0,0,0])},
            age : 0,
            last_action : CreatureActions::Stay,
            color : CreatureColor::new_from_vec(DEFAULT_CREATURE_COLOR),
            reproduction_age : DEFAULT_REPRODUCE_AGE,
            input_neuron_types : vec![Age, Energy, VisionDistance, VisionColorRed, VisionColorGreen, VisionColorBlue],
            output_neuron_types : vec![Stay, MoveUp, MoveDown, MoveLeft, MoveRight, RotateCCW, RotateCW],
        };

        return temp_creature;
    }


    /// Constructor to create a new creature from a JSON string
    pub fn new_from_json(id : usize, json_in : &str) -> serde_json::Result<CreatureV1> {
        // This is pretty neat we can basically just tell serde_json to copy all
        // fields into a given structure.
        let mut temp_creature : CreatureV1 = serde_json::from_str(json_in)?;

        // Set the ID to the new value
        temp_creature.id = id;

        return Ok(temp_creature);
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

    /// Set reproduction age
    pub fn set_reproduction_age(&mut self, repro_age : usize) {
        self.reproduction_age = repro_age;
    }

    /// Returns true if the creature is dead and false if it is alive
    pub fn is_dead(&self) -> bool {
        if self.energy > 0  && self.age < MAX_POSSIBLE_AGE {
            return false
        } else {
            return true;
        }
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
                _ => {
                    if DEBUG_LEVEL > 0 {
                        println!("Warning: unused/unpopulated input neuron {:?} at idx {}", neuron_type, input_neuron_idx);
                    }
                }
            }
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

        // Reduce the energy and increase age
        if self.energy > 0 {
            self.energy -= 1;
            self.age += 1;
        }

        // If we get to a certain age, then we've survived long enough! Reproduce
        if (self.age % self.reproduction_age) == 0 {
            self.last_action = CreatureActions::Reproduce;
            return CreatureActions::Reproduce;
        }

        // Otherwise, evaluate the brain network based on the current state of the input neurons
        // To check what our next action will be
        let action = self.brain.get_next_action();

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
const BRAIN_V2_LAYER_SIZES : [usize; 4] = [PLACEHOLDER_NUM_NODES, 8, 8, PLACEHOLDER_NUM_NODES]; // 2 internal layers with 8 neurons each. First/Last layer sizes will be specified by constructor
const BRAIN_V2_MIN_INIT_NODE_VAL : f32 = -10.0;  // Min initial value that a node will take
const BRAIN_V2_MAX_INIT_NODE_VAL : f32 = 10.0;   // Max initial value that a node will take 

impl Brain {

    /// Create a new instance of the Brain with random values for all weights/biases
    /// and explicit mapping of input/output neuron types
    /// * `input_node_types` is a vector mapping the input neuron idx to which type of input it is. It
    /// also determines the number of neurons in the input layer by it's size
    /// * `output_node_types` is a vector mapping the output neuron idx to which type of action it is. It
    /// also determines the number of neurons in the output layer by it's size
    pub fn new(input_node_types : &Vec<CreatureInputs>, output_node_types : &Vec<CreatureActions>) -> Brain {
        let mut layer_sizes  = BRAIN_V2_LAYER_SIZES.to_vec();
        let output_layer_idx = layer_sizes.len() - 1;
        layer_sizes[0] = input_node_types.len();
        layer_sizes[output_layer_idx] = output_node_types.len();
        return Brain {
            net : NeuralNet::new(&layer_sizes, BRAIN_V2_MIN_INIT_NODE_VAL, BRAIN_V2_MAX_INIT_NODE_VAL),
            input_node_types : input_node_types.clone(), 
            output_node_types : output_node_types.clone(),
        };
    }

    /// Create a new brain copy, but randomly mutate some of the weights/biases
    /// with chance of mutation for each of mutation_prob 
    pub fn new_copy(other_brain : &Brain, mutation_prob : f32) -> Brain {

        // Copy the neural net and apply random mutations to it
        let mut nn = other_brain.net.clone();
        nn.apply_rand_mutations(mutation_prob, BRAIN_V2_MIN_INIT_NODE_VAL, BRAIN_V2_MAX_INIT_NODE_VAL);

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


