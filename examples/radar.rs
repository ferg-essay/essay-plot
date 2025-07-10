use essay_plot::{prelude::*, transform::AngleCoord};
use essay_tensor::{init::linspace, ten, tensor::Tensor};

fn main() {
    let mut figure = Figure::new();
    figure.multichart(|ui| {
        ui.horizontal(|ui| {
            let x = linspace(0., 100., 40);
            let one = Tensor::ones(x.shape());
    
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
                (0., "A".to_string()), 
                (1., "B".to_string()), 
                (2., "C".to_string()), 
                (3., "D".to_string()), 
            ]);
            chart.y().ticks(&[1., 2., 3., 4., 5., 7.5, 10.]);
            chart.y().visible(false);
            chart.radar(&y);
            
            let mut chart = ui.polar();

            chart.angle_coord(AngleCoord::Degrees);
            chart.x().tick_labels(&[
                (0., "A".to_string()), 
                (1., "B".to_string()), 
                (2., "C".to_string()), 
                (2.5, "D1".to_string()), 
                (3.5, "D2".to_string()), 
            ]);
            chart.y().ticks(&[1., 2., 3., 4., 5., 7.5, 10.]);
            chart.y().visible(false);
            let mut radar = chart.radar(&y);
            radar.set_y([10., 2., 3., 7.]);
        });
    });

    figure.show();
}
