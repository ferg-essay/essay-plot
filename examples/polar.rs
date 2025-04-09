use std::f32::consts::{PI, TAU};

use essay_plot::{prelude::*, transform::AngleCoord};
use essay_tensor::{init::linspace, tensor::Tensor};

fn main() {
    let mut figure = Figure::new();
    figure.multichart(|ui| {
        let x = linspace(0., 100., 40);
        let one = Tensor::ones(x.shape());
    
        let theta = PI * &x / 100.;
    
        let mut chart = ui.polar();

        chart.angle_coord(AngleCoord::Radians);
        chart.plot(&x, &one);
        chart.plot(&x, (3. * &theta).sin());
        chart.plot(&x, 2. * &theta.cos());
    
    
        let mut chart = ui.polar();

        chart.angle_coord(AngleCoord::Degrees);
        chart.plot(&x, &one);
        chart.plot(&x, (3. * &theta).sin());
        chart.plot(&x, 2. * &theta.cos());
    
    });

    figure.show();
}
