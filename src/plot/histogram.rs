use essay_tensor::Tensor;

use crate::{chart::Chart, artist::{Histogram, HistogramOpt}};

pub fn hist(
    graph: &mut Chart,
    data: impl Into<Tensor>,
) -> HistogramOpt {
    let histogram = Histogram::new(data);
    
    graph.artist(histogram)
}

impl Chart {
    pub fn hist(&mut self, data: impl Into<Tensor>) -> HistogramOpt {
        hist(self, data)
    }
}