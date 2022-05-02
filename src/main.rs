
/** ===============================================================================
 * File: main.rs
 * Author: Scott Stack
 * Created: 4/29/2022
 * Description: main application entry point
 * ===============================================================================*/

mod creature;
mod environment;
// mod matrix_math;
use creature::creature_v1::*;

/// Main application entry point
fn main() {

    // let mut net = Brain {
    //     weights : [[0; MAX_CONNECTIONS_PER_NODE]; NUM_NODES],
    //     weights_flat : [0; MAX_CONNECTIONS_PER_NODE * NUM_NODES],
    //     biases : [0; NUM_NODES],
    //     values : [0; NUM_NODES],
    //     dna : [0; DNA_SIZE],
    // };

    let mut creature = get_new_creature(1);

    creature.brain.show();
    creature.perform_next_action();

    // let mut brain = get_new_brain();
    // brain.set_input(0, 123);
    // brain.set_input(1, 321);
    // brain.show();

    // brain.evaluate_network();
    // brain.show();

    // net.initialize();
    // net.show();

    // net.set_input(0, 123);
    // net.set_input(1, 321);
    // // net.show();

    // net.evaluate_network();
    // net.show();

    // Instantiate a neural net
    // let mut x = NeuralNetV1 {
    //     input_layer_bias : [0; NUM_INPUT_NODES],
    //     input_layer_vals : [0; NUM_INPUT_NODES],

    //     hidden_layer1_bias : [0; NUM_HIDDEN1_NODES],
    //     hidden_layer1_vals : [0; NUM_HIDDEN1_NODES],
    //     hidden_layer1_weights : [[0; NUM_HIDDEN1_NODES]; NUM_INPUT_NODES],

    //     output_layer_bias : [0; NUM_OUTPUT_NODES],
    //     output_layer_vals : [0; NUM_OUTPUT_NODES],
    //     output_layer_weights : [[0; NUM_OUTPUT_NODES]; NUM_HIDDEN1_NODES],
    // };

    // // Initialize the weights and biases to random values
    // x.initialize_rand();
    // x.show();

}
