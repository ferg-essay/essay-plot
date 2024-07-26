use essay_plot::{
    prelude::*, artist::patch::PathPatch, 
    chart::{Chart, PlotOpt}
};
use essay_plot::api::{Point, PathCode, Path};
use essay_tensor::init::linspace;

fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();

    for j in 0..3 {
        for i in 0..3 {
            let mut chart = figure.chart(((i as f32, j as f32), [1., 1.]));

            let t = linspace(0., 6.28, 40);
            let x = ((i as f32 + 1.) * &t).sin();
            let y = ((j as f32 + 1.) * t).cos();

            chart.title("My Title"); // .color(0x008033).size(18.);
            chart.plot(&x, &y);
        }
    }

    let mut chart = figure.chart(((3., 0.), [1., 3.]));

    let t = linspace(0., 6.28, 40);
    let x = t.sin();
    let y = (4. * t).cos();

    chart.title("My Title"); // .color(0x008033).size(18.);
    chart.plot(&x, &y);

    figure.show();
}

pub fn plot_quad(
    graph: &mut Chart, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::LineTo(p1.into()),
        PathCode::LineTo(p2.into()),
        PathCode::ClosePoly(p3.into()),
    ])))
}

pub fn plot_line(
    graph: &mut Chart, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>,
    p4: impl Into<Point>,
    p5: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::LineTo(p1.into()),
        PathCode::MoveTo(p2.into()),
        PathCode::LineTo(p3.into()),
        PathCode::MoveTo(p4.into()),
        PathCode::LineTo(p5.into()),
    ])))
}
