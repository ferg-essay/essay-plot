use essay_plot::prelude::*;
use essay_tensor::tensor::Tensor;

fn main() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    chart.title("Box Plot");

    let x = Tensor::random_normal([21], None);
    let y = Tensor::random_normal([21], None);

    chart.box_plot([&x, &y]);
    chart.x().tick_labels(&[(1., "a"), (2., "b")]);

    figure.show();
}
