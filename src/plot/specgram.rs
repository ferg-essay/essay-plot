use essay_tensor::{array::stack, init::linspace, signal::rfft_norm, Tensor};

use crate::{artist::{Lines2d, LinesOpt}, graph::Graph};

use super::{grid_color, matshow};

pub fn specgram(
    graph: &mut Graph, 
    y: impl Into<Tensor>, 
) {
    let y = y.into();
    let nfft = 256;
    let overlap = 128;

    let mut i : usize = 0;
    let mut values : Vec<Tensor> = Vec::new();
    while i + nfft <= y.len() {
        let slice = y.subslice(i, nfft);

        let value = rfft_norm(slice, ());

        values.push(value);

        i += overlap;
    }

    let v2 = stack(values, 1);

    grid_color(graph, &v2);
}
