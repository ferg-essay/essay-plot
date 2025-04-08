use essay_tensor::tensor::Tensor;

use crate::{chart::Chart, artist::{GridColor, GridColorOpt}};

pub fn grid_color(
    graph: &mut Chart,
    data: impl Into<Tensor>,
) -> GridColorOpt {
    let colormesh = GridColor::new(data);
    
    graph.artist(colormesh)
}

impl Chart {
    pub fn grid_color(&mut self, data: impl Into<Tensor>) -> GridColorOpt {
        grid_color(self, data)
    }
}