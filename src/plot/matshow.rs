use essay_tensor::Tensor;

use crate::{graph::Graph, artist::Image};

pub fn matshow(
    graph: &mut Graph, 
    data: impl Into<Tensor>,
) {
    let matplot = Image::new(data);
    
    graph.add_simple_artist(matplot);
    graph.aspect(1.);
    graph.flip_y(true);
}
