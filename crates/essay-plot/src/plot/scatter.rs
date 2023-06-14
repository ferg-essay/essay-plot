use essay_plot_base::{Canvas, Path, Affine2d};
use essay_tensor::Tensor;

use crate::{artist::{Collection, paths}, graph::{Graph, PlotOpt}};


pub fn scatter(
    graph: &mut Graph, 
    x: impl Into<Tensor>, 
    y: impl Into<Tensor>, 
) -> PlotOpt {
    let x : Tensor = x.into();
    let y = y.into();
    let xy = x.stack(&[y], -1); // Axis::axis(-1));
    let path: Path<Canvas> = paths::unit().transform(
        &Affine2d::eye().scale(10., 10.)
    );

    let collection = Collection::new(path, &xy);

    graph.add_data_artist(collection)
}
