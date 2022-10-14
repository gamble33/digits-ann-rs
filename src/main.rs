mod neural_net;
mod layer;

use neural_net::NeuralNet;

fn main() {
  let nn = NeuralNet::new(&[28 * 28, 16, 16, 10]);
}
