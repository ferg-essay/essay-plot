use essay_tensor::Tensor;

use crate::{graph::Graph, artist::{Quiver, QuiverOpt}};

pub fn quiver(
    graph: &mut Graph,
    x: impl Into<Tensor>,
    y: impl Into<Tensor>,
    u: impl Into<Tensor>,
    v: impl Into<Tensor>,
) -> QuiverOpt {
    let quiver = Quiver::new(x, y, u, v);
    
    graph.artist(quiver)
}

impl Graph {
    pub fn quiver(
        &mut self, 
        x: impl Into<Tensor>,
        y: impl Into<Tensor>,
        u: impl Into<Tensor>,
        v: impl Into<Tensor>,
    ) -> QuiverOpt {
        quiver(self, x, y, u, v)
    }
}