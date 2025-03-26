use essay_plot::prelude::*;
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    chart.title("Scatter Example");

    let x = linspace(0., 6.28, 20);

    chart.scatter(&x, &x.sin());

    figure.show();
}
