use essay_graphics::api::{
    renderer::{Renderer, Result}, 
    Bounds, Path, PathCode, PathOpt, Point
};
use essay_tensor::tensor::Tensor;

use crate::{artist::artist::Stale, chart::Data, transform::ToCanvas, tri::{triangulate, Triangulation}};

use super::ArtistDraw;

pub struct TriPlot {
    data: Tensor,
    triangulation: Option<Triangulation>,
    is_stale: bool,
    stale: Stale,
}

impl TriPlot {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "triplot requires 2d value {:?}", data.shape());
        assert!(data.cols() == 2, "triplot requires 2d value {:?}", data.shape());

        Self {
            data,
            triangulation: None,
            is_stale: true,
            stale: Stale::default(),
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
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        self.stale = to_canvas.stale().eq_or(self.stale, || {
            self.resize(renderer);
        });

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
