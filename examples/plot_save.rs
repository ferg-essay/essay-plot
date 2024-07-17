use std::f32::consts::TAU;

use essay_plot::{prelude::*, artist::{patch::PathPatch, Markers, IntoMarker, Shading, DrawStyle}, plot::{bar, grid_color, contour, triplot, tricontour, matshow, stem, fill_between, plot}, graph::{Graph, PlotOpt}};
use essay_plot::api::{Point, Color, PathCode, Path, JoinStyle, CapStyle, LineStyle, Angle};
use essay_tensor::{prelude::*, init::{linspace, meshgrid, meshgrid_ij}, tensor::TensorVec};

fn main() {
//    main_plot();
    main_plot2();
}

fn main_plot() {
    let mut figure = Figure::new();
    let mut graph = figure.new_graph([1., 1.]);

    let x = linspace(0., TAU, 20);
    let y = x.sin();

    graph.title("My Title");

    graph.plot(&x, &y);

    figure.save("../test.png", 144.);
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

    figure.save("../test.png", 144.);
}

fn main_plot2() {
    let mut figure = Figure::new();
    let mut graph1 = figure.new_graph([0., 0., 0.9, 0.9]);
    let mut graph2 = figure.new_graph([1.1, 0., 2., 0.9]);

    let mut graph3 = figure.new_graph([0., 1.1, 0.9, 2.]);
    let mut graph4 = figure.new_graph([1.1, 1.1, 2., 2.]);

    let x = linspace(0., 1., 10);
    let odor = tf32!([
        0., 1., 1., 1., 1., 1., 1., 1., 1., 0.,
    ]) * 0.9;
    let failure = tf32!([
        1., 1., 1., 1., 1., 1., 1., 1., 0., 0.,
    ]);

    graph1.title("Signal");
    graph1.ylim(0., 2.0);
    graph1.plot(&x, &odor).label("odor");
    graph1.plot(&x, &failure).label("LTD").draw_style(DrawStyle::StepsMid);
    graph1.x().visible(false);
    graph1.y().visible(false);

    graph2.title("Timeout");
    graph2.ylim(0., 2.0);
    graph2.plot(&x, &odor).label("odor");
    let x2 = tf32!([0., 0.1, 0.8, 1.0]);
    let failure2 = tf32!([
        1., 1., 0., 0.,
    ]);
    graph2.plot(&x2, &failure2).label("LTD");
    graph2.x().visible(false);
    graph2.y().visible(false);

    graph3.title("Signal");
    graph3.plot(&x, &odor).label("odor");
    let failure = tf32!([
        1., 1., 1., 1.,
    ]);
    graph3.plot(&x2, &failure).label("LTD").draw_style(DrawStyle::StepsMid);

    let x3 = tf32!([
        0., 0.6, 0.7, 1.,
    ]);
    let reward = tf32!([
        0., 0.8, 0., 0.,
    ]);
    graph3.plot(&x3, &reward).label("reward").draw_style(DrawStyle::StepsMid);
    graph3.ylim(0., 2.0);
    graph3.x().visible(false);
    graph3.y().visible(false);

    graph3.title("Signal");

    graph4.title("Timeout");
    graph4.plot(&x, &odor).label("odor");
    let failure = tf32!([
        1., 1., 1., 1.,
    ]);
    graph4.plot(&x2, &failure2).label("LTD");

    let x3 = tf32!([
        0., 0.6, 0.7, 1.,
    ]);
    let reward = tf32!([
        0., 0.8, 0., 0.,
    ]);
    graph4.plot(&x3, &reward).label("reward").draw_style(DrawStyle::StepsMid);
    graph4.ylim(0., 2.0);
    graph4.x().visible(false);
    graph4.y().visible(false);
    //graph3.title("Reward");
    //graph4.title("Reward");

    figure.save("../test.png", 144.);
    figure.show();
}
