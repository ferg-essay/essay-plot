use essay_plot::{prelude::*, artist::patch::{Patch, self}, chart::Data};

fn main() { 
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    let path = Path::<Data>::new(vec![
        PathCode::MoveTo(Point(1., 1.)),
        PathCode::LineTo(Point(11., 11.)),
        PathCode::LineTo(Point(6., 11.)),
        PathCode::LineTo(Point(7.7, 10.)),
        PathCode::ClosePoly(Point(0., 2.)),
    ]);
    chart.artist(Patch::new(path)).color("teal").edge_color("black");
    //let patch = patch::arrow((4., 1.), (0.707, 0.707));
    //graph.artist(patch).color("teal").edge_color("black");

    // TODO: triangulation bug
    //let patch = patch::arrow((6., 1.), (-0.707, 0.707));
    //graph.artist(patch).color("black");

    chart.aspect(1.);
    chart.xlim(0., 20.);
    chart.ylim(0., 20.);
    //graph.aspect_mode(AspectMode::View);

    figure.show();
}
