use essay_tensor::Tensor;

use crate::{graph::Graph, artist::{GridColor, GridColorOpt}};

pub fn grid_color(
    graph: &mut Graph,
    data: impl Into<Tensor>,
) -> GridColorOpt {
    let colormesh = GridColor::new(data);
    
    graph.artist(colormesh)
}

impl Graph {
    pub fn grid_color(&mut self, data: impl Into<Tensor>) -> GridColorOpt {
        grid_color(self, data)
    }
}