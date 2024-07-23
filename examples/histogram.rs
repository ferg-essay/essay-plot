use essay_plot::prelude::*;
use essay_tensor::Tensor;

fn main() { 
    let data = Tensor::random_normal([128], ());

    let mut figure = Figure::new();
    let mut graph = figure.chart(());

    graph.hist(data);

    figure.show();
}
