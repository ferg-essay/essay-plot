use essay_plot::{prelude::*, plot::grid_color, artist::Shading};
use essay_tensor::init::{linspace, meshgrid};

fn main() { 
    let x = linspace(0., 2. * 6.28, 51);
    let y = linspace(0., 6.28, 101);
    let [x, y] = meshgrid([x, y]);

    let z = &x.sin() + &y.sin();

    let mut figure = Figure::new();
    let mut graph1 = figure.new_graph(());
    let mut graph2 = figure.new_graph(());

    grid_color(&mut graph1, &z).shading(Shading::Flat);
    grid_color(&mut graph2, &z).shading(Shading::Gouraud);

    figure.show();
}
