use essay_plot::{
    chart::{Chart, Scaling, SubFigure}, 
    palette::{ColorMap, Colorcet}, 
    plot::grid_color, 
    prelude::*
};
use essay_tensor::init::linspace;

fn main() { 
    let mut figure = Figure::new();
    // let mut graph1 = figure.chart(());
    figure.multichart(|ui| {
        ui.horizontal(|ui| {
            draw_grid(ui, Colorcet::C1).title("C1");
            draw_grid(ui, Colorcet::C2).title("C2");
            draw_grid(ui, Colorcet::C3).title("C3");
            draw_grid(ui, Colorcet::C4).title("C4");
            draw_grid(ui, Colorcet::C5).title("C5");
            draw_grid(ui, Colorcet::C6).title("C6");
            draw_grid(ui, Colorcet::C7).title("C7");
        });
        ui.horizontal(|ui| {
            draw_grid(ui, Colorcet::D01).title("D01");
            draw_grid(ui, Colorcet::D01).title("D01A");
            draw_grid(ui, Colorcet::D02).title("D02");
            draw_grid(ui, Colorcet::D03).title("D03");
            draw_grid(ui, Colorcet::D04).title("D04");
            // draw_grid(ui, Colorcet::D05).title("D05");
            draw_grid(ui, Colorcet::D06).title("D06");
        });
        ui.horizontal(|ui| {
            draw_grid(ui, Colorcet::D07).title("D07");
            draw_grid(ui, Colorcet::D08).title("D08");
            draw_grid(ui, Colorcet::D09).title("D09");
            draw_grid(ui, Colorcet::D10).title("D10");
            draw_grid(ui, Colorcet::D11).title("D11");
            draw_grid(ui, Colorcet::D12).title("D12");
            draw_grid(ui, Colorcet::D13).title("D13");
        });
        ui.horizontal(|ui| {
            draw_grid(ui, Colorcet::L01).title("L01");
            draw_grid(ui, Colorcet::L02).title("L02");
            draw_grid(ui, Colorcet::L03).title("L03");
            draw_grid(ui, Colorcet::L04).title("L04");
            draw_grid(ui, Colorcet::L05).title("L05");
            draw_grid(ui, Colorcet::L06).title("L06");
            draw_grid(ui, Colorcet::L07).title("L07");
            draw_grid(ui, Colorcet::L08).title("L08");
            draw_grid(ui, Colorcet::L09).title("L09");
            draw_grid(ui, Colorcet::L10).title("L10");
        });
        ui.horizontal(|ui| {
            draw_grid(ui, Colorcet::L11).title("L11");
            draw_grid(ui, Colorcet::L12).title("L12");
            draw_grid(ui, Colorcet::L13).title("L13");
            draw_grid(ui, Colorcet::L14).title("L14");
            draw_grid(ui, Colorcet::L15).title("L15");
            draw_grid(ui, Colorcet::L16).title("L16");
            draw_grid(ui, Colorcet::L17).title("L17");
            draw_grid(ui, Colorcet::L18).title("L18");
            draw_grid(ui, Colorcet::L19).title("L19");
            draw_grid(ui, Colorcet::L20).title("L20");
        });
        ui.horizontal(|ui| {
            draw_grid(ui, Colorcet::I1).title("I1");
            draw_grid(ui, Colorcet::I2).title("I2");
            draw_grid(ui, Colorcet::I3).title("I3");

            draw_grid(ui, Colorcet::R2).title("R2");
            draw_grid(ui, Colorcet::R3).title("R3");
            draw_grid(ui, Colorcet::R4).title("R4");
        });
    });

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
