use essay_plot::{
    artist::Shading, 
    palette::{Colorcet, Diverging, EssayColors}, 
    plot::grid_color, 
    prelude::*
};
use essay_tensor::init::{linspace, meshgrid};

fn main() { 
    let x = linspace(0., 2. * 6.28, 21);
    let y = linspace(0., 6.28,21);
    let [x, y] = meshgrid([x, y]);

    let z = &x.sin() + &y.sin();

    let mut figure = Figure::new();
    let mut graph1 = figure.chart();

    grid_color(&mut graph1, &z)
        .shading(Shading::Gouraud)
        .color_map(Diverging::RedBlue);
    
    figure.show();
}
