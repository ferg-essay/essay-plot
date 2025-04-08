use essay_plot::prelude::*;
use essay_tensor::{init::linspace, ten, tensor::Tensor};

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.polar();

    let x = linspace(0., 6.28, 40);
    let one = Tensor::ones(x.shape());

    chart.plot(&x, &one);
    chart.plot(&x, (2. * &x).sin());
    chart.plot(&x, 2. * &x.cos());
    //chart.plot(&x, &x.clone());
    //chart.fill(&x, &x.clone()).alpha(0.2);
    //chart.plot(&x, &x.cos());

    figure.show();
}
