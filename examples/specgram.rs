use essay_plot::prelude::*;
use essay_tensor::init::linspace;

fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();
    let mut graph = figure.chart([1., 1.]);

    //let x = linspace(0., 2. * PI, 30);
    //let y = x.sin();

    let len = 256 * 4;
    let x = linspace(0., len as f32, len);
    let y = x.sin();

    graph.title("My Title"); // .color(0x008033).size(18.);
    //graph.x_label("My X-Label"); // .color("brown");
    //graph.y_label("Y-Label"); // .color("teal").size(8.);
    graph.specgram(&y).nfft(256).overlap(128);

    figure.show();
}
