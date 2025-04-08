use essay_plot::prelude::*;
use essay_tensor::tensor::Tensor;

fn main() { 
    let data = Tensor::random_normal([128], None);

    let mut figure = Figure::new();
    let mut graph = figure.chart();

    graph.hist(data);

    figure.show();
}
