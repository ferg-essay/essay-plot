use std::f32::consts::TAU;

use essay_plot::{
    artist::{DrawStyle, Patch, Shading}, plot::grid_color, prelude::*
};
use essay_tensor::{init::{linspace, meshgrid}, ten};

fn main() {
    let t = 0;
    match t {
        0 => main_plot(),
        1 => main_plot2(),
        2 => main_grid(),
        _ => main_plot(),
    }
}

fn main_plot() {
    let mut figure = Figure::new();
    let mut chart = figure.chart();

    let x = linspace(0., TAU, 20);
    let y = x.sin();

    chart.title("My Title");

    chart.plot(&x, &y);
    
    let patch = Patch::rect([0.5, 0.5], [2., 2.]);
    chart.artist(patch).color("red").edge_color("black");
    // graph.artist()

    figure.save("../test.png", 144.);
    figure.show();
}

fn main_grid() { 
    let x = linspace(0., 2. * 6.28, 51);
    let y = linspace(0., 6.28, 101);
    let [x, y] = meshgrid([x, y]);

    let z = &x.sin() + &y.sin();

    let mut figure = Figure::new();
    let mut graph1 = figure.chart();
    let mut graph2 = figure.chart();

    grid_color(&mut graph1, &z).shading(Shading::Flat);
    grid_color(&mut graph2, &z).shading(Shading::Gouraud);

    figure.save("../test.png", 144.);
}

fn main_plot2() {
    let mut figure = Figure::new();

    let ((mut graph1, mut graph2), (mut graph3, mut graph4))
        = figure.multichart(|ui| {
        (
            ui.horizontal(|ui| {
                (ui.chart(), ui.chart())
            }),
            ui.horizontal(|ui| {
                (ui.chart(), ui.chart())
            }),
        )
    });

    let x = linspace(0., 1., 10);
    let odor = ten![
        0., 1., 1., 1., 1., 1., 1., 1., 1., 0.,
    ] * 0.9f32;
    let failure = ten![
        1., 1., 1., 1., 1., 1., 1., 1., 0., 0.,
    ];

    graph1.title("Signal");
    graph1.ylim(0., 2.0);
    graph1.plot(&x, &odor).label("odor");
    graph1.plot(&x, &failure).label("LTD").draw_style(DrawStyle::StepsMid);
    graph1.x().visible(false);
    graph1.y().visible(false);

    graph2.title("Timeout");
    graph2.ylim(0., 2.0);
    graph2.plot(&x, &odor).label("odor");
    let x2 = ten![0., 0.1, 0.8, 1.0];
    let failure2 = ten![
        1., 1., 0., 0.,
    ];
    graph2.plot(&x2, &failure2).label("LTD");
    graph2.x().visible(false);
    graph2.y().visible(false);

    graph3.title("Signal");
    graph3.plot(&x, &odor).label("odor");
    let failure = ten![
        1., 1., 1., 1.,
    ];
    graph3.plot(&x2, &failure).label("LTD").draw_style(DrawStyle::StepsMid);

    let x3 = ten![
        0., 0.6, 0.7, 1.,
    ];
    let reward = ten![
        0., 0.8, 0., 0.,
    ];
    graph3.plot(&x3, &reward).label("reward").draw_style(DrawStyle::StepsMid);
    graph3.ylim(0., 2.0);
    graph3.x().visible(false);
    graph3.y().visible(false);

    graph3.title("Signal");

    graph4.title("Timeout");
    graph4.plot(&x, &odor).label("odor");
    graph4.plot(&x2, &failure2).label("LTD");

    let x3 = ten![
        0., 0.6, 0.7, 1.,
    ];
    let reward = ten![
        0., 0.8, 0., 0.,
    ];
    graph4.plot(&x3, &reward).label("reward").draw_style(DrawStyle::StepsMid);
    graph4.ylim(0., 2.0);
    graph4.x().visible(false);
    graph4.y().visible(false);
    //graph3.title("Reward");
    //graph4.title("Reward");

    figure.save("../test.png", 144.);
    figure.show();
}
