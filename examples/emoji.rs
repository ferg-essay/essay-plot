use essay_plot::prelude::*;
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();
    let mut graph = figure.graph([1., 1.]);

    let x = linspace(0., 6.28, 20);
    let y = x.sin();

    graph.title("My Title"); // .color(0x008033).size(18.);
    graph.plot(&x, &y);

    // graph.text((0., 1.), "hello").coord(TextCoords::FrameFraction).color("purple");
    //let family = "/Users/ferg/wsp/essay-mind/assets/font/NotoEmoji-Regular.ttf";
    let family = "/Users/ferg/wsp/essay-mind/assets/font/NotoEmoji-Bold.ttf";
    graph.text((0.5, 0.5), "\u{1f980}\u{1f990}").family(family).color("red");

    figure.show();
}
