use essay_graphics::layout::{Grid, Layout};
use essay_graphics::wgpu::WgpuBackend;

use essay_graphics::api::{
    driver::Backend,
    Bounds, Point,
};

use crate::frame::Frame;
use crate::graph::Graph; // , frame::{Layout, LayoutArc}};

use super::config::read_config;
use super::ConfigArc;

pub struct Figure {
    size: (f32, f32),
    dpi: f32,

    device: Box<dyn Backend>,
    // inner: FigureInner,
    config: ConfigArc,
    layout: Layout,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            // inner: Arc::new(Mutex::new(FigureInner::new())),
            // inner: FigureInner::new(),
            device: Box::new(WgpuBackend::new()),
            config: read_config().into_arc(),

            layout: Layout::new(),

            size: (6.4, 4.8),
            dpi: 200.,
        }
    }

    pub fn new_graph(&mut self, pos: impl Into<Bounds<Grid>>) -> Graph {
        Graph::new(self.layout.add_view(pos, Frame::new(&self.config)))
    }

    pub fn poly_graphs<'a, R: PolyRow<'a>>(&'a mut self, _layout: R) -> R::Item {
        todo!()
        //let mut row = 0;
        //R::axes(self, layout, &mut row)
    }

    pub fn show(self) {
        let layout = self.layout;
        let mut device = self.device;

        device.main_loop(Box::new(layout)).unwrap();
    }

    pub fn get_width(&self) -> f32 {
        self.size.0
    }

    pub fn get_height(&self) -> f32 {
        self.size.1
    }

    pub fn get_dpi(&self) -> f32 {
        self.dpi
    }

    pub fn save(&mut self, path: impl AsRef<std::path::Path>, dpi: f32) {
        crate::wgpu::draw_hardcopy(
            self.get_width() * dpi,
            self.get_height() * dpi,
            dpi,
            &mut self.layout, 
            path
        );    
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GraphId(usize);

impl GraphId {
    #[inline]
    pub fn index(&self) -> usize {
        self.0
    }
}

pub trait PolyRow<'a> {
    type Item;

    fn axes(figure: &'a mut Figure, layout: Self, row: &mut Counter) -> Self::Item;
}

pub trait PolyCol<'a> {
    type Item;

    fn axes(figure: &'a mut Figure, layout: Self, row: usize, col: &mut Counter) -> Self::Item;
}

impl<'a> PolyRow<'a> for [usize; 0] {
    type Item = GraphId;

    fn axes(figure: &'a mut Figure, _layout: Self, row: &mut Counter) -> Self::Item {
        PolyRow::axes(figure, [1, 1], row)
    }
}

impl<'a> PolyRow<'a> for [usize; 1] {
    type Item = GraphId;

    fn axes(figure: &'a mut Figure, layout: Self, row: &mut Counter) -> Self::Item {
        PolyRow::axes(figure, [layout[0], 1], row)
    }
}

impl<'a> PolyRow<'a> for [usize; 2] {
    type Item = GraphId;

    fn axes(_figure: &'a mut Figure, _layout: Self, _row: &mut Counter) -> Self::Item {
        todo!()
        /*
        let rows = layout[0];
        let cols = layout[1];

        let graph = figure.new_graph(Bounds::new(
            Point(0., row.0 as f32), 
            Point(0., (row.0 + rows) as f32),
        ));

        row.0 += rows;
        */

        //graph.id()
    }
}

impl<'a> PolyCol<'a> for [usize; 0] {
    type Item = Graph;

    fn axes(figure: &'a mut Figure, _layout: Self, row: usize, col: &mut Counter) -> Self::Item {
        PolyCol::axes(figure, [1], row, col)
    }
}

impl<'a> PolyCol<'a> for [usize; 1] {
    type Item = Graph;

    fn axes(figure: &'a mut Figure, layout: Self, row: usize, col: &mut Counter) -> Self::Item {
        let cols = layout[0];

        let axes = figure.new_graph(Bounds::new(
            Point(col.0 as f32, row as f32), 
            Point((col.0 + cols) as f32, row as f32),
        ));

        col.0 += cols;

        axes
    }
}

impl<'a> PolyCol<'a> for [usize; 2] {
    type Item = Graph;

    fn axes(figure: &'a mut Figure, layout: Self, row: usize, col: &mut Counter) -> Self::Item {
        let cols = layout[0];

        let axes = figure.new_graph(Bounds::new(
            Point(col.0 as f32, row as f32), 
            Point((col.0 + cols) as f32, row as f32),
        ));

        col.0 += cols;

        axes
    }
}

impl<'a> PolyRow<'a> for () {
    type Item = ();

    fn axes(_figure: &'a mut Figure, _layout: Self, row: &mut Counter) -> Self::Item {
        row.0 += 1;

        ()
    }
}

impl<'a, R1:PolyCol<'a>> PolyRow<'a> for (R1,) {
    type Item = (R1::Item,);

    fn axes(figure: &'a mut Figure, layout: Self, row: &mut Counter) -> Self::Item {
        let (r1,) = layout;
        (
            R1::axes(figure, r1, row.0, &mut Counter(0)),
        )
    }
}

impl<'a, R1:PolyCol<'a>, R2:PolyCol<'a>> PolyRow<'a> for (R1, R2) {
    type Item = (R1::Item, R2::Item);

    fn axes(_figure: &'a mut Figure, _layout: Self, _row: &mut Counter) -> Self::Item {
        todo!();
        /*
        let (r1, r2) = layout;
        (
            R1::axes(figure, r1, row.0, &mut Counter(0)),
            R2::axes(figure, r2, row.0, &mut Counter(0)),
        )
        */
    }
}

impl<'a, R1:PolyCol<'a>> PolyCol<'a> for (R1,) {
    type Item = (R1::Item,);

    fn axes(figure: &'a mut Figure, layout: Self, row: usize, col: &mut Counter) -> Self::Item {
        let (r1,) = layout;
        (
            R1::axes(figure, r1, row, col),
        )
    }
}

impl<'a, R1:PolyCol<'a>, R2:PolyCol<'a>> PolyCol<'a> for (R1, R2) {
    type Item = (R1::Item, R2::Item);

    fn axes(_figure: &'a mut Figure, _layout: Self, _row: usize, _col: &mut Counter) -> Self::Item {
        todo!();
        /*
        let (r1, r2) = layout;
        (
            R1::axes(figure, r1, row, col),
            R2::axes(figure, r2, row, col),
        )
        */
    }
}

pub struct Counter(usize);

#[cfg(test)]
mod test {
    // use super::Figure;

    #[test]
    fn test_polyaxes() {
        /*
        let mut figure = Figure::new();

        let axes = figure.poly_graphs([]);
        let axes = figure.poly_graphs(([], [2]));
        let axes = figure.poly_graphs((
            ([], []),
            ([2, 2]),
        ));
        */
    }
}

