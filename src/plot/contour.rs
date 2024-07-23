use essay_tensor::Tensor;

use crate::{chart::Chart, artist::{Contour}};

pub fn contour(
    graph: &mut Chart, 
    data: impl Into<Tensor>,
) {
    let contour = Contour::new(data);
    
    graph.add_simple_artist(contour);
}
