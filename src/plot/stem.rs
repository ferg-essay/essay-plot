use essay_tensor::tensor::Tensor;

use crate::{artist::{Stem, StemOpt}, chart::Chart};

pub fn stem(
    graph: &mut Chart, 
    x: impl Into<Tensor>, 
    y: impl Into<Tensor>, 
) -> StemOpt {
    let stem = Stem::from_xy(x, y);

    //self.artist(lines)
    graph.artist(stem)
}

