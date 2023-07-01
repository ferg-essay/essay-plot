use essay_tensor::Tensor;

use crate::{graph::Graph, artist::{ColorMesh, Image}};

pub fn matplot(
    graph: &mut Graph, 
    data: impl Into<Tensor>,
) {
    let matplot = Image::new(data);
    
    graph.add_simple_artist(matplot);
}
