use essay_plot::prelude::*;
use essay_tensor::{prelude::*, init::{arange, meshgrid, linspace}};

fn main() { 

    let mut figure = Figure::new();
    let mut graph = figure.new_graph(());

    let x = arange(0., 6.28, 0.2);
    let y = arange(0., 6.28, 0.2);
    let [u, v] = meshgrid([&x, &y]);

    graph.quiver(x, y, u.sin(), v.cos());

    //graph.bar(data).x([1., 3., 5., 7.]);
    //graph.bar(tf32!([2., 0.5, 13., 8.]));
    //graph.bar(tf32!([0.5, 0.25, 3., 5.])).width(tf32!([0.4, 0.6, 0.2, 0.8]));

    figure.show();
}
