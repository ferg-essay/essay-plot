use essay_tensor::{init::linspace, Tensor};

use crate::{artist::{Lines2d, LinesOpt}, chart::Chart};

pub fn plot(
    graph: &mut Chart, 
    x: impl Into<Tensor>, 
    y: impl Into<Tensor>, 
) -> LinesOpt {
    let lines = Lines2d::from_xy(x, y);

    //self.artist(lines)
    graph.artist(lines)
}

pub fn plot_y(
    graph: &mut Chart, 
    y: impl Into<Tensor>, 
) -> LinesOpt {
    let y = y.into();
    let x = linspace(0., y.len() as f32, y.len());
    
    let lines = Lines2d::from_xy(x, y);

    //self.artist(lines)
    graph.artist(lines)
}
