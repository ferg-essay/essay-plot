use essay_tensor::Tensor;

use crate::{chart::Chart, artist::{Quiver, QuiverOpt}};

pub fn quiver(
    graph: &mut Chart,
    x: impl Into<Tensor>,
    y: impl Into<Tensor>,
    u: impl Into<Tensor>,
    v: impl Into<Tensor>,
) -> QuiverOpt {
    let quiver = Quiver::new(x, y, u, v);
    
    graph.artist(quiver)
}

impl Chart {
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