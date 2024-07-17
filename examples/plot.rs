use essay_plot::{prelude::*, artist::{patch::PathPatch, Markers, IntoMarker, DrawStyle}, plot::{bar, grid_color, contour, triplot, tricontour, matshow, stem, fill_between, plot}, graph::{Graph, PlotOpt}};
use essay_plot::api::{Point, Color, PathCode, Path, JoinStyle, CapStyle, LineStyle, Angle};
use essay_tensor::{prelude::*, init::{linspace, meshgrid, meshgrid_ij}, tensor::TensorVec};

fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();
    let mut graph = figure.new_graph([1., 1.]);

    //let x = linspace(0., 2. * PI, 30);
    //let y = x.sin();

    let x = linspace(0., 6.28, 20);
    let y = x.sin();
    // gui.main_loop().unwrap();
    //axes.pcolor();

    // axes.title("My Title").style().color(0x008033);
    // axes.xlabel("My X-Label").style().color(0x0030ff);
    // axes.ylabel("Y-Label").style().color("r");

    graph.title("My Title"); // .color(0x008033).size(18.);
    //graph.x_label("My X-Label"); // .color("brown");
    //graph.y_label("Y-Label"); // .color("teal").size(8.);
    graph.plot(&x, &y);
    // graph.xlim(0., 1.);
/*
    graph.scatter(&x, &y).color("blue").marker("X")
        .line_color(0xff8000)
        .size(2500.)
        // .fill_color("none")
        .line_width(5.);
    */

    let x = linspace(0., 6.28, 51);
    let y1 = x.sin();
    let y2 = (2. * &x).cos();
    //let y2 = linspace(2., 2., 21);
    //graph.scatter(&x, &y).color("blue").marker(Markers::Asterisk(5, Angle::Deg(0.))); // .size(2500.);
    //graph.plot(&x, &y).color("xkcd:purple"); // .label("sin");

    //graph.scatter(&x, &y).color("blue").marker(Markers::Asterisk(5, Angle::Deg(0.))); // .size(2500.);
    //graph.plot(&x, &y).color("xkcd:amber"); // .label("cos");
    //fill_between(graph, &x, y1, y2);
    //fill_between(graph, &x, y1, y2);
    //stem(graph, x, y);
    /*
    let x = linspace(0., 6.28, 21);
    let y = linspace(0., 6.28, 21);
    let [grid_x, grid_y] = meshgrid([&x, &y]);
    let grid_x = grid_x.flatten();
    let grid_y = grid_y.flatten();

    let xy = grid_x.stack([grid_y.clone()], -1);
    let mut vec = TensorVec::<[f32; 2]>::new();
    vec.push([0., 0.]);
    vec.push([4., 0.]);
    vec.push([2., 4.]);

    vec.push([1.5, 2.]);
    vec.push([2.5, 2.]);
    vec.push([2., 1.]);
    */

    //let z = &grid_x.sin() + &grid_y.cos();

    // triplot(graph, vec.into_tensor());
    //tricontour(graph, xy, z);

    //matshow(&mut graph, &z);
    //pcolormesh(graph, &z);


    //pcolormesh(graph, &z);
    //contour(graph, &z);
    //graph.colorbar();
    /*
    let y2 = x.cos();
    graph.plot(&x, &y2).face_color("xkcd:purple");

    graph.plot(&x, (2. * &x).sin());
    graph.plot(&x, (2. * &x).cos());

    graph.plot(&x, 0.5 * (3. * &x).sin());
    graph.plot(&x, 0.5 * (3. * &x).cos());

    graph.x().show_grid(true);
    //graph.x().major_grid().color(0xc04040).line_width(1.5);
    //graph.x().major().color(0x2000c0).line_width(1.5);
    graph.y().show_grid(true);
    */
    //bar_y(graph, &y)
    //    .edgecolor(0x400080)
    //    .facecolor(0x80c0e0)
    //    .width(0.2);
    
    //axes.scatter(&x, &y, ());
    
    //let x = tf32!([40., 30., 20., 5., 5.]);
    //let x = tf32!([40., 30.]);
    let x = tf32!([25., 25., 50.]);
    // let axes = figure.new_graph(());
    //graph.pie(tf32!([40., 30., 20., 5., 5., 5., 5.]));
    // let x = linspace(0., 20., 21);
    // let axes = figure.new_graph([1., 1., 2., 2.]);
    // axes.plot(&x, &x.exp(), ());
    //bezier2(graph, [-0.5, 0.], [-1.0, 1.0], [-1.5, 0.0]).color(Color(0x0080c080));
    
    /*
    plot_quad(graph, [0.0, 0.0], [1.0, 0.0], [1., 1.], [0., 1.])
        .facecolor(Color(0))
        .edgecolor(0xe08000)
        .linewidth(20.)
        .joinstyle(JoinStyle::Bevel);
    */
    
    /*
    plot_line(graph, 
        [0.0, 0.0], [1.0, 0.0],
        [1., 1.], [0., 1.],
        [0.5, 0.25], [0.5, 0.75],
    ).facecolor(Color(0))
        .edgecolor(0xe08000)
        .linewidth(20.)
        .joinstyle(JoinStyle::Miter)
        .capstyle(CapStyle::Round);
    */
    
    //bezier2(graph, [0.5, 0.], [1.0, 1.0], [1.5, 0.0]).color(Color(0x0080c080));
    //bezier2(graph, [-1.5, 0.], [-1.0, -1.0], [-0.5, 0.0]).color(Color(0x0080c080));
    //bezier2(graph, [0., -0.5], [1.0, -1.0], [0.0, -1.5]).color(Color(0x0080c080));

    //axes.bezier2([-1., 0.], [0.5, 1.0], [1.0, 0.0]);
    // axes.bezier2([0., -1.], [-0.5, 0.0], [0.0, 1.]);
    //axes.bezier3([0., 0.], [0.25, 1.0], [0.5, -1.0], [1.0, 0.0]);
    //axes.bezier3([-1., 0.], [1.0, 1.0], [-1.0, -1.0], [1.0, 0.0]);
    figure.show();

    /*
    let mut figure = Figure::new();

    let x = tf32!([1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
    let y = &x * &x + tf32!(1.);

    figure.plot(&x, &y, ()); // , ().label("My Item"));
    //plot.plot(&x, &y, ().label("My Item"));
    //plot.scatter(&x, &x * &x * &x, ().label("My Item 3"));
    //plot.set_title("My Title");
    //plot.set_xlabel("My x-axis");

    figure.show();
    */
}

pub fn bezier3(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>
) {
    //graph.add_data_artist(Bezier3(p0.into(), p1.into(), p2.into(), p3.into()));
    todo!()
}

pub fn bezier2(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
) -> PlotOpt {
    //graph.add_data_artist(Bezier2(p0.into(), p1.into(), p2.into()))
    todo!()
}

pub fn plot_quad(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::LineTo(p1.into()),
        PathCode::LineTo(p2.into()),
        PathCode::ClosePoly(p3.into()),
    ])))
}

pub fn plot_line(
    graph: &mut Graph, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>,
    p4: impl Into<Point>,
    p5: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::LineTo(p1.into()),
        PathCode::MoveTo(p2.into()),
        PathCode::LineTo(p3.into()),
        PathCode::MoveTo(p4.into()),
        PathCode::LineTo(p5.into()),
    ])))
}
