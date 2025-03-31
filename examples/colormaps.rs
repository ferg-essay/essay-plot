use essay_plot::{chart::{Scaling, ShowGrid, SubFigure}, color::ColorMaps, plot::grid_color, prelude::*};
use essay_tensor::init::linspace;

fn main() { 
    let mut figure = Figure::new();
    // let mut graph1 = figure.chart(());
    figure.multichart(|ui| {
        ui.horizontal(|ui| {
            draw_grid(ui, ColorMaps::RedYellow);
            draw_grid(ui, ColorMaps::BlueOrange);
            draw_grid(ui, ColorMaps::VioletWhite);
        })
    });
    // let mut graph2 = figure.chart(());


    figure.show();
}

fn draw_grid(figure: &mut SubFigure, colormap: ColorMaps) {
    let mut chart = figure.chart();

    chart.scaling(Scaling::Image);
    chart.x().visible(false);

    // chart.y().visible(false);
    chart.y().show_grid(ShowGrid::Major);

    let z = linspace(0., 1., 100);
    let z = z.reshape([100, 1]);

    grid_color(&mut chart, &z)
        // .shading(Shading::Flat)
        .color_map(colormap);
    // grid_color(&mut graph2, &z).shading(Shading::Gouraud);
}
