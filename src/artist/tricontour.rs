use essay_graphics::api::{renderer::{Canvas, Renderer, Result}, Bounds, Path, PathOpt};
use essay_tensor::tensor::Tensor;

use crate::{chart::Data, config::PathStyle, contour::TriContourGenerator, tri::Triangulation};

use super::{ArtistDraw, ToCanvas};

pub struct Level {
    paths: Vec<Path<Data>>,
}

impl Level {
    fn new(paths: Vec<Path<Data>>) -> Self {
        Self {
            paths,
        }
    }
}

pub struct TriContour {
    data: Tensor,
    // color_map: ColorMap,

    tri: Triangulation,
    norm: Tensor,
    levels: Vec<Level>,
    bounds: Bounds<Data>,
}

impl TriContour {
    pub fn new(tri: impl Into<Triangulation>, data: impl Into<Tensor>) -> Self {
        let tri: Triangulation = tri.into();
        let data : Tensor = data.into();

        assert!(data.rank() == 1, "contour requires 1d value {:?}", data.shape());

        Self {
            data,
            tri,
            norm: Tensor::from(None),
            // color_map: ColorMaps::Default.into(),
            bounds: Bounds::zero(),
            levels: Vec::new(),
        }
    }

    pub(crate) fn _set_data(&mut self, data: Tensor) {
        assert!(data.rank() == 2, "contour requires 2d value {:?}", data.shape());

        self.data = data;
    }
    fn resize(&mut self, _renderer: &mut dyn Renderer) {
        //let (rows, cols) = (self.data.rows(), self.data.cols());

        //for vert in self.tri.triangles().iter_slice() {
        //  xy.push([i as f32, j as f32]);
        //}

        self.bounds = Bounds::<Data>::from(self.tri.vertices());

        self.norm = self.data.normalize_unit();

        let mut cg = TriContourGenerator::new(&self.tri, self.data.clone());

        let level_thresholds = vec![
            -1.5,
            -1.,
            -0.5, 
            0., 
            0.5,
            1.,
            1.5,
            ];
        let mut levels = Vec::<Level>::new();

        for threshold in &level_thresholds {
            let paths = cg.contour_lines(*threshold);

            let paths: Vec<Path<Data>> = paths.iter()
                .map(|p| Path::<Data>::lines(p))
                .collect();

            levels.push(Level::new(paths));
        }

        self.levels = levels;

        // self.xy = xy.into_tensor();
    }

}

impl ArtistDraw<Data> for TriContour {
    fn bounds(&mut self) -> Bounds<Data> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        _style: &dyn PathOpt,
    ) -> Result<()> {
        self.resize(renderer);

        let mut style = PathStyle::new();

        style.edge_color("k");
        style.line_width(1.);

        //renderer.draw_markers(&path, &xy, &tf32!(), &colors, &style, clip).unwrap();

        for level in &self.levels {
            for path in &level.paths {
                let path : Path<Canvas> = to_canvas.transform_path(path);

                renderer.draw_path(&path, &style)?;
            }
        }

        Ok(())
    }
}
