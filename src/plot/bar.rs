use essay_tensor::Tensor;

use crate::{graph::Graph, artist::{Bar, BarOpt}};

pub fn bar(
    graph: &mut Graph,
    data: impl Into<Tensor>,
) -> BarOpt {
    let bar = Bar::new(data);
    
    graph.artist(bar)
}

impl Graph {
    pub fn bar(&mut self, data: impl Into<Tensor>) -> BarOpt {
        bar(self, data)
    }
}