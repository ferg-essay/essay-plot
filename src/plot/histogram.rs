use essay_tensor::Tensor;

use crate::{graph::Graph, artist::{Histogram, HistogramOpt}};

pub fn hist(
    graph: &mut Graph,
    data: impl Into<Tensor>,
) -> HistogramOpt {
    let histogram = Histogram::new(data);
    
    graph.artist(histogram)
}

impl Graph {
    pub fn hist(&mut self, data: impl Into<Tensor>) -> HistogramOpt {
        hist(self, data)
    }
}