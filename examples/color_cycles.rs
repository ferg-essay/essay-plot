use std::f32::consts::PI;

use essay_plot::{chart::Chart, prelude::*};
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();

    figure.multichart(|ui| {
        let mut chart = ui.chart();

        plot(&mut chart);
    });

    figure.show();
}

fn plot(chart: &mut Chart) {
    let x = linspace(0., 6.28, 20);

    let n = 8;

    for i in 0..n {
        chart.plot(&x, (&x + i as f32 * PI / n as f32).cos());
    }
}
