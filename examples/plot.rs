use essay_plot::{chart::ShowGrid, prelude::*};
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    chart.title("My Title").color("dark teal").size(18.);

    let x = linspace(0., 6.28, 20);

    chart.plot(&x, &x.sin());
    chart.plot(&x, &x.cos());

    // chart.x().major_grid().color("purple");
    chart.x().show_grid(ShowGrid::Major);

    figure.show();
}
