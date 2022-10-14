use Vec;
use crate::layer::Layer;

pub struct NeuralNet {
  pub layers: Vec<Layer>,
}

impl NeuralNet {
  pub fn new(layers_neurons: &[u32]) -> Self {
    let mut nn = Self {
      layers: Vec::new()
    };
    for i in 0..layers_neurons.len() - 1 {
      nn.layers.push(Layer::new(layers_neurons[i], layers_neurons[i + 1]));
    }
    return nn;
  }

  pub fn forward() {
    !unimplemented!()
  }
}