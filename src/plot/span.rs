use crate::{artist::{HorizontalLine, HorizontalLineOpt}, chart::Chart};

pub fn hline(
    graph: &mut Chart, 
    y: f32,
) -> HorizontalLineOpt {
    let hline = HorizontalLine::new(0., 1., y);
    
    graph.artist(hline)
}
