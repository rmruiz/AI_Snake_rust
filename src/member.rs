use ndarray::Array2;
use rand_distr::{Distribution, Normal};
use rand::{Rng, SeedableRng, rngs::StdRng, rng};
use std::cmp::Ordering;

use crate::nn_architecture::{NN_Architecture, Activation}; 
use crate::snakegame::{Direction, Snakegame};

use serde::Serialize;

// Define the struct
#[derive(Debug, Clone, Serialize)]
pub struct Member {
    pub fitness: f64,
    pub nn_architecture: NN_Architecture,
    pub weights: Vec<Array2<f64>>,
    pub biases: Vec<Array2<f64>>,
    pub generation: usize, 
    pub killed_by_wall: usize,
    pub killed_by_myself: usize,
    pub killed_by_hunger: usize,
    pub apples_eaten: usize
}

/// Implement methods
impl Member {
    pub fn new(
        weights: Option<Vec<Array2<f64>>>,
        biases: Option<Vec<Array2<f64>>>,
        seed: Option<[u8; 32]>,
        generation: usize,
    ) -> Self {
        let mut rng: StdRng = match seed {
                Some(s) => StdRng::from_seed(s),
                None => {
                    let mut entropy = rng();
                    let random_seed= entropy.random();
                    StdRng::from_seed(random_seed)
                }
            };

        let normal: Normal<f64> = rand_distr::Normal::new(0.0, 1.0).unwrap();
        let nn_architecture: NN_Architecture = NN_Architecture::new();
        // Use provided weights or generate new ones
        let weights = weights.unwrap_or_else(|| {
            nn_architecture
                .layers
                .iter()
                .map(|layer| {
                    Array2::from_shape_fn(
                        (layer.output_dim, layer.input_dim),
                        |_| normal.sample(&mut rng),
                    )
                })
                .collect()
        });

        // Same for biases
        let biases = biases.unwrap_or_else(|| {
            nn_architecture
                .layers
                .iter()
                .map(|layer| {
                    Array2::from_shape_fn((layer.output_dim, 1), |_| normal.sample(&mut rng))
                })
                .collect()
        });

        Self {
            fitness: 0.0,
            nn_architecture,
            weights,
            biases,
            killed_by_hunger: 0,
            killed_by_myself: 0,
            killed_by_wall: 0,
            apples_eaten: 0,
            generation: generation
        }
    }
    
    fn feedforward(&self, mut a: Array2<f64>) -> Array2<f64> {
        for (idx, layer) in self.nn_architecture.layers.iter().enumerate() {
            let w: &Array2<f64> = &self.weights[idx];
            let b: &Array2<f64> = &self.biases[idx];
            a = single_layer_forward_propagation(&a, w, b, layer.activation.clone());
        }
        a
    }

    pub fn play_game_to_update_fitness(&mut self) -> usize {
        let mut sg = Snakegame::new();

        while sg.alive {
            //sg.print_board();
            //println!();
            let input: Array2<f64> = sg.get_current_input(); 
            let next_move: usize = self.next_move_from_input(input);
            sg.move_snake(Direction::from_usize(next_move));
        }

        if sg.killed_by_hunger {
            self.killed_by_hunger += 1;
        }
        else if sg.killed_by_myself {
            self.killed_by_myself += 1;
        }
        else if sg.killed_by_wall {
            self.killed_by_wall += 1;
        }
        self.apples_eaten += sg.apples_eaten; 

        sg.get_score()
    }

    pub fn iterate_to_update_fitness(&mut self, iterations: usize) {

        self.killed_by_hunger = 0;
        self.killed_by_myself = 0;
        self.killed_by_wall = 0;
        self.apples_eaten = 0;
        self.fitness = 0.0;
        let mut max_score = 0;
        
        let mut sum: usize = 0;

        for _ in 0..iterations {
            let score = self.play_game_to_update_fitness(); 
            sum += score;
            if score > max_score {
                max_score = score;
            }     
        }
        self.fitness = sum as f64 / iterations as f64;
        //printing stats per member
        //println!("MyGen {}: KxH={}, KxM={}, KxW={}, AE={}, Fit={:.3}", 
        //    self.generation, self.killed_by_hunger, self.killed_by_myself, self.killed_by_wall, self.apples_eaten, self.fitness)
        //print!("{}.",max_score)
    }

