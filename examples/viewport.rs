use essay_plot::{artist::{patch, Patch}, prelude::*};

fn main() { 
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    let patch = Patch::new(Path::move_to(0., 0.).line_to(1., 0.).close_poly(1., 1.));
    chart.artist(patch).color("purple").edge_color("black").rotate(Angle::Unit(0.1)).scale(2.);

    let patch = Patch::rect([0.5, 0.5], [2., 2.]);
    chart.artist(patch).color("red").edge_color("black");

    let patch = patch::arrow([3., 1.], [0., 1.]).width(2.);
    chart.artist(patch).color("teal").edge_color("black");

    let patch = patch::arrow([4., 1.], [0.707, 0.707]);
    chart.artist(patch).color("teal").edge_color("black");

    let patch = patch::arrow([6., 1.], [0.707, 0.707])
        .tail_width(0.6)
        .head_width(1.)
        .head_length(0.8);
    chart.artist(patch).color("teal").edge_color("black");

    // TODO: triangulation bug
    let patch = patch::arrow([8., 1.], [-0.707, 0.707]);
    chart.artist(patch).color("amber").edge_color("black");

    chart.aspect(1.);
    chart.xlim(Some(0.), Some(10.));
    chart.ylim(Some(0.), Some(10.));
    //graph.aspect_mode(AspectMode::View);

    figure.show();
}
