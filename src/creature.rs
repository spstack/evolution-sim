/** ===============================================================================
 * File: creature.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: Includes all code that describes a single creature in the 2D sim
 * ===============================================================================*/

// Define external crates to use in this module
use rand::Rng;
use std::fmt::Debug;

//===============================================================================
// CONSTANTS
//===============================================================================
pub const DEFAULT_ENERGY_LEVEL : usize = 20;
pub const MAX_POSSIBLE_ENERGY : usize = 200;
pub const MAX_POSSIBLE_AGE : usize = 200;

pub const REPRODUCE_AGE : usize = 21;           // Age at which creature will reproduce

const DEBUG_LEVEL : usize = 1;



/// Defines the possible actions that a creature of any type can take
#[derive(Copy, Clone, Debug)]
pub enum CreatureActions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Stay,
    Reproduce,
    // Kill, //....soon
}

/// Defines all possible input neurons to a creature
#[derive(Copy, Clone)]
pub enum CreatureInputs {
    Unused,
    Age,
    Energy,
    VisionUp,
    VisionDown,
    VisionLeft,
    VisionRight,
}

#[derive(Copy, Clone, PartialEq)]
pub struct CreaturePosition {
    pub x : usize, // x position of the creature
    pub y : usize, // y position of the creature
}


// Ensure we don't have to re-type the actions and inputs constantly
use CreatureActions::*;
use CreatureInputs::*;

/// Version 1 of a simple creature. It's only available actions are to move and eat
pub struct CreatureV1 {

    /// Creatures brain represented as a neural network
    pub brain : BrainV1,

    /// ID number of this creature
    pub id : usize,

    /// Indicates whether the creature is alive
    pub is_alive : bool,

    /// Current position in 2d coordinates x, y
    pub position : CreaturePosition,

    /// Current energy level
    pub energy : usize,

    /// Current age of the creature in time-steps 
    pub age : usize,

    input_neuron_types : [CreatureInputs; NUM_INPUT_NODES],
    output_neuron_types : [CreatureActions; NUM_OUTPUT_NODES],
}

impl CreatureV1 {

    /// Constructor returns creature instance w/ default values
    pub fn new(id : usize) -> CreatureV1 {
        let mut temp_creature = CreatureV1 {
            brain : BrainV1::new(),
            id : id,
            is_alive : true,
            position : CreaturePosition {x : 0, y : 0},
            energy : DEFAULT_ENERGY_LEVEL,
            age : 0,
            input_neuron_types : [Age, Energy, VisionUp, VisionDown, VisionLeft, VisionRight],
            output_neuron_types : [Stay, MoveUp, MoveDown, MoveLeft, MoveRight],
        };

        temp_creature.brain.assign_input_node_types(temp_creature.input_neuron_types);
        temp_creature.brain.assign_output_node_types(temp_creature.output_neuron_types);

        return temp_creature;
    }

    /// Constructor to create a new creature from a provided parent (genes copied with optional mutations)
    pub fn new_offspring(id : usize, parent : &CreatureV1) -> CreatureV1 {
        let parent_dna = parent.brain.get_dna_copy();
        println!("Initializing offspring w/ dna {parent_dna:?}");

        let mut temp_creature = CreatureV1 {
            brain : BrainV1::new_from_dna(&parent_dna),
            id : id,
            is_alive : true,
            position : CreaturePosition {x : parent.position.x, y : parent.position.y},
            energy : DEFAULT_ENERGY_LEVEL,
            age : 0,
            input_neuron_types : [Age, Energy, VisionUp, VisionDown, VisionLeft, VisionRight],
            output_neuron_types : [Stay, MoveUp, MoveDown, MoveLeft, MoveRight],
        };

        temp_creature.brain.assign_input_node_types(temp_creature.input_neuron_types);
        temp_creature.brain.assign_output_node_types(temp_creature.output_neuron_types);

        return temp_creature;
    }

