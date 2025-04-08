use essay_tensor::tensor::Tensor;

use crate::{chart::Chart, artist::{Bar, BarOpt}};

pub fn bar(
    graph: &mut Chart,
    data: impl Into<Tensor>,
) -> BarOpt {
    let bar = Bar::new(data);
    
    graph.artist(bar)
}

impl Chart {
    pub fn bar(&mut self, data: impl Into<Tensor>) -> BarOpt {
        bar(self, data)
    }
}