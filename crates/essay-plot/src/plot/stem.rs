use essay_tensor::Tensor;

use crate::{artist::{Stem, StemOpt}, graph::{Graph}};

pub fn stem(
    graph: &mut Graph, 
    x: impl Into<Tensor>, 
    y: impl Into<Tensor>, 
) -> StemOpt {
    let stem = Stem::from_xy(x, y);

    //self.artist(lines)
    graph.add_plot_artist(stem)
}

