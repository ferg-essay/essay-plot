use essay_tensor::tensor::Tensor;

use crate::{chart::Chart, artist::TriPlot};

pub fn triplot(
    graph: &mut Chart, 
    data: impl Into<Tensor>,
) {
    let triplot = TriPlot::new(data);
    
    graph.add_simple_artist(triplot);
}
