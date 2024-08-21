use essay_plot::{artist::{ColorMaps, Shading}, chart::{Chart, Scaling, ShowGrid}, plot::grid_color, prelude::*};
use essay_tensor::init::{linspace, meshgrid};

fn main() { 
    let mut figure = Figure::new();
    // let mut graph1 = figure.chart(());
    draw_grid(&mut figure, (0., 0.), ColorMaps::RedYellow);
    draw_grid(&mut figure, (1., 0.), ColorMaps::BlueOrange);
    draw_grid(&mut figure, (2., 0.), ColorMaps::VioletWhite);
    // let mut graph2 = figure.chart(());


    figure.show();
}

fn draw_grid(figure: &mut Figure, pos: (f32, f32), colormap: ColorMaps) {
    let mut chart = figure.chart((pos, [1., 1.]));

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
