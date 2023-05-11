use crate::{Tensor};

pub fn ones(shape: &[usize]) -> Tensor {
    Tensor::fill(1., shape)
}

impl Tensor {
    pub fn ones(shape: &[usize]) -> Tensor {
        Tensor::fill(1., shape)
    }
}

