use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, 
    Bounds, CapStyle, Color, JoinStyle, Path, PathOpt, Point
};
use essay_tensor::{tensor::Tensor, ten};

use crate::{
    artist::{Norm, Norms}, chart::{Data, LegendHandler}, palette::{ColorMap, EssayColors}, config::{ConfigArc, PathStyle}, data_artist_option_struct
};

use super::{Artist, ArtistDraw, ArtistView, ToCanvas};

pub enum Shading {
    Flat,
    Gouraud,
}

pub struct GridColor {
    data: Tensor,
    xy: Tensor,
    color_map: ColorMap,
    shading: Shading,
    norm: Norm,

    is_stale: bool,
}

impl GridColor {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "colormesh requires 2d value {:?}", data.shape());

        Self {
            data,
            xy: Tensor::from(None),
            color_map: EssayColors::Default.into(),
            shading: Shading::Flat,
            norm: Norm::from(Norms::Linear),
            is_stale: true,
        }
    }

    pub(crate) fn set_data(&mut self, data: Tensor) {
        assert!(data.rank() == 2, "colormesh requires 2d value {:?}", data.shape());

        self.data = data;
        self.is_stale = true;
    }

    pub(crate) fn norm(&mut self, norm: impl Into<Norm>) {
        self.norm = norm.into();
    }

    pub(crate) fn color_map(&mut self, color_map: impl Into<ColorMap>) {
        self.color_map = color_map.into();
    }

    pub(crate) fn shading(&mut self, shading: impl Into<Shading>) {
        self.shading = shading.into();
    }

    fn draw_solid_shading(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        _style: &dyn PathOpt,
    ) -> Result<()> {
        let path = Path::<Data>::closed_poly(ten![
            [0.0, 0.0], [1.0, 0.0], [1.0, 1.0],
            [0.0, 1.0]
            ]);
            
        let to_canvas = to_canvas.affine2d();
        let scale_canvas = to_canvas.strip_translation();
        let path: Path<Canvas> = path.map(|pt| scale_canvas.transform_point(pt));
        let xy = to_canvas.transform(&self.xy);

        // let norm = normalize_unit(&self.data);

        let colormap = &self.color_map;

        let colors = self.data.iter().map(|v| {
            let v = self.norm.norm(*v);
            colormap.map(v).to_rgba()
        }).collect();

        let mut style = PathStyle::new();
        style.edge_color(Color(0));
        style.line_width(0.);

        // style.edge_color("k");
        style.join_style(JoinStyle::Bevel);
        style.cap_style(CapStyle::Butt);

        renderer.draw_markers(&path, &xy, &Tensor::from(None), &colors, &style)
    }

    fn draw_gouraud_shading(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        _style: &dyn PathOpt,
    ) -> Result<()> {
        let xy = to_canvas.transform_tensor(&self.xy);

        let norm = self.data.normalize_unit();
        
        let cmap = &self.color_map;

        let mut vertices = Vec::<[f32; 2]>::new();
        let mut colors = Vec::<u32>::new();
        let mut triangles = Vec::<[u32; 3]>::new();

        let (rows, cols) = (norm.rows(), norm.cols());

        let j_stride = cols; //  + 1;

        for j in 0..rows {
            for i in 0..cols {
                let x0 = xy[(j * j_stride + i, 0)];
                let y0 = xy[(j * j_stride + i, 1)];
                
                vertices.push([x0, y0]);
                colors.push(cmap.map(norm[(j, i)]).to_rgba());

                if i + 1 < cols && j + 1 < rows {
                    triangles.push([
                        (j * j_stride + i) as u32, 
                        (j * j_stride + i + 1) as u32, 
                        ((j + 1) * j_stride + i + 1) as u32,
                    ]);

                    triangles.push([
                        ((j + 1) * j_stride + i + 1) as u32, 
                        ((j + 1) * j_stride + i) as u32, 
                        (j * j_stride + i) as u32,
                    ]);
                }
            }
        }

        let vertices = Tensor::from(vertices);
        let colors = Tensor::from(colors);
        let triangles = Tensor::from(triangles);

        renderer.draw_triangles(&vertices, &colors, &triangles)
    }

    pub(crate) fn set_norm(&mut self, min: f32, max: f32) {
        self.norm.set_vmin(min);
        self.norm.set_vmax(max);
    }
}

impl ArtistDraw<Data> for GridColor {
    fn bounds(&mut self) -> Bounds<Data> {
        let (rows, cols) = match self.shading {
            Shading::Gouraud => (self.data.rows() - 1, self.data.cols() - 1),
            Shading::Flat => (self.data.rows(), self.data.cols())
        };

        Bounds::new(
            Point(0.0, 0.0), 
            Point(cols as f32, rows as f32)
        )
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        if self.is_stale {
            self.is_stale = false;

            let mut xy = Vec::<[f32; 2]>::new();
            let (rows, cols) = (self.data.rows(), self.data.cols());

            for j in 0..rows {
                for i in 0..cols {
                    xy.push([i as f32, j as f32]);
                }
            }

            self.xy = Tensor::from(xy);
            self.norm.set_bounds(&self.data);
        }

        match self.shading {
            Shading::Gouraud => {
                self.draw_gouraud_shading(renderer, to_canvas, style)
            },
            Shading::Flat => {
                self.draw_solid_shading(renderer, to_canvas, style)
            }
        }
    }
}

impl Artist<Data> for GridColor {
    type Opt = GridColorOpt;

    fn config(&mut self, _cfg: &ConfigArc) {
        // self.style = PathStyle::from_config(cfg, "color_grid");
    }

    fn opt(&mut self, view: ArtistView<Data, GridColor>) -> Self::Opt {
        GridColorOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(GridColorOpt, GridColor);

impl GridColorOpt {
    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 2, "ColorGrid data must be rank-2. Shape={:?}", data.shape());

        self.write(|artist| {
            artist.data = data;
            artist.is_stale = true;
        });

        self
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
            self.write(|artist| {
            artist.norm = norm.into();
            artist.is_stale = true;
        });

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.write(|artist| {
        artist.color_map = cmap.into();
        artist.is_stale = true;
    });

    self
}

    pub fn shading(&mut self, shading: Shading) -> &mut Self {
        self.write(|artist| {
        artist.shading = shading;
        artist.is_stale = true;
    });

    self
}
}
