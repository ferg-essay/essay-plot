use essay_plot::{artist::{ColorMaps, Shading}, chart::{Chart, Scaling, ShowGrid}, plot::grid_color, prelude::*};
use essay_tensor::{init::{linspace, meshgrid}, Tensor};

fn main() { 
    let mut figure = Figure::new();

    let mut vec = Vec::<f32>::new();
    let w = 20;
    let len = 150;

    for _ in 0..20 {
      add_line(&mut vec, 0, len, w);
    }

    for i in 0..50 {
        let j = (2. * i as f32 + (i * i) as f32 * 0.3) as usize;
        add_line(&mut vec, j + 20, len, w);
    }
    /*
    for i in 1..28 {
        let j = i * 20 + ((i * i) as f32 * 0.15) as usize;
        add_line(&mut vec, j + 150, len, w);
    }
    */

    let data = Tensor::from(vec);
    let len = data.len();
    let data = data.reshape([len / 1000, 1000]);
    // let mut graph1 = figure.chart(());
    draw_grid(&mut figure, (1., 0.), data, ColorMaps::BlueOrange);
    // let mut graph2 = figure.chart(());


    figure.show();
}

fn add_line(data: &mut Vec<f32>, start: usize, len: usize, w: usize) {
    let w_b = (len - w) / 2;
    let w_e = len - w_b;

    for _ in 0..start {
        data.push(0.);
    }


    for i in 0..w_b {
        data.push((i + 1) as f32 / (w_b + 1) as f32);
    }

    for _ in w_b..w_e {
        data.push(1.0);
    }

    let end = (start + len).min(1000);

    for i in w_e..len {
        data.push((len - i) as f32 / (w_b + 1) as f32);
    }

    for _ in end..1000 {
        data.push(0.);
    }
}

fn draw_grid(figure: &mut Figure, pos: (f32, f32), data: Tensor, colormap: ColorMaps) {
    let mut chart = figure.chart();

    chart.scaling(Scaling::Image);
    //chart.x().visible(false);

    // chart.y().visible(false);
    // chart.y().show_grid(ShowGrid::Major);
    chart.flip_y(true);

    grid_color(&mut chart, &data)
        // .shading(Shading::Flat)
        .color_map(colormap);
    // grid_color(&mut graph2, &z).shading(Shading::Gouraud);
}
