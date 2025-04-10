use std::f32::consts::PI;

use essay_plot::{chart::ShowGrid, prelude::*};
use essay_tensor::init::linspace;

fn main() {
    let mut figure = Figure::new();

    let x = linspace(0., 6.28, 20);

    figure.multichart(|ui| {
        ui.horizontal(|ui| {
            let mut chart = ui.chart();

            chart.plot(&x, &x.sin());
            chart.plot(&x, &x.cos());
            chart.x().show_grid(ShowGrid::Major);
            chart.y().show_grid(ShowGrid::Major);
            //chart.x().show_grid(ShowGrid::Major);
            //chart.y().show_grid(ShowGrid::Major);

            let mut chart = ui.chart();

            chart.plot(&x, &x.sin());
            chart.plot(&x, &x.cos());
            chart.x().show_grid(ShowGrid::Major);
            chart.y().show_grid(ShowGrid::Major);
            chart.x().visible(false);
            chart.y().visible(false);
        });

        ui.horizontal(|ui| {
            let mut chart = ui.chart();

            chart.plot(&x, &x.sin());
            chart.plot(&x, &x.cos());
            chart.x().show_grid(ShowGrid::Major);
            chart.y().show_grid(ShowGrid::Major);

            chart.x().ticks(&[PI / 3., PI, 3. * PI / 2.]);
            chart.y().ticks(&[-0.5, -0.2, 0.0, 0.2, 0.50]);

            let mut chart = ui.chart();

            chart.plot(&x, &x.sin());
            chart.plot(&x, &x.cos());
            chart.x().show_grid(ShowGrid::Major);
            chart.y().show_grid(ShowGrid::Major);

            chart.x().tick_labels(&[(2., "A"), (3.14159, "PI"), (4., "C")]);
            chart.y().tick_labels(&[(-0.5, "M"), (-0.1, ""), (0.1, ""), (0.25, "A"), (0.40, "C")]);
        });
    });

    figure.show();
}
