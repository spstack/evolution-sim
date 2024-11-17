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
use rand::Rng;
use std::fmt::Debug;
use crate::linalg::*;

use serde::{Deserialize, Serialize};
use serde_json::{Value, Result};

//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEFAULT_ENERGY_LEVEL : usize = 20;
pub const MAX_POSSIBLE_ENERGY : usize = 200;
pub const MAX_POSSIBLE_AGE : usize = 200;

pub const REPRODUCE_AGE : usize = 21;                       // Age at which creature will reproduce
pub const DEFAULT_CREATURE_COLOR : [u8; 3] = [0, 0, 255];   // default color each creature will be (blue)
pub const DEFAULT_ORIENTATION : CreatureOrientation = CreatureOrientation::Up; // Which way creature will face by default

pub const VISION_NEURON_INVALID_VAL : isize = -10000;         // Value that should be applied to a vision input neuron if there's nothing in view

const DEBUG_LEVEL : usize = 1;  // Debug print level (higher number = more detail)


/// Defines the possible actions that a creature of any type can take
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum CreatureActions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateCW,   // rotate clockwise
    RotateCCW,  // rotate counter-clockwise
    Stay,
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

/// Version 1 of a simple creature. It contains all of the state information about a creature
/// The environment that creates the creature should call ``
#[derive(Serialize, Deserialize)]
pub struct CreatureV1 {

    /// Creatures brain represented as a neural network
    pub brain : BrainV1,

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
        let mut temp_creature = CreatureV1 {
            brain : BrainV1::new(),
            id : id,
            is_alive : true,
            position : CreaturePosition {x : 0, y : 0},
            orientation : DEFAULT_ORIENTATION,
            energy : DEFAULT_ENERGY_LEVEL,
            vision_state : CreatureVisionState {obj_in_view : false, dist : 0, color : CreatureColor::new_from_vec([0,0,0])},
            age : 0,
            last_action : CreatureActions::Stay,
            color : CreatureColor::new_from_vec(DEFAULT_CREATURE_COLOR),
            input_neuron_types : vec![Age, Energy, VisionDistance, VisionColorRed, VisionColorGreen, VisionColorBlue],
            output_neuron_types : vec![Stay, MoveUp, MoveDown, MoveLeft, MoveRight, RotateCCW, RotateCW],
        };

        // Initialize the neural network
        temp_creature.brain.assign_input_node_types(&temp_creature.input_neuron_types);
        temp_creature.brain.assign_output_node_types(&temp_creature.output_neuron_types);