    fn next_move_from_input(&self, input: Array2<f64>) -> usize {
        let output: Array2<f64> = self.feedforward(input);
        output
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

}

/// Forward propagation through the neural network
fn single_layer_forward_propagation(
    a: &Array2<f64>,
    w: &Array2<f64>,
    b: &Array2<f64>,
    activation: Activation,
) -> Array2<f64> {
    let z: Array2<f64> = w.dot(a) + b;
    match activation {
        Activation::Relu => relu(&z),
        Activation::Sigmoid => sigmoid(&z),
    }
}

/// ReLU activation on a 2D array
fn relu(z: &Array2<f64>) -> Array2<f64> {
    z.mapv(|x| x.max(0.0))
}

/// Sigmoid activation on a 2D array
fn sigmoid(z: &Array2<f64>) -> Array2<f64> {
    z.mapv(|x| 1.0 / (1.0 + (-x).exp()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use crate::nn_architecture::LayerConfig;
    
    #[test]
    fn test_sigmoid_values() {
        let input = array![[0.0, 1.0], [-1.0, 2.0]];
        let expected = array![[0.5, 0.7310585786300049], [0.2689414213699951, 0.8807970779778823]];
        let result = sigmoid(&input);
        assert_eq!(result, expected, "Sigmoid output does not match expected values");
    }

    #[test]
    fn test_relu_values() {
        let input = array![[-1.0, 0.0, 1.0], [2.5, -3.2, 0.3]];
        let expected = array![[0.0, 0.0, 1.0], [2.5, 0.0, 0.3]];
        let result = relu(&input);
        assert_eq!(result, expected, "ReLU output does not match expected values");
    }

    #[test]
    fn test_single_layer_forward_propagation_relu_positive() {
        let a = array![[1.0], [2.0]]; // shape (2, 1)
        let w = array![[-5.0, 4.0]];  // shape (1, 2)
        let b = array![[0.5]];        // shape (1, 1)
        // z = w.dot(a) + b = [-5.0*1.0 + 4.0*2.0 + 0.5] = [-5.0 + 8.0 + 0.5] = [3.5]
        let expected = array![[3.5]];
        let result = single_layer_forward_propagation(&a, &w, &b, Activation::Relu);
        assert_eq!(result, expected, "ReLU forward propagation failed");
    }

    #[test]
    fn test_single_layer_forward_propagation_relu_negative() {
        let a = array![[1.0], [2.0]]; // shape (2, 1)
        let w = array![[5.0, -4.0]];  // shape (1, 2)
        let b = array![[0.5]];        // shape (1, 1)
        // z = w.dot(a) + b = [5.0*1.0 + (-4.0)*2.0 + 0.5] = [5.0 - 8.0 + 0.5] = [-2.5]
        let expected = array![[0.0]];
        let result = single_layer_forward_propagation(&a, &w, &b, Activation::Relu);
        assert_eq!(result, expected, "ReLU forward propagation failed");
    }

    #[test]
    fn test_single_layer_forward_propagation_sigmoid() {
        let a = array![[1.0], [2.0]]; // shape (2, 1)
        let w = array![[0.5, -0.3]];  // shape (1, 2)
        let b = array![[0.1]];        // shape (1, 1)
        // z = [0.0] => sigmoid(0.0) = 0.5
        let expected = array![[0.5]];
        let result = single_layer_forward_propagation(&a, &w, &b, Activation::Sigmoid);
        assert_eq!(result, expected, "Sigmoid forward propagation failed");
    }

    #[test]
    fn test_feedforward_with_known_weights_and_biases() {
        // Build a minimal architecture manually
        let mut nn_arch = NN_Architecture::new();
        nn_arch.layers = vec![
            LayerConfig {
                input_dim: 2,
                output_dim: 1,
                activation: Activation::Relu,
            }
        ];

        // Set weights and biases to deterministic values
        // Input A: shape (2, 1)
        // W: shape (1, 2)
        // B: shape (1, 1)
        // z = W·A + b = [1.0 * 2.0 + (-1.0) * 3.0 + 0.5] = [-0.5] → relu = 0.0
        let weights: Vec<Array2<f64>> = vec![ array![[1.0, -1.0]] ]; // shape (1, 2)
        let biases: Vec<Array2<f64>> = vec![ array![[0.5]] ]; // shape (1, 1)
        let member: Member = Member::new(Some(weights), Some(biases), None, 0);
        // Force our test architecture into the struct
        let mut member = member;
        member.nn_architecture = nn_arch;

        let input: Array2<f64> = array![[2.0], [3.0]]; // shape (2, 1)
        let output: Array2<f64> = member.feedforward(input);

        let expected: Array2<f64> = array![[0.0]]; // relu(-0.5) = 0.0

        assert_eq!(output, expected, "Feedforward output is incorrect");
    }

    #[test]
    fn test_next_move_from_input() {
        // Define a simple architecture with one layer
        let architecture = NN_Architecture {
            layers: vec![LayerConfig {
                input_dim: 2,
                output_dim: 3,
                activation: Activation::Relu, // assuming you have a no-op activation
            }],
        };

        // Manually set weights and biases for predictable output
        let weights: Vec<Array2<f64>> = vec![array![[1.0, 2.0], [0.0, 0.0], [-1.0, -1.0]]];
        let biases: Vec<Array2<f64>> = vec![array![[0.0], [0.0], [0.0]]];

        // Create a Member with known weights and biases
        let member = Member {
            fitness: 0.0,
            nn_architecture: architecture,
            weights,
            biases,
            killed_by_hunger: 0,
            killed_by_myself: 0,
            killed_by_wall: 0,
            apples_eaten: 0,
            generation: 0
        };

        // Input vector: shape (2, 1)
        let input: Array2<f64> = array![[1.0], [1.0]];

        // Feedforward result:
        // Row 0: 1*1 + 2*1 = 3
        // Row 1: 0
        // Row 2: -1 -1 = -2
        // So max is at index 0
        let result = member.next_move_from_input(input);
        assert_eq!(result, 0);
    }

}