    /// Set position in the board
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.position.x = x;
        self.position.y = y;
    }


    /// Sense surroundings (populate the input neurons)
    pub fn sense_surroundings(&mut self) {
        for (input_neuron_idx, neuron_type) in self.input_neuron_types.iter().enumerate() {
            match neuron_type {
                Age => self.brain.set_input(input_neuron_idx, self.age as isize),
                Energy => self.brain.set_input(input_neuron_idx, self.energy as isize),
                _ => {}
            } 
        }

    }

    /// Eat a piece of food that gives it the specified amount of energy
    pub fn eat_food(&mut self, food_energy : usize) {
        if self.energy + food_energy > MAX_POSSIBLE_ENERGY {
            self.energy = MAX_POSSIBLE_ENERGY;
        } else {
            self.energy += food_energy;
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

    // Perform next action (evaluate neural network and decide on next action based on output)
    // Perform any environmental actions like eating nearby food/reproducing/fighting
    pub fn perform_next_action(&mut self) -> CreatureActions {

        if self.is_dead() {
            // Creature is dead, just return stay action
            return CreatureActions::Stay;
        }

        // Reduce the energy and increase age
        if self.energy > 0 {
            self.energy -= 1;
            self.age += 1;
        }

        // If we get to a certain age, then we've survived long enough! Reproduce
        if self.age == REPRODUCE_AGE {
            return CreatureActions::Reproduce;
        }

        // Otherwise, evaluate the brain network based on the current state of the input neurons
        // To check what our next action will be
        self.brain.evaluate_network();

        // Show results!
        if DEBUG_LEVEL > 1 {
            self.brain.show();
        }

        // Get the value of the action to be taken
        let action = self.brain.get_current_action();
        

        return action; 
    }

}


// Define layers and each layer's size
pub const NUM_LAYERS : usize = 3;
pub const LAYER_SIZES : [usize; NUM_LAYERS] = [6, 4, 5];
pub const NUM_INPUT_NODES : usize = LAYER_SIZES[0];
pub const NUM_OUTPUT_NODES : usize = LAYER_SIZES[NUM_LAYERS-1];
pub const NUM_NODES : usize = 15; // Total number of nodes in the network. Must be consistent with LAYER_SIZES
pub const MAX_CONNECTIONS_PER_NODE : usize = 6; // This must be greater than or equal to max layer size

// Define min/max values that input neurons can have and that weights/biases can have
pub const VAL_MIN : isize = -1000;
pub const VAL_MAX : isize = 1000;

pub const DNA_SIZE : usize = NUM_NODES + MAX_CONNECTIONS_PER_NODE * NUM_NODES;

// Define dna type
type Dna = [isize; DNA_SIZE];

/// Second attempt at making a more generic neural network for creature brains
pub struct BrainV1 {

    /// 2D array representing all weights in the network. First index is the id of starting neuron
    /// and second dimension index is the destination node number in the previous layer.
    /// For example, the connection between node ID 5 and the second node in the previous layer from id 5
    /// would be: weights[5][1]
    //pub weights : [[isize; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
    pub weights_flat : [isize; MAX_CONNECTIONS_PER_NODE * NUM_NODES],

    /// Defines the bias values for each neuron in the network. Index is the neuron ID, and value at that 
    /// index is the bias
    pub biases : [isize; NUM_NODES],

    /// "DNA" array that uniquely identifies this brain structure. It's composed of the biases of each neuron
    /// followed by the flattened matrix of weights for each neuron connection in the brain
    pub dna : Dna,

    /// Current value that each neuron (node) is holding
    pub values : [isize; NUM_NODES],

    // Defines the types of the input nodes
    pub input_node_types : [CreatureInputs; NUM_INPUT_NODES],
    pub output_node_types : [CreatureActions; NUM_OUTPUT_NODES],
}


impl BrainV1 {

    /// Constructor to allocate a new BrainV1 instance
    fn new() -> BrainV1 {
        let mut brain = BrainV1 { 
            // weights : [[0; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
            weights_flat : [0; MAX_CONNECTIONS_PER_NODE * NUM_NODES],
            biases : [0; NUM_NODES],
            dna : [0; DNA_SIZE],
            values : [0; NUM_NODES],
            input_node_types : [CreatureInputs::Unused; NUM_INPUT_NODES],
            output_node_types : [CreatureActions::Stay; NUM_OUTPUT_NODES],
        };

        brain.initialize_rand_weights_biases();
        return brain;
    }

    /// Constructor that takes in dna type to 
    fn new_from_dna(dna : &Dna) -> BrainV1 {
        let mut brain = BrainV1 { 
            // weights : [[0; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
            weights_flat : [0; MAX_CONNECTIONS_PER_NODE * NUM_NODES],
            biases : [0; NUM_NODES],
            dna : [0; DNA_SIZE],
            values : [0; NUM_NODES],
            input_node_types : [CreatureInputs::Unused; NUM_INPUT_NODES],
            output_node_types : [CreatureActions::Stay; NUM_OUTPUT_NODES],
        };
        brain.initialize_with_dna(&dna);
        return brain;
    }


    /// Initialize the brain with the weights and biases from the provided DNA
    pub fn initialize_with_dna(&mut self, _dna : &Dna) {

        // Copy dna into this struct
        self.dna = *_dna;

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
                    // self.weights[node_id][dst_idx] = val;
                    self.weights_flat[self.weights_idx(node_id, dst_idx)] = val;
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
    fn assign_input_node_types(&mut self, input_node_types : [CreatureInputs; NUM_INPUT_NODES]) {
        self.input_node_types = input_node_types;
    }

    /// Assign each output node an action to take if that node is activated
    fn assign_output_node_types(&mut self, output_node_types : [CreatureActions; NUM_OUTPUT_NODES]) {
        self.output_node_types = output_node_types;
    }


    /// Convert node id to layer number 
    fn node_id_to_layer_num(&self, node_id : usize) -> usize {
        let mut node_sum = 0;

        // Loop through each layer except last one and determine where the requested node lives
        for layer in 0..LAYER_SIZES.len() {
            node_sum += LAYER_SIZES[layer];
            if node_id < node_sum {
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
        let ret_dna : Dna = self.dna;
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
                    // self.values[row] += self.weights[row][col] * self.values[prev_layer_start_node + col];
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
        // Print the weights
        // println!("WEIGHTS");
        // for node in 0..NUM_NODES {
        //     for dst_idx in 0..MAX_CONNECTIONS_PER_NODE {
        //         print!(" {:6}", self.weights[node][dst_idx]);
        //     }
        //     println!();
        // }

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


/// Single network layer
struct NetLayer {
    num_nodes : usize,          // Number of nodes in this layer
    activations : Vec<isize>,   // activation values of each node in this layer
    weights : Vec<isize>,       // weights matrix. Will be of size <prev
    biases : Vec<isize>,
}


pub struct BrainV2 {
}


/*pub struct NeuralNetV1 {

    pub input_layer_bias : [isize; NUM_INPUT_NODES], // column vector of bias
    pub input_layer_vals : [isize; NUM_INPUT_NODES], // column vector of current values each input layer neuron holds

    pub hidden_layer1_bias : [isize; NUM_HIDDEN1_NODES],
    pub hidden_layer1_vals : [isize; NUM_HIDDEN1_NODES],
    pub hidden_layer1_weights : [[isize; NUM_HIDDEN1_NODES]; NUM_INPUT_NODES], // weights are a 2D array where [x][y] is the weight for connection between node x in first layer and node y in the second

    pub output_layer_bias : [isize; NUM_OUTPUT_NODES],
    pub output_layer_vals : [isize; NUM_OUTPUT_NODES],
    pub output_layer_weights : [[isize; NUM_OUTPUT_NODES]; NUM_HIDDEN1_NODES], // weights are a 2D array where [x][y] is the weight for connection between node x in first layer and node y in the second
    
}

/// Implement the neural network part of the brain
impl NeuralNetV1 {

    pub fn initialize_rand(&mut self) {
        let mut rng = rand::thread_rng();

        // First initialize input bias
        for idx in 0..NUM_INPUT_NODES {
            let val : isize = rng.gen_range(VAL_MIN..VAL_MAX+1);
            self.input_layer_bias[idx] = val;
        }

        // input -> hidden1 weights
        for row in 0..NUM_INPUT_NODES {
            for col in 0..NUM_HIDDEN1_NODES {
                let val : isize = rng.gen_range(VAL_MIN..VAL_MAX+1);
                self.hidden_layer1_weights[row][col] = val;
            }
        }

        // hidden1 bias
        for idx in 0..NUM_HIDDEN1_NODES {
            let val : isize = rng.gen_range(VAL_MIN..VAL_MAX+1);
            self.hidden_layer1_bias[idx] = val;
        }

        // hidden1 -> output weights
        for row in 0..NUM_HIDDEN1_NODES {
            for col in 0..NUM_OUTPUT_NODES {
                let val : isize = rng.gen_range(VAL_MIN..VAL_MAX+1);
                self.output_layer_weights[row][col] = val;
            }
        }

        // output bias
        for idx in 0..NUM_OUTPUT_NODES {
            let val : isize = rng.gen_range(VAL_MIN..VAL_MAX+1);
            self.output_layer_bias[idx] = val;
        }
    }

    fn set_input(&mut self, index : usize, val : isize) {
        self.input_layer_vals[index] = val;
    }

    fn evaluate_network(&self) {

    }

    pub fn show(&self) {

        println!("INPUT BIAS");
        for idx in 0..NUM_INPUT_NODES {
            println!(" {:6}", self.input_layer_bias[idx]);
        }
        println!();

        println!("INPUT -> HIDDEN1 WEIGHTS");
        for row in 0..NUM_INPUT_NODES {
            for col in 0..NUM_HIDDEN1_NODES {
                print!(" {:6}", self.hidden_layer1_weights[row][col]);
            }
            println!();
        }
        println!();

        println!("HIDDEN1 BIAS");
        for idx in 0..NUM_HIDDEN1_NODES {
            println!(" {:6}", self.hidden_layer1_bias[idx]);
        }
        println!();

        println!("HIDDEN1 -> OUTPUT WEIGHTS");
        for row in 0..NUM_HIDDEN1_NODES {
            for col in 0..NUM_OUTPUT_NODES {
                print!(" {:6}", self.output_layer_weights[row][col]);
            }
            println!();
        }
        println!();

        println!("OUTPUT BIAS");
        for idx in 0..NUM_OUTPUT_NODES {
            println!(" {:6}", self.output_layer_bias[idx]);
        }
        println!();
    }
}
*/


/*
/// Defines a single neuron
struct Neuron {
    id : u32;
    neuron_type : NeuronType;
    num_inputs : u32;
    num_outputs : u32;
    layer : u32;
    // inputs : [&NeuronConnection; MAX_CONNECTIONS_PER_NODE];
    value : i32;
    bias : i32;
}

/// Implementation of neuron
impl Neuron {

}


struct NeuronConnection {
    src_neuron : &Neuron;
    dst_neuron : &Neuron;
    weight : i32;
}

/// Defines types of neurons
enum NeuronType {
    Input(InputNeuronType),
    Middle, 
    Action(ActionNeuronType),
}

/// Defines types of output neurons (ones that result in things happening)
enum ActionNeuronType {
    MoveX(i32),
    MoveY(i32),
    Kill,
    Die,
}

/// Defines types on input neurons
enum InputNeuronType {
    SightFood,
    SightPredator,
    Hunger,
    Age,
    NumOffspring,
}
*/
    

