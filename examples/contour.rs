use essay_plot::{plot, prelude::*};
use essay_tensor::init::{arange, meshgrid};

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    chart.title("Contour");

    let delta = 0.025;
    let x = arange(-3., 3., delta);
    let y = arange(-2., 2., delta);
    let [x, y] = meshgrid([x, y]);

    let z1 = (-x.powi(2) - y.powi(2)).exp();
    let z2 = (-(x.clone() - 1.).powi(2) - (y.clone() - 1.).powi(2)).exp();
    let z = (z1 - z2) * 2.;

    plot::contour(&mut chart, z);

    figure.show();
}
