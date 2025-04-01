use essay_plot::{
    chart::{Chart, Scaling, SubFigure}, 
    palette::{ColorMap, Diverging, Sequential}, 
    plot::grid_color, 
    prelude::*
};
use essay_tensor::init::linspace;

fn main() { 
    let mut figure = Figure::new();
    // let mut graph1 = figure.chart(());
    figure.multichart(|ui| {
        ui.horizontal(|ui| {
            draw_grid(ui, Diverging::RedBlue).title("rd_bu");
            draw_grid(ui, Diverging::RedYellowBlue).title("rd_yl_bu");
            draw_grid(ui, Diverging::PurpleGreen).title("pu_gn");
            draw_grid(ui, Diverging::Spectral).title("Spectral");
        });
        ui.horizontal(|ui| {
            draw_grid(ui, Sequential::RedPurple).title("rd_pu");
            draw_grid(ui, Sequential::Blues).title("blues");            
            draw_grid(ui, Sequential::Viridis).title("viridis");
            draw_grid(ui, Sequential::Inferno).title("inferno");
            draw_grid(ui, Sequential::Magma).title("magma");
            draw_grid(ui, Sequential::Plasma).title("plasma");
        });
    });
    // let mut graph2 = figure.chart(());


    figure.show();
}

fn draw_grid(figure: &mut SubFigure, colormap: impl Into<ColorMap>) -> Chart {
    let mut chart = figure.chart();

    chart.scaling(Scaling::Image);
    chart.x().visible(false);

    chart.y().visible(false);
    //chart.y().show_grid(ShowGrid::Major);

    let z = linspace(0., 1., 100);
    let z = z.reshape([1, 100]);

    grid_color(&mut chart, &z)
        // .shading(Shading::Flat)
        .color_map(colormap);
    // grid_color(&mut graph2, &z).shading(Shading::Gouraud);

    chart
}
