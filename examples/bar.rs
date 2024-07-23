use essay_plot::prelude::*;
use essay_tensor::prelude::*;

fn main() { 

    let mut figure = Figure::new();
    let mut graph = figure.chart(());

    let bottom = Tensor::zeros([3]);
    graph.bar([1., 2., 3.]).bottom(&bottom);

    let bottom = bottom + tf32!([1., 2., 3.]);
    graph.bar([2., 1., 2.]).bottom(&bottom);

    let bottom = bottom + tf32!([2., 1., 2.]);
    graph.bar([1., 1., 0.]).bottom(&bottom);

    //graph.bar(data).x([1., 3., 5., 7.]);
    //graph.bar(tf32!([2., 0.5, 13., 8.]));
    //graph.bar(tf32!([0.5, 0.25, 3., 5.])).width(tf32!([0.4, 0.6, 0.2, 0.8]));

    figure.show();
}
