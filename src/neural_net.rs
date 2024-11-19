/** ===============================================================================
 * File: neural_net.rs
 * Author: Scott Stack
 * Description: Implements generic neural network that serves as the "brain" for
 * creatures in the environment
 * ===============================================================================*/


/// Implements a simple generic neural network for use as a brain for creatures
/// The generic represents the underlying type that network values/weights/biases
/// will use (f32, isize, etc...)
pub struct NeuralNet<T> {
    // num layers
    num_layers : usize,

    // weights - vector of Matrices index n is the matrix representing weights between layer n and n+1
    weights : Vec<Matrix<T>>,

    // biases - List of vectors (first)
    biases : Vec<Vec<T>>,

}


/// Implementation of generic NeuralNetwork
impl NeuralNet {

    /// Get a new randomly populated network
    pub fn new(layer_sizes : Vec<usize>) -> NeuralNet {

    }

    /// Evaluate the network (feed-forward) and return output node number with highest activation 
    pub fn evaluate_network() -> usize {

    }

    /// Set value of specified input node
    pub fn set_input_node(input_node_idx : usize, val : T) {

    }

}


