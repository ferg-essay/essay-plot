use essay_tensor::tensor::Tensor;

use crate::{artist::{Image, ImageOpt}, chart::Chart};

pub fn image(
    graph: &mut Chart, 
    data: impl Into<Tensor>,
) -> ImageOpt {
    let image = Image::new(data);
    
    graph.artist(image)
}
