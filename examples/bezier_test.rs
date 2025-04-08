use essay_plot::{
    prelude::*,
    artist::{patch::PathPatch, paths}, 
    chart::{Chart, Data, PlotOpt}, 
};
use essay_plot::api::{Point, Color, PathCode, Path, Angle};


fn main() {
    //let mut gui = WgpuBackend::new();

    let mut figure = Figure::new();
    let mut chart = figure.chart();

    let test = Tests::WEDGE;

    chart.add_simple_artist(PathPatch::new(test.path())).color(Color(0x0080c080));

    /*
    match test {
        Tests::A => {
            graph.add_simple_artist(PathPatch::new(
                Path::move_to(0.5, 0.)
                    .bezier2_to([1.0, 1.0], [1.5, 0.0])
                    .to_path())
            ).color(Color(0x0080c080));
        },
        Tests::A_P => {
            graph.add_simple_artist(PathPatch::new(
                Path::move_to(0.5, 0.)
                    .bezier2_to([1.0, 1.0], [1.5, 0.0])
                    .close_poly(0.5, 0.)
                    .to_path())
            ).color(Color(0x0080c080));
        },
        Tests::SEMICIRCLE => {
            graph.add_simple_artist(PathPatch::new(
                Path::move_to(0.0, 0.)
                    .bezier2_to([0.5, 1.0], [1., 0.0])
                    .line_to(0.75, 0.)
                    .bezier2_to([0.5, 0.5], [0.25, 0.0])
                    .close_poly(0., 0.)
                    .to_path())
            ).color(Color(0x0080c080));
        },
        Tests::CUT_BOX => {
            graph.add_simple_artist(PathPatch::new(
                Path::move_to(0.0, 0.)
                    .line_to(0., 1.)
                    .line_to(1., 1.)
                    .line_to(1., 0.)
                    .line_to(0.75, 0.)
                    .bezier2_to([0.5, 0.5], [0.25, 0.0])
                    .close_poly(0., 0.)
                    .to_path())
            ).color(Color(0x0080c080));
        },
        Tests::CUT_BOX_B => {
            graph.add_simple_artist(PathPatch::new(
                Path::move_to(0.0, 0.)
                    .line_to(0., 1.)
                    .line_to(1., 1.)
                    .line_to(1., 0.)
                    .line_to(0.75, 0.)
                    .line_to(0.5, 0.5)
                    .line_to(0.25, 0.0)
                    .close_poly(0., 0.)
                    .to_path())
            ).color(Color(0x0080c080));
        },
    }
    */
    //bezier2(graph, [-1.5, 0.], [-1.0, -1.0], [-0.5, 0.0]).color(Color(0x0080c080));
    //bezier2(graph, [0., -0.5], [1.0, -1.0], [0.0, -1.5]).color(Color(0x0080c080));

    //axes.bezier2([-1., 0.], [0.5, 1.0], [1.0, 0.0]);
    // axes.bezier2([0., -1.], [-0.5, 0.0], [0.0, 1.]);
    //axes.bezier3([0., 0.], [0.25, 1.0], [0.5, -1.0], [1.0, 0.0]);
    //axes.bezier3([-1., 0.], [1.0, 1.0], [-1.0, -1.0], [1.0, 0.0]);
    figure.show();
}
enum Tests {
    A, A_P,
    CIRCLE,
    WEDGE,
    SEMICIRCLE, SEMICIRCLE_T,
    CUT_BOX, CUT_BOX_B,
    HOLLOW_BOX, HOLLOW_BOX_BZ,
}

impl Tests {
    fn path(&self) -> Path<Data> {
        match self {
            Tests::A => todo!(),
            Tests::A_P => todo!(),
            Tests::CIRCLE => {
                paths::circle().map(|pt| pt)
            },
            Tests::WEDGE => {
                paths::wedge((Angle::Unit(0.0), Angle::Unit(0.25))).map(|pt| pt)
            },
            Tests::SEMICIRCLE => {
                Path::move_to(0.0, 0.)
                    .bezier2_to([0.5, 1.0], [1., 0.0])
                    .line_to(0.75, 0.)
                    .bezier2_to([0.5, 0.5], [0.25, 0.0])
                    .close_poly(0., 0.)
                    .to_path()
            },
            Tests::SEMICIRCLE_T => {
                Path::move_to(0.0, 0.)
                    .line_to(0.5, 1.0)
                    .line_to(1., 0.0)
                    .line_to(0.75, 0.)
                    .line_to(0.5, 0.5)
                    .line_to(0.25, 0.0)
                    .close_poly(0., 0.)
                    .to_path()
            },
            Tests::CUT_BOX => {
                Path::move_to(0.0, 0.)
                .line_to(0.25, 0.)
                .bezier2_to([0.5, 0.5], [0.75, 0.0])
                .line_to(1., 0.)
                .line_to(1., 1.)
                .close_poly(0., 1.)
                .to_path()
            },
            Tests::CUT_BOX_B => {
                Path::move_to(0.0, 0.)
                    .line_to(0., 1.)
                    .line_to(1., 1.)
                    .line_to(1., 0.)
                    .line_to(0.75, 0.)
                    .line_to(0.5, 0.5)
                    .line_to(0.25, 0.0)
                    .close_poly(0., 0.)
                    .to_path()
            },
            Tests::HOLLOW_BOX => {
                Path::move_to(0.0, 0.)
                .line_to(0., 1.)
                .line_to(1., 1.)
                .line_to(1., 0.)
                .close_poly(0., 0.)
                .move_to(0.25, 0.25)
                .line_to(0.25, 0.75)
                .line_to(0.75, 0.75)
                .line_to(0.75, 0.25)
                .close_poly(0.25, 0.25)
                .to_path()
            },
            Tests::HOLLOW_BOX_BZ => {
                Path::move_to(0.0, 0.)
                .line_to(0., 1.)
                .line_to(1., 1.)
                .line_to(1., 0.)
                .close_poly(0., 0.)
                .move_to(0.25, 0.25)
                .line_to(0.25, 0.75)
                .bezier2_to([0.5, 1.], [0.75, 0.75])
                .line_to(0.75, 0.25)
                .close_poly(0.25, 0.25)
                .to_path()
            },
        }
    }
}

pub fn bezier3(
    graph: &mut Chart, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
    p3: impl Into<Point>
) {
    //graph.add_data_artist(Bezier3(p0.into(), p1.into(), p2.into(), p3.into()));
    todo!()
}

pub fn bezier2(
    graph: &mut Chart, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
) -> PlotOpt {
    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0.into()),
        PathCode::Bezier2(p1.into(), p2.into()),
    ])))
}

pub fn bezier2_poly(
    graph: &mut Chart, 
    p0: impl Into<Point>,
    p1: impl Into<Point>,
    p2: impl Into<Point>,
) -> PlotOpt {
    let p0 = p0.into();

    graph.add_simple_artist(PathPatch::new(Path::new(vec![
        PathCode::MoveTo(p0),
        PathCode::Bezier2(p1.into(), p2.into()),
        PathCode::ClosePoly(p0),
    ])))
}

pub fn plot_quad(
    graph: &mut Chart, 
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
    graph: &mut Chart, 
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
