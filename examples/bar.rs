use essay_plot::prelude::*;
use essay_tensor::prelude::*;

fn main() { 

    let mut figure = Figure::new();
    let mut chart = figure.chart();

    let bottom = Tensor::zeros([3]);
    chart.bar([1., 2., 3.]).bottom(&bottom);

    let bottom = bottom + tf32!([1., 2., 3.]);
    chart.bar([2., 1., 2.]).bottom(&bottom);

    let bottom = bottom + tf32!([2., 1., 2.]);
    chart.bar([1., 1., 0.]).bottom(&bottom);

    figure.show();
}
