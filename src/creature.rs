/** ===============================================================================
 * File: creature.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: Includes all code that describes a single creature in the 2D sim
 * ===============================================================================*/


pub mod creature_v1 {

    use rand::Rng;
    // use nalgebra as na;
    // use crate::matrix_math;

    const DEFAULT_ENERGY_LEVEL : usize = 100;

    /// Top level function to instantiate a new creature and return it
    pub fn get_new_creature(id : usize) -> Creature {
        let creat = Creature {
            brain : get_new_brain(),
            creature_id : id,
            position : [0; 2],
            energy : DEFAULT_ENERGY_LEVEL,
            age : 0,
        };

        return creat;
    }

    /// Define the actual creature
    pub struct Creature {

        /// Creatures brain represented as a neural network
        pub brain : Brain,

        /// ID number of this creature
        pub creature_id : usize,

        /// Current position in 2d coordinates x, y
        pub position : [usize; 2],

        /// Current energy level
        pub energy : usize,

        /// Current age of the creature in time-steps 
        pub age : usize,
    }

    impl Creature {

        // Sense surroundings (populate the input neurons)
        pub fn sense_surroundings(&self) {

        }

        // Perform next action (evaluate neural network and decide on next action based on output)
        // Perform any environmental actions like eating nearby food/reproducing/fighting
        pub fn perform_next_action(&mut self) {
            self.brain.evaluate_network();
            self.brain.show();
        }

        // Check new surroundings for food


    }

    /// Allocate and return a new randomly generated brain
    pub fn get_new_brain() -> Brain {
        let mut brain = Brain { 
            weights : [[0; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
            weights_flat : [0; MAX_CONNECTIONS_PER_NODE * NUM_NODES],
            biases : [0; NUM_NODES],
            values : [0; NUM_NODES],
            dna : [0; DNA_SIZE],
        };

        brain.initialize();
        return brain;
    }

    // Define layers and each layer's size
    pub const NUM_LAYERS : usize = 3;
    pub const LAYER_SIZES : [usize; NUM_LAYERS] = [2, 4, 2];
    pub const NUM_NODES : usize = 8; // Total number of nodes in the network. Must be consistent with LAYER_SIZES
    pub const MAX_CONNECTIONS_PER_NODE : usize = 4; // This must be greater than or equal to max layer size

    // Define min/max values that input neurons can have and that weights/biases can have
    pub const VAL_MIN : isize = -1000;
    pub const VAL_MAX : isize = 1000;

    pub const DNA_SIZE : usize = NUM_NODES + MAX_CONNECTIONS_PER_NODE * NUM_NODES;
    
    /// Second attempt at making a more generic neural network for creature brains
    pub struct Brain {

        /// 2D array representing all weights in the network. First index is the id of starting neuron
        /// and second dimension index is the destination node number in the previous layer.
        /// For example, the connection between node ID 5 and the second node in the previous layer from id 5
        /// would be: weights[5][1]
        pub weights : [[isize; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
        pub weights_flat : [isize; MAX_CONNECTIONS_PER_NODE * NUM_NODES],

        /// Defines the bias values for each neuron in the network. Index is the neuron ID, and value at that 
        /// index is the bias
        pub biases : [isize; NUM_NODES],

        /// "DNA" array that uniquely identifies this brain structure. It's composed of the biases of each neuron
        /// followed by the flattened matrix of weights for each neuron connection in the brain
        pub dna : [isize; DNA_SIZE],

        /// Current value that each neuron (node) is holding
        pub values : [isize; NUM_NODES],
    }


    impl Brain {

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

        /// Initialize the weights and biases in the network with random values
        pub fn initialize(&mut self) {

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
                        self.weights[node_id][dst_idx] = val;
                        self.weights_flat[self.weights_idx(node_id, dst_idx)] = val;
                    }
                }

                // Initialize bias for this node_id while we're at it
                self.biases[node_id] = rng.gen_range(VAL_MIN..VAL_MAX+1);
            }

            // Update the DNA array
            self.update_dna();
        }


        /// Evaluate the neural network with the inputs previously provided to `set_input`
        /// Return a slice that points to the output neuron values
        pub fn evaluate_network(&mut self) {
            // For each layer starting at second layer (input values are given in first layer)
            for layer_num in 1..NUM_LAYERS {

                let prev_layer_start_node = self.layer_to_starting_node(layer_num - 1);
                let curr_layer_start_node = self.layer_to_starting_node(layer_num);
                let num_nodes_curr_layer = LAYER_SIZES[layer_num];
                let num_nodes_prev_layer = LAYER_SIZES[layer_num-1];

                println!("prev_layer = {} curr_layer = {} nodes_prev = {} curr_nodes = {}", prev_layer_start_node, curr_layer_start_node, num_nodes_prev_layer, num_nodes_curr_layer);

                // construct matrix of weights: weights[prev_node..curr_node][num_nodes_curr_layer]
                // let weights_slice = &self.weights[prev_layer_start_node..curr_layer_start_node][0..num_nodes_curr_layer];
                // let weights_slice = &self.weights[prev_layer_start_node..curr_layer_start_node];

                // let weight_start_idx = self.weights_idx(prev_layer_start_node, 0);
                // let weight_end_idx = self.weights_idx(curr_layer_start_node, 0);
                // let weights_flat = &self.weights_flat[weight_start_idx..weight_end_idx];
                // let weights_mat = na::DMatrix::from_row_slice(num_nodes_prev_layer, num_nodes_curr_layer, weights_slice);
                
                // construct array of values from previous layer
                // let val_slice = &self.values[prev_layer_start_node..curr_layer_start_node];
                // let val_vec = na::DVector::from_column_slice(val_slice);

                // construct array of bias values for current layer
                let next_layer_start_node = curr_layer_start_node + num_nodes_curr_layer;
                // let bias_slice = &self.biases[curr_layer_start_node..next_layer_start_node];
                // let bias_vec = na::DVector::from_column_slice(bias_slice);

                // Do the linear algebras!
                // let new_vals = &self.values[curr_layer_start_node..next_layer_start_node];
                // matrix_math::matrix_vec_mult::<isize, MAX_CONNECTIONS_PER_NODE, MAX_CONNECTIONS_PER_NODE>(new_vals, weights_slice, val_slice);
                // println!("{:?}", weights_mat * val_vec + bias_vec);
                // values = W * a + b
                // matrix_math::matrix_vec_mult_flat(new_vals, weights_flat, val_slice, num_nodes_curr_layer, num_nodes_prev_layer, MAX_CONNECTIONS_PER_NODE);

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
            if neuron_id > LAYER_SIZES[0] {
                panic!("Invalid neuron_id, must be an input neuron!");
            }

            self.values[neuron_id] = value;
        }


        pub fn get_output(&self, output_neuron_id : usize) {
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

            println!("WEIGHTS FLAT");
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
        

}
