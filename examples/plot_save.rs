use std::f32::consts::TAU;

use essay_plot::{prelude::*, artist::{patch::PathPatch, Markers, IntoMarker, Shading}, plot::{bar, grid_color, contour, triplot, tricontour, matshow, stem, fill_between, plot}, graph::{Graph, PlotOpt}};
use essay_plot_api::{Point, Color, PathCode, Path, JoinStyle, CapStyle, LineStyle, Angle};
use essay_tensor::{prelude::*, init::{linspace, meshgrid, meshgrid_ij}, tensor::TensorVec};

fn main() {
    main_plot();
}

fn main_plot() {
    let mut figure = Figure::new();
    let mut graph = figure.new_graph([1., 1.]);

    let x = linspace(0., TAU, 20);
    let y = x.sin();

    graph.title("My Title");

    graph.plot(&x, &y);

    figure.save(640, 480, "../test.png");
}

fn main_grid() { 
    let x = linspace(0., 2. * 6.28, 51);
    let y = linspace(0., 6.28, 101);
    let [x, y] = meshgrid([x, y]);

    let z = &x.sin() + &y.sin();

    let mut figure = Figure::new();
    let mut graph1 = figure.new_graph(());
    let mut graph2 = figure.new_graph(());

    grid_color(&mut graph1, &z).shading(Shading::Flat);
    grid_color(&mut graph2, &z).shading(Shading::Gouraud);

    figure.save(640, 480, "../test.png");
}
