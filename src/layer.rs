use Vec;
use core::fmt::{Display};
use rand;

#[derive(Debug)]
pub struct Layer {
  w: Vec<Vec<f64>>,
  b: Vec<f64>,

}

impl Layer {
  pub fn new(n_in: u32, n_out: u32) -> Self {
    let mut nl = Self {
      w: Vec::new(),
      b: Vec::new()
    };
    for _ in 0..n_out {
      let mut v = Vec::new();
      for _ in 0..n_in {
        v.push(rand::random::<f64>());
      }
      nl.w.push(v);
      nl.b.push(rand::random::<f64>());
    };
    return nl;
  }

  pub fn calc_outputs(&self, inputs: &Vec<f64>) {
    outputs = Vec::new();
    for i in 0..self.w.len() {
      let output: f64;
      for j in 0..self.w[i].len() {
        output += self.w[i][j] * inputs[j] + self.b
        outputs.add()
      }
    }
  }
}