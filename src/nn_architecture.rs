use serde::Serialize;

const INPUT_SIZE: usize = 7;
const NEURONS_PER_LAYER_1: usize = 32;
const NEURONS_PER_LAYER_2: usize = 64;
const OUTPUT_SIZE: usize = 3;

/// Enum representing activation functions
#[derive(Debug, Clone, Serialize)]
pub enum Activation {
    Relu,
    Sigmoid,
}

/// Struct for a layer configuration
#[derive(Debug, Clone, Serialize)]
pub struct LayerConfig {
    pub input_dim: usize,
    pub output_dim: usize,
    pub activation: Activation,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Serialize)]
pub struct NN_Architecture {
    pub layers: Vec<LayerConfig>,
}

impl NN_Architecture {
    pub fn new() -> Self {
        let nn_arch: Vec<LayerConfig> = vec![
            LayerConfig {
                input_dim: INPUT_SIZE,
                output_dim: NEURONS_PER_LAYER_1,
                activation: Activation::Relu,
            },

            LayerConfig {
                input_dim: NEURONS_PER_LAYER_1,
                output_dim: NEURONS_PER_LAYER_2,
                activation: Activation::Relu,
            },
            LayerConfig {
                input_dim: NEURONS_PER_LAYER_2,
                output_dim: OUTPUT_SIZE,
                activation: Activation::Sigmoid,
            },
        ];
        NN_Architecture { layers: nn_arch }
    }
}

