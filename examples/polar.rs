use std::f32::consts::PI;

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
                (0., "A".to_string()), 
                (1., "B".to_string()), 
                (2., "C".to_string()), 
                (2.4, "D".to_string()), 
                (2.6, "E".to_string())]
            );
            chart.y().tick_labels(&[
                (1., "".to_string()), 
                (2., "".to_string()), 
                (3., "".to_string()), 
                (4., "".to_string()), 
                (5., "J".to_string()), 
                (7.5, "K".to_string()), 
                (10., "L".to_string()), 
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
