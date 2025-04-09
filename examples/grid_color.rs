use essay_plot::{
    artist::Shading, 
    palette::Diverging, 
    plot::grid_color, 
    prelude::*
};
use essay_tensor::init::{linspace, meshgrid};

fn main() { 
    let x = linspace(0., 2. * 6.28, 51);
    let y = linspace(0., 6.28, 101);
    let [x, y] = meshgrid([x, y]);

    let z = &x.sin() + &y.sin();

    let mut figure = Figure::new();
    let mut graph1 = figure.chart();

    //graph1.colorbar();
    //let mut graph2 = figure.chart();

    grid_color(&mut graph1, &z)
        .shading(Shading::Flat)
        .color_map(Diverging::RedYellowBlue);
    //grid_color(&mut graph2, &z).shading(Shading::Gouraud);
    
    figure.show();
}
