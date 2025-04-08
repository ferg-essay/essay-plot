use essay_tensor::tensor::Tensor;

use crate::{chart::Chart, artist::TriContour, tri::Triangulation};

pub fn tricontour(
    graph: &mut Chart, 
    tri: impl Into<Triangulation>,
    data: impl Into<Tensor>,
) {
    let tricontour = TriContour::new(tri, data);
    
    graph.add_simple_artist(tricontour);
}
