use std::f32::consts::PI;

use essay_plot::{chart::Chart, color::Sequential, prelude::*};
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();

    figure.multichart(|ui| {
        let mut chart = ui.chart();
        chart.color_cycle(Sequential::PuBuGn);
        plot(&mut chart, 3);

        let mut chart = ui.chart();
        chart.color_cycle(Sequential::PuBuGn);
        plot(&mut chart, 12);
    });

    figure.show();
}

fn plot(chart: &mut Chart, n: usize) {
    let x = linspace(0., 6.28, 20);

    for i in 0..n {
        chart.plot(&x, (&x + i as f32 * PI / n as f32).cos());
    }
}
