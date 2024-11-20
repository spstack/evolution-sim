use std::vec;

/** ===============================================================================
 * File: neural_net.rs
 * Author: Scott Stack
 * Description: Implements generic neural network that serves as the "brain" for
 * creatures in the environment
 * ===============================================================================*/
use crate::linalg::*;


/// Implements a simple generic neural network for use as a brain for creatures
/// The generic represents the underlying type that network values/weights/biases
/// will use (f32, isize, etc...)
pub struct NeuralNet<T> {
    // num layers
    pub num_layers : usize,

    // weights - vector of Matrices index n is the matrix representing weights between layer n and n+1
    pub weights : Vec<Matrix<T>>,

    // biases - List of vectors (nx1 matrix types) where each index into the vector is the layers
    pub biases : Vec<Matrix<T>>,

    // Activations of each neuron n the network. Matrix is an nx1 vector
    pub activations : Vec<Matrix<T>>,
}


/// Implementation of generic NeuralNetwork
impl<T> NeuralNet<T> where 
T: std::ops::Add<Output = T>,
T: std::ops::Sub<Output = T>,
T: std::ops::Mul<Output = T>,
T: std::ops::AddAssign,
T: std::default::Default,
T: Copy,
T: std::cmp::PartialOrd,
T: rand::distributions::uniform::SampleUniform,
{

    /// Get a new randomly populated network
    pub fn new(layer_sizes : Vec<usize>, min_node_val : T, max_node_val : T) -> NeuralNet<T> {
        let mut weights : Vec<Matrix<T>> = Vec::new();
        let mut biases : Vec<Matrix<T>> = Vec::new();

        // Populate activation values for first (input) layer. `biases` does not need this
        // because there are no weights for the input layer
        let mut activations : Vec<Matrix<T>> = vec![Matrix::new(layer_sizes[0], 1)];

        for layer_num in 0..(layer_sizes.len() - 1) {
            let cur_layer_size = layer_sizes[layer_num];
            let next_layer_size = layer_sizes[layer_num+1];

            weights.push(Matrix::random(cur_layer_size, next_layer_size, min_node_val, max_node_val));
            biases.push(Matrix::random(next_layer_size, 1, min_node_val, max_node_val));
            activations.push(Matrix::new(next_layer_size, 1));
        }

        return NeuralNet::<T> {
            num_layers : layer_sizes.len(),
            weights : weights,
            biases : biases,
            activations : activations,
        }
    }


    /// Evaluate the network (feed-forward) and return output node number with highest activation 
    pub fn evaluate_network(&mut self) -> usize {

        // calculate the activation for each layer
        for layer_num in 0..self.num_layers {
            self.activations[layer_num+1] = self.weights[layer_num].mult(&self.activations[layer_num]); 
            self.activations[layer_num+1] = self.activations[layer_num+1].add(&self.biases[layer_num]);

            // Use activation function
            // TODO: Finish this and fix borrow checker
            // self.relu(&mut self.activations[layer_num+1]);
        }

        return 0usize;
    }

    /// Set value of specified input node
    pub fn set_input_node(&mut self, input_node_idx : usize, val : T) {

    }

    /// Perform RELU activation function in-place on nx1 matrix
    fn relu(&self, vector : &mut Matrix<T>) {
        if vector.get_ncols() != 1 {
            panic!("Attempted to perform RELU on non-vector with {} cols", vector.get_ncols());
        }

        for i in 0..vector.get_nrows() {
            // if vector.get(i, 1) >= 0 {
            // TODO: Fix
            // }
        }

    }

}



#[cfg(test)]
mod neuralnet_test {
    use super::*;

    #[test]
    fn test_neural_net_init() {
        let layer_sizes = vec![5, 4, 4, 4, 8];
        let nn = NeuralNet::<isize>::new(layer_sizes, -1000, 1000);

        // Check that couple weights are within bounds
        assert!(nn.weights[0].get(0,0) < 1000 && nn.weights[0].get(0,0) > -1000);
        assert!(nn.weights[1].get(0,0) < 1000 && nn.weights[1].get(0,0) > -1000);
        assert!(nn.weights[1].get(1,0) < 1000 && nn.weights[1].get(1,0) > -1000);
    }
}
