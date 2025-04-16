use essay_graphics::api::{
    renderer::{Renderer, Result}, 
    Bounds, Mesh2dColor, PathOpt, Point
};
use essay_tensor::tensor::Tensor;

use crate::{
    artist::{Norm, Norms}, 
    chart::{Data, LegendHandler}, 
    config::ConfigArc, 
    data_artist_option_struct, 
    palette::{ColorMap, EssayColors}, 
    transform::ToCanvas
};

use super::{Artist, ArtistDraw, ArtistView};

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
        ui: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        _style: &dyn PathOpt,
    ) -> Result<()> {
        let xy = to_canvas.transform_tensor(&self.xy);

        let norm = self.data.normalize_unit();
        
        let cmap = &self.color_map;

        let (rows, cols) = (norm.rows(), norm.cols());

        let j_stride = cols + 1;

        let mut mesh = Mesh2dColor::new();

        for j in 0..rows {
            for i in 0..cols {
                let index = j * j_stride + i;
                let x00 = xy[(index, 0)];
                let y00 = xy[(index, 1)];
                let c00 = cmap.map(norm[(j, i)]);

                let index = j * j_stride + i + 1;
                let x01 = xy[(index, 0)];
                let y01 = xy[(index, 1)];

                let index = (j + 1) * j_stride + i;
                let x10 = xy[(index, 0)];
                let y10 = xy[(index, 1)];

                let index = (j + 1) * j_stride + i + 1;
                let x11 = xy[(index, 0)];
                let y11 = xy[(index, 1)];

                mesh.triangle(
                    ([x00, y00], c00),
                    ([x01, y01], c00),
                    ([x11, y11], c00),
                );

                mesh.triangle(
                    ([x00, y00], c00),
                    ([x11, y11], c00),
                    ([x10, y10], c00),
                );
            }
        }

        ui.draw_mesh2d_color(&mesh)
    }

    fn draw_gouraud_shading(
        &mut self, 
        ui: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        _style: &dyn PathOpt,
    ) -> Result<()> {
        let xy = to_canvas.transform_tensor(&self.xy);

        let norm = self.data.normalize_unit();
        
        let cmap = &self.color_map;

        let (rows, cols) = (norm.rows(), norm.cols());

        let j_stride = cols + 1;

        let mut mesh = Mesh2dColor::new();

        for j in 0..rows - 1 {
            for i in 0..cols - 1 {
                let index = j * j_stride + i;
                let x00 = xy[(index, 0)];
                let y00 = xy[(index, 1)];
                let c00 = cmap.map(norm[(j, i)]);
                
                let index = j * j_stride + i + 1;
                let x01 = xy[(index, 0)];
                let y01 = xy[(index, 1)];
                let c01 = cmap.map(norm[(j, i + 1)]);

                let index = (j + 1) * j_stride + i;
                let x10 = xy[(index, 0)];
                let y10 = xy[(index, 1)];
                let c10 = cmap.map(norm[(j + 1, i)]);

                let index = (j + 1) * j_stride + i + 1;
                let x11 = xy[(index, 0)];
                let y11 = xy[(index, 1)];
                let c11 = cmap.map(norm[(j + 1, i + 1)]);

                mesh.triangle(
                    ([x00, y00], c00),
                    ([x01, y01], c01),
                    ([x11, y11], c11),
                );

                mesh.triangle(
                    ([x00, y00], c00),
                    ([x11, y11], c11),
                    ([x10, y10], c10),
                );
            }
        }

        ui.draw_mesh2d_color(&mesh)
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
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        if self.is_stale {
            self.is_stale = false;

            let mut xy = Vec::<[f32; 2]>::new();
            let (rows, cols) = (self.data.rows(), self.data.cols());

            for j in 0..rows + 1 {
                for i in 0..cols + 1 {
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
