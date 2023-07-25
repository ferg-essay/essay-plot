use essay_tensor::Tensor;

use crate::{graph::Graph, artist::{GridColor, GridColorOpt}};

pub fn grid_color(
    graph: &mut Graph,
    data: impl Into<Tensor>,
) -> GridColorOpt {
    let colormesh = GridColor::new(data);
    
    graph.add_plot_artist(colormesh)
}