        return temp_creature;
    }

    /// Constructor to create a new creature from a provided parent (genes copied with optional mutations)
    pub fn new_offspring(id : usize, parent : &CreatureV1, mutation_prob : f32) -> CreatureV1 {
        let parent_dna = parent.brain.get_dna_copy();
        if DEBUG_LEVEL > 1 {
            println!("Initializing offspring w/ dna {parent_dna:?}");
        }

        let mut temp_creature = CreatureV1 {
            brain : BrainV1::new_from_dna(&parent_dna, mutation_prob),
            id : id,
            is_alive : true,
            position : CreaturePosition {x : parent.position.x, y : parent.position.y},
            orientation : CreatureOrientation::Up,
            energy : DEFAULT_ENERGY_LEVEL,
            vision_state : CreatureVisionState {obj_in_view : false, dist : 0, color : CreatureColor::new_from_vec([0,0,0])},
            age : 0,
            last_action : CreatureActions::Stay,
            color : CreatureColor::new_from_vec(DEFAULT_CREATURE_COLOR),
            input_neuron_types : vec![Age, Energy, VisionDistance, VisionColorRed, VisionColorGreen, VisionColorBlue],
            output_neuron_types : vec![Stay, MoveUp, MoveDown, MoveLeft, MoveRight, RotateCCW, RotateCW],
        };

        temp_creature.brain.assign_input_node_types(&temp_creature.input_neuron_types);
        temp_creature.brain.assign_output_node_types(&temp_creature.output_neuron_types);

        return temp_creature;
    }


    /// Constructor to create a new creature from a JSON string
    pub fn new_from_json(id : usize, json_in : &str) -> Result<CreatureV1> {
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
        let mut vis_dist : isize = 0;
        let mut vis_red : isize = 0;
        let mut vis_green : isize = 0;
        let mut vis_blue : isize = 0;

        if self.vision_state.obj_in_view {
            vis_dist = self.vision_state.dist as isize;
            vis_red = self.vision_state.color.red as isize;
            vis_green = self.vision_state.color.green as isize;
            vis_blue = self.vision_state.color.blue as isize;
        } else {
            vis_dist = VISION_NEURON_INVALID_VAL;
            vis_red = VISION_NEURON_INVALID_VAL;
            vis_green = VISION_NEURON_INVALID_VAL;
            vis_blue = VISION_NEURON_INVALID_VAL;
        }


        // Loop through each input neuron and set it depending on the type
        for (input_neuron_idx, neuron_type) in self.input_neuron_types.iter().enumerate() {
            match neuron_type {
                Age => self.brain.set_input(input_neuron_idx, self.age as isize),
                Energy => self.brain.set_input(input_neuron_idx, self.energy as isize),
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
        if self.age == REPRODUCE_AGE {
            self.last_action = CreatureActions::Reproduce;
            return CreatureActions::Reproduce;
        }

        // Otherwise, evaluate the brain network based on the current state of the input neurons
        // To check what our next action will be
        self.brain.evaluate_network();

        // Get the value of the action to be taken
        let action = self.brain.get_current_action();
        self.last_action = action;

        // Show the state of the brain if debug level high enough
        if DEBUG_LEVEL > 1 {
            self.brain.show();
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


// Define layers and each layer's size
pub const NUM_LAYERS : usize = 3;
pub const LAYER_SIZES : [usize; NUM_LAYERS] = [6, 4, 7];
pub const NUM_INPUT_NODES : usize = LAYER_SIZES[0];
pub const NUM_OUTPUT_NODES : usize = LAYER_SIZES[NUM_LAYERS-1];
pub const NUM_NODES : usize = 18; // Total number of nodes in the network. Must be consistent with LAYER_SIZES
pub const MAX_CONNECTIONS_PER_NODE : usize = 8; // This must be greater than or equal to max layer size

// Define min/max initial values that input neurons can have and that weights/biases can have
pub const VAL_MIN : isize = -1000;
pub const VAL_MAX : isize = 1000;

pub const DNA_SIZE : usize = NUM_NODES + MAX_CONNECTIONS_PER_NODE * NUM_NODES;

// Define dna type
type Dna = Vec<isize>;

/// Second attempt at making a more generic neural network for creature brains
/// TODO: This needs to be reworked to be more general...
#[derive(Serialize, Deserialize)]
pub struct BrainV1 {
    /// flat array representing all of the weights in the network. The weight between node X and node Y
    /// would be 
    pub weights_flat : Vec<isize>,

    /// Defines the bias values for each neuron in the network. Index is the neuron ID, and value at that 
    /// index is the bias
    pub biases : Vec<isize>,

    /// "DNA" array that uniquely identifies this brain structure. It's composed of the biases of each neuron
    /// followed by the flattened matrix of weights for each neuron connection in the brain
    pub dna : Dna,

    /// Current value that each neuron (node) is holding
    pub values : Vec<isize>,

    // Defines the types of the input and output nodes just for diagnostic info
    pub input_node_types : Vec<CreatureInputs>,
    pub output_node_types : Vec<CreatureActions>,
}


impl BrainV1 {

    /// Constructor to allocate a new BrainV1 instance
    fn new() -> BrainV1 {
        let mut brain = BrainV1 { 
            weights_flat : vec![0; MAX_CONNECTIONS_PER_NODE * NUM_NODES],
            biases : vec![0; NUM_NODES],
            dna : vec![0; DNA_SIZE],
            values : vec![0; NUM_NODES],
            input_node_types : vec![CreatureInputs::Unused; NUM_INPUT_NODES],
            output_node_types : vec![CreatureActions::Stay; NUM_OUTPUT_NODES],
        };

        brain.initialize_rand_weights_biases();
        return brain;
    }

    /// Constructor that takes in dna type to 
    fn new_from_dna(dna : &Dna, mutation_prob : f32) -> BrainV1 {
        // Create new temp brain
        let mut brain = BrainV1 { 
            // weights : [[0; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
            weights_flat : vec![0; MAX_CONNECTIONS_PER_NODE * NUM_NODES],
            biases : vec![0; NUM_NODES],
            dna : vec![0; DNA_SIZE],
            values : vec![0; NUM_NODES],
            input_node_types : vec![CreatureInputs::Unused; NUM_INPUT_NODES],
            output_node_types : vec![CreatureActions::Stay; NUM_OUTPUT_NODES],
        };
        brain.initialize_with_dna(&dna);

        // apply mutations to new DNA
        brain.apply_random_dna_mutation(mutation_prob);

        return brain;
    }


    /// Initialize the brain with the weights and biases from the provided DNA
    pub fn initialize_with_dna(&mut self, _dna : &Dna) {

        // Copy dna into this struct
        self.dna = _dna.clone();

        // First populate biases
        for i in 0..NUM_NODES {
            self.biases[i] = _dna[i];
        }

        // Next populate flat weights array
        let mut w_idx : usize = 0;
        for dna_idx in NUM_NODES..DNA_SIZE {
            self.weights_flat[w_idx] = _dna[dna_idx];
            w_idx += 1;
        }
    }

    /// Apply random mutations to this brains "DNA" where probability of a single
    /// value being changed to a random number is given by `mutation_prob` [0, 1]
    fn apply_random_dna_mutation(&mut self, mutation_prob : f32) {
        let mut rng = rand::thread_rng();
        for idx in 0..self.dna.len() {
            // gen() generates a floating point between 0 and 1 if less than the mutation prob, generate a new value
            if rng.gen::<f32>() <= mutation_prob {
                self.dna[idx] = rng.gen_range(VAL_MIN..=VAL_MAX);
            }
        }
    }

    /// Initialize the weights and biases in the network with random values
    pub fn initialize_rand_weights_biases(&mut self) {

        // Initialize the rng
        let mut rng = rand::thread_rng();

        // For each node, find out it's layer, then find how many nodes it connects to.
        for node_id in 0..NUM_NODES {
            let curr_layer = self.node_id_to_layer_num(node_id);

            // Loop over number of nodes in the previous layer to define connections between this node and 
            // all nodes in previous layer. Input layer does not have weights which means there will be some
            // blank spots in the beginning of the weights array
            if curr_layer > 0 {
                for dst_idx in 0..LAYER_SIZES[curr_layer - 1] {
                    let val : isize = rng.gen_range(VAL_MIN..VAL_MAX+1);
                    let idx = self.weights_idx(node_id, dst_idx);
                    self.weights_flat[idx] = val;
                }
            }

            // Initialize bias for this node_id while we're at it
            self.biases[node_id] = rng.gen_range(VAL_MIN..VAL_MAX+1);
        }

        // Update the DNA array
        self.update_dna();
    }

    /// Assign each input node to a type of sensory input. Each index in the array represents the input type
    /// that the corresponding input node takes
    fn assign_input_node_types(&mut self, input_node_types : &Vec<CreatureInputs>) {
        if input_node_types.len() != NUM_INPUT_NODES {
            panic!("Error! Number of input node types does not match!");
        }
        self.input_node_types = input_node_types.clone();
    }

    /// Assign each output node an action to take if that node is activated
    fn assign_output_node_types(&mut self, output_node_types : &Vec<CreatureActions>) {
        if output_node_types.len() != NUM_OUTPUT_NODES {
            panic!("Error! Number of output node types does not match!");
        }
        self.output_node_types = output_node_types.clone();
    }


    /// Convert node id to layer number 
    fn node_id_to_layer_num(&self, node_id : usize) -> usize {
        let mut node_sum = 0;

        // Loop through each layer except last one and determine where the requested node lives
        for layer in 0..LAYER_SIZES.len() {
            node_sum += LAYER_SIZES[layer];
            if node_id <= node_sum {
                return layer;
            }
        }

        println!("Invalid node_id! node_id = {} sum = {}", node_id, node_sum);
        panic!("Invalid node_id!");
    }

    /// Get the starting node_id of the given layer
    fn layer_to_starting_node(&self, layer : usize) -> usize {

        // Check input first
        if layer >= NUM_LAYERS {
            panic!("Invalid layer number {}!", layer);
        }

        return LAYER_SIZES[0..layer].iter().sum();
    }


    /// Calculate the index into the weights matrix array given src neuron ID and the destination node index
    /// For example, if we wanted to know the weight between neuron id 2 and the second neuron in the next layer,
    /// we'd access self.weights(self.weights_idx(2, 1))
    pub fn weights_idx(&self, src_node : usize, dst_idx : usize) -> usize {
        return (MAX_CONNECTIONS_PER_NODE  * src_node) + dst_idx;
    }


    /// Update DNA from the current weights and biases
    pub fn update_dna(&mut self) {
        let mut dna_idx = 0;

        // Populate the biases first
        for idx in 0..self.biases.len() {
            self.dna[dna_idx] = self.biases[idx];
            dna_idx += 1;
        }

        // Populate the weights next
        for idx in 0..self.weights_flat.len() {
            self.dna[dna_idx] = self.weights_flat[idx];
            dna_idx += 1;
        }
    }

    /// Get a copy of the DNA array for potential use in creating another 
    /// dna_out - reference to dna array where 
    pub fn get_dna_copy(&self) -> Dna {
        let ret_dna : Dna = self.dna.clone();
        return ret_dna;
    }

    /// Evaluate the neural network with the inputs previously provided to `set_input`
    pub fn evaluate_network(&mut self) {
        // For each layer starting at second layer (input values are given in first layer)
        for layer_num in 1..NUM_LAYERS {

            // To start, calculate some parameters that will allow us to compute the new neuron values
            let prev_layer_start_node = self.layer_to_starting_node(layer_num - 1);
            let curr_layer_start_node = self.layer_to_starting_node(layer_num);
            let num_nodes_curr_layer = LAYER_SIZES[layer_num];
            let num_nodes_prev_layer = LAYER_SIZES[layer_num-1];
            let next_layer_start_node = curr_layer_start_node + num_nodes_curr_layer;

            // println!("prev_layer = {} curr_layer = {} nodes_prev = {} curr_nodes = {}", prev_layer_start_node, curr_layer_start_node, num_nodes_prev_layer, num_nodes_curr_layer);

            // Perform matrix multiplication to calculate the new values in each of the nodes in this current layer
            for row in curr_layer_start_node..next_layer_start_node {
                self.values[row] = 0; // Clear value stored in this neuron
                for col in 0..num_nodes_prev_layer { 
                    self.values[row] += self.weights_flat[self.weights_idx(row, col)] * self.values[prev_layer_start_node + col];
                    // println!("values[{}] = {} | weights[{}] = {}", row, self.values[row], self.weights_idx(row,col), self.weights_flat[self.weights_idx(row, col)]);
                }
            }

            // Calculate biases and activation functions on the values
            for row in curr_layer_start_node..next_layer_start_node {

                // Add the biases to the results
                self.values[row] += self.biases[row];

                // Evaluate activation function on values (using simple RELU here)
                if self.values[row] < 0 {
                    self.values[row] = 0;
                }
            }

        }
    }

    /// Set input value for a neuron
    pub fn set_input(&mut self, neuron_id : usize, value : isize) {
        if neuron_id > NUM_INPUT_NODES {
            panic!("Invalid neuron_id, must be an input neuron!");
        }

        self.values[neuron_id] = value;
    }


    /// Get the computed value of the requested output neuron (node)
    pub fn get_output_val(&self, output_node : usize) -> isize {
        if output_node > NUM_OUTPUT_NODES {
            panic!("Invalid output_node number!");
        }

        let output_layer_start_node = self.layer_to_starting_node(NUM_LAYERS-1);
        return self.values[output_layer_start_node + output_node];
    }

    /// Get the ID of the output neuron with the highest value as well as the actual value
    /// This will be called after evaluate_network and corresponds to the action the creature 
    /// will take
    pub fn get_current_action(&self) -> CreatureActions {
        let output_layer_start_node = self.layer_to_starting_node(NUM_LAYERS-1);
        let mut max_idx : usize = usize::MAX;
        let mut max_val : isize = isize::MIN;

        for (idx, val) in self.values[output_layer_start_node..NUM_NODES].iter().enumerate() {
            if *val > max_val {
                max_idx = idx;
                max_val = *val;
            }
        }

        // Sanity check output
        if max_idx == usize::MAX {
            panic!("Max value in output neuron not found or no output neurons defined?");
        }

        if DEBUG_LEVEL > 2 {
            println!("Highest output neuron is idx {} w/ value {}", max_idx, max_val);
        }

        // Return the action type associated with the index that had the highest values
        // The actual value doesn't really matter.
        return self.output_node_types[max_idx];
    }


    /// Print out the weights and biases
    pub fn show(&self) {
        println!("\nWEIGHTS FLAT");
        for node in 0..NUM_NODES {
            for dst_idx in 0..MAX_CONNECTIONS_PER_NODE {
                print!(" {:6}", self.weights_flat[self.weights_idx(node, dst_idx)]);
            }
            println!();
        }

        // Print the biases
        println!("\nBIASES");
        for node in 0..NUM_NODES {
            println!(" {:6}", self.biases[node]);
        }

        // Values 
        println!("\nVALUES");
        for node in 0..NUM_NODES {
            println!(" {:6}", self.values[node]);
        }
    }
}


