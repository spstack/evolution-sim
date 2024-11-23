/** ===============================================================================
 * File: neural_net.rs
 * Author: Scott Stack
 * Description: Implements generic neural network that serves as the "brain" for
 * creatures in the environment
 * ===============================================================================*/
use crate::linalg::*;
use rand::Rng;
use macroquad::input;
use num;


/// Possible error types to be returned from neural network functions
#[derive(Debug)]
pub enum NeuralNetErrors {
    OUTPUT_ACTIVATION_ERROR,    // Error finding the max activation value of an output neuron after evaluating network
}

/// Implements a simple generic neural network for use as a brain for creatures
/// The generic represents the underlying type that network values/weights/biases
/// will use (f32, isize, etc...)
#[derive(Debug, Clone)]
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
T: num::Zero,
{

    /// Get a new randomly populated network
    pub fn new(layer_sizes : &Vec<usize>, min_node_val : T, max_node_val : T) -> NeuralNet<T> {
        let mut weights : Vec<Matrix<T>> = Vec::new();
        let mut biases : Vec<Matrix<T>> = Vec::new();

        // Populate activation values for first (input) layer. `biases` does not need this
        // because there are no weights for the input layer
        let mut activations : Vec<Matrix<T>> = vec![Matrix::new(layer_sizes[0], 1)];

        for layer_num in 0..(layer_sizes.len() - 1) {
            let cur_layer_size = layer_sizes[layer_num];
            let next_layer_size = layer_sizes[layer_num+1];

            weights.push(Matrix::random(next_layer_size, cur_layer_size, min_node_val, max_node_val));
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
    pub fn evaluate_network(&mut self) -> Result<usize, NeuralNetErrors> {

        // calculate the activation for each layer
        for layer_num in 0..(self.num_layers - 1) {
            self.activations[layer_num+1] = self.weights[layer_num].mult(&self.activations[layer_num]); 
            self.activations[layer_num+1] = self.activations[layer_num+1].add(&self.biases[layer_num]);

            // Use activation function to update values
            for i in 0..self.activations[layer_num+1].get_nrows() {
                let cur_val = self.activations[layer_num+1].get(i, 0);
                self.activations[layer_num+1].set(i, 0, NeuralNet::relu(cur_val));
            }
        }

        // Get values of output neurons and return the one that has the highest activation val
        let output_layer = &self.activations[self.num_layers - 1];
        let mut max_act: T = T::zero();
        let mut max_act_node: Result<usize, NeuralNetErrors> = Err(NeuralNetErrors::OUTPUT_ACTIVATION_ERROR);
        for i in 0..output_layer.get_nrows() {
            let act = output_layer.get(i, 0);
            if  act >= max_act {
                max_act = act;
                max_act_node = Ok(i);
            }
        }

        return max_act_node;
    }

    /// Set value of specified input node
    pub fn set_input_node(&mut self, input_node_idx : usize, val : T) {
        self.activations[0].set(input_node_idx, 0, val);
    }


    /// Apply random mutation to every weight/bias with probability of mutation `mutation_prob` 
    /// to a value between val_min and val_max
    pub fn apply_rand_mutations(&mut self, mutation_prob : f32, val_min : T, val_max : T) {
        let mut rng = rand::thread_rng();

        // apply mutations to biases in each layer
        for layer in 0..(self.num_layers - 1) {
            // Apply mutations to biases
            let num_nodes = self.biases[layer].get_nrows();
            let num_nodes_next = self.weights[layer].get_ncols();
            for i in 0..num_nodes {

                // Mutate biases 
                // If random number [0,1) is less than mutation_prob, entry should be mutated!
                if rng.gen::<f32>() <= mutation_prob {
                    // Set to new random value
                    self.biases[layer].set(i, 0, rng.gen_range(val_min..=val_max));
                }

                // Mutate weights
                for j in 0..num_nodes_next {
                    if rng.gen::<f32>() <= mutation_prob {
                        // Set to new random value
                        self.weights[layer].set(i, j, rng.gen_range(val_min..=val_max));
                    }
                }
            }
        }
    }

    /// Perform RELU activation function on number.
    /// Static method that can be called on anything
    fn relu(num : T) -> T {
        if num >= T::zero() {
            return num;
        } else {
            return T::zero();
        }
    }

}



#[cfg(test)]
mod neuralnet_test {
    use super::*;

    #[test]
    fn test_neuralnet_init() {
        let layer_sizes = vec![5, 4, 4, 4, 8];
        let nn = NeuralNet::<isize>::new(&layer_sizes, -1000, 1000);

        // Check that couple weights are within bounds
        assert!(nn.weights[0].get(0,0) < 1000 && nn.weights[0].get(0,0) > -1000);
        assert!(nn.weights[1].get(0,0) < 1000 && nn.weights[1].get(0,0) > -1000);
        assert!(nn.weights[1].get(1,0) < 1000 && nn.weights[1].get(1,0) > -1000);
    }

    #[test]
    fn test_neuralnet_eval() {
        let layer_sizes = vec![6, 10, 10, 8];
        let mut nn = NeuralNet::<f32>::new(&layer_sizes, -1e6, 1e6);

        // Set inputs
        let input_neuron_vals = [1.2, 3.4, 1000.3, 6.2, -20000.0, -0.223];
        for (i, val) in input_neuron_vals.iter().enumerate() {
            nn.set_input_node(i, *val);
        }

        // Eval network
        let res = nn.evaluate_network().unwrap();
        assert!(res < layer_sizes[layer_sizes.len()-1]);
        println!("Output layer activated {}", res);

    }
}
