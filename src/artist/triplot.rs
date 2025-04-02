use essay_graphics::api::{
    renderer::{Renderer, Result}, 
    Bounds, Path, PathCode, PathOpt, Point
};
use essay_tensor::Tensor;

use crate::{chart::Data, tri::{Triangulation, triangulate}};

use super::{ArtistDraw, ToCanvas};

pub struct TriPlot {
    data: Tensor,
    triangulation: Option<Triangulation>,
    is_stale: bool,
}

impl TriPlot {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "triplot requires 2d value {:?}", data.shape().as_slice());
        assert!(data.cols() == 2, "triplot requires 2d value {:?}", data.shape().as_slice());

        Self {
            data,
            triangulation: None,
            is_stale: true,
        }
    }

    fn resize(&mut self, _renderer: &mut dyn Renderer) {
        if self.is_stale {
            self.is_stale = false;
            self.triangulation = Some(triangulate(&self.data));
        }
    }
}

impl ArtistDraw<Data> for TriPlot {
    fn bounds(&mut self) -> Bounds<Data> {
        Bounds::from(&self.data)
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        self.resize(renderer);

        if let Some(tri) = &self.triangulation {
            let mut codes = Vec::<PathCode>::new();

            let xy = tri.vertices();
            for edge in tri.edges().iter_row() {
                let (x0, y0) = (xy[(edge[0], 0)], xy[(edge[0], 1)]);
                let (x1, y1) = (xy[(edge[1], 0)], xy[(edge[1], 1)]);

                codes.push(PathCode::MoveTo(Point(x0, y0)));
                codes.push(PathCode::LineTo(Point(x1, y1)));
            
            }

            let path = Path::<Data>::new(codes);
            let path = to_canvas.transform_path(&path);

            renderer.draw_path(&path, style)?;
        }

        Ok(())
    }
}
