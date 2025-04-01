use std::f32::consts::PI;

use essay_plot::{chart::{Chart, SubFigure}, palette::{Category, Palette}, prelude::*};
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();

    figure.multichart(|ui| {
        ui.horizontal(|ui| {
            plot_pair(ui, Category::Tableau, "Tableau");
        });
        ui.horizontal(|ui| {
            plot_pair(ui, Category::CategoryC, "CategoryC");
        });
    });

    figure.show();
}

fn plot_pair(
    ui: &mut SubFigure, 
    palette: impl Into<Palette>,
    title: &str,
) {
    let palette = palette.into();

    let mut chart = ui.chart();
    chart.color_cycle(palette.clone()).title(title);
    chart.x().visible(false);
    chart.y().visible(false);
    plot(&mut chart, 5);

    let mut chart = ui.chart();
    chart.color_cycle(palette.clone()).title(title);
    plot(&mut chart, 20);
    chart.x().visible(false);
    chart.y().visible(false);
}

fn plot(chart: &mut Chart, n: usize) {
    let x = linspace(0., 6.28, 20);

    for i in 0..n {
        chart.plot(&x, (&x + i as f32 * PI / n as f32).cos());
    }
}
