use essay_plot::prelude::*;
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    chart.title("My Title").size(18.);

    let x = linspace(0., 6.28, 20);

    chart.plot(&x, &x.sin());
    chart.plot(&x, &x.cos());
    chart.hline(0.5);

    figure.show();
}
