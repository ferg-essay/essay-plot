use essay_tensor::Tensor;

use crate::{artist::{Image, ImageOpt}, graph::Graph};

pub fn image(
    graph: &mut Graph, 
    data: impl Into<Tensor>,
) -> ImageOpt {
    let image = Image::new(data);
    
    graph.artist(image)
}
