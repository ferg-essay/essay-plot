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
            chart.radar(&one);
    
            let mut chart = ui.polar();

            chart.angle_coord(AngleCoord::Degrees);
            chart.radar(&one);
        });

        ui.horizontal(|ui| {
            let y = ten![10., 4., 1., 5.];

            let mut chart = ui.polar();

            chart.angle_coord(AngleCoord::Degrees);
            // chart.x().ticks(&[0., 1., 2., 2.4, 2.6]);
            chart.x().tick_labels(&[
                (0., "A"), 
                (1., "B"), 
                (2., "C"), 
                (3., "D"), 
            ]);
            chart.y().ticks(&[1., 2., 3., 4., 5., 7.5, 10.]);
            chart.y().visible(false);
            chart.radar(&y);
            
            let mut chart = ui.polar();

            chart.angle_coord(AngleCoord::Degrees);
            chart.x().tick_labels(&[
                (0., "A"), 
                (1., "B"), 
                (2., "C"), 
                (2.5, "D1"), 
                (3.5, "D2"), 
            ]);
            chart.y().ticks(&[1., 2., 3., 4., 5., 7.5, 10.]);
            chart.y().visible(false);
            let mut radar = chart.radar(&y);
            radar.set_y([10., 2., 3., 7.]);
        });
    });

    figure.show();
}
