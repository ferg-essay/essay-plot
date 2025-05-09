use essay_plot::prelude::*;
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();

    let mut chart = figure.chart();

    chart.title("My Title").color("dark teal").size(18.);

    let x = linspace(0., 6.28, 20);

    chart.plot(&x, &x.sin()).label("A");
    chart.plot(&x, &x.cos()).label("B");

    figure.show();
}
