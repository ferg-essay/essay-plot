use essay_graphics::layout::MainLoop;
use essay_plot::{chart::{Chart, ConfigArc}, prelude::*};
use essay_tensor::prelude::*;

fn main() { 
    let config = ConfigArc::default();
    let mut chart = Chart::new(&config);

    let bottom = Tensor::zeros([3]);
    chart.bar([1., 2., 3.]).bottom(&bottom);

    let bottom = bottom + tf32!([1., 2., 3.]);
    chart.bar([2., 1., 2.]).bottom(&bottom);

    let bottom = bottom + tf32!([2., 1., 2.]);
    chart.bar([1., 1., 0.]).bottom(&bottom);

    MainLoop::new().show(chart.view().arc().clone());
}
