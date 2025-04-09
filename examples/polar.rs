use std::f32::consts::{PI, TAU};

use essay_plot::{prelude::*, transform::AngleCoord};
use essay_tensor::{init::linspace, ten, tensor::Tensor};

fn main() {
    let mut figure = Figure::new();
    figure.multichart(|ui| {
        ui.horizontal(|ui| {
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

        ui.horizontal(|ui| {
            let x = ten![0., 1., 2., 3., 4.];
            let y = ten![10., 5., 1., 5., 10.];

            let mut chart = ui.polar();

            chart.angle_coord(AngleCoord::Degrees);
            // chart.x().ticks(&[0., 1., 2., 2.4, 2.6]);
            chart.x().tick_labels(&[
                (0., "A"), 
                (1., "B"), 
                (2., "C"), 
                (2.4, "D"), 
                (2.6, "E")]
            );
            chart.y().tick_labels(&[
                (1., ""), 
                (2., ""), 
                (3., ""), 
                (4., ""), 
                (5., "J"), 
                (7.5, "K"), 
                (10., "L"), 
            ]);
            chart.plot(&x, &y);
            
            let mut chart = ui.polar();

            chart.angle_coord(AngleCoord::Degrees);
            chart.x().visible(false);
            chart.y().visible(false);
            chart.plot(&x, &y);
        });
    });

    figure.show();
}
