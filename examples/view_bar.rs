use essay_graphics::layout::MainLoop;
use essay_plot::chart::Chart;
use essay_tensor::{ten, tensor::Tensor};

fn main() { 
    let mut chart = Chart::default();

    let bottom = Tensor::zeros([3]);
    chart.bar([1., 2., 3.]).bottom(&bottom);

    let bottom = bottom + ten![1., 2., 3.];
    chart.bar([2., 1., 2.]).bottom(&bottom);

    let bottom = bottom + ten![2., 1., 2.];
    chart.bar([1., 1., 0.]).bottom(&bottom);

    MainLoop::new().show(chart);
}
