use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, Bounds, PathOpt, Point
};
use essay_tensor::tensor::Tensor;

use crate::{
    artist::{Artist, ArtistDraw, Norm, Norms}, chart::{Data, LegendHandler}, config::ConfigArc, data_artist_option_struct, palette::{ColorMap, EssayColors}, transform::ToCanvas
};

use super::ArtistView;

pub struct Image {
    data: Tensor,
    norm: Norm,
    color_map: ColorMap,
    extent: Option<Bounds<Data>>,
}

impl Image {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "image requires 2d value {:?}", data.shape());

        let mut image = Self {
            data,
            norm: Norm::from(Norms::Linear),
            color_map: EssayColors::Default.into(), // ColorMaps::Default.into(),
            extent: None,
        };

        image.update_bounds();

        image
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
        self.norm = norm.into();
        self.update_bounds();

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.color_map = cmap.into();

        self
    }

    fn set_data(&mut self, data: Tensor) {
        self.data = data;
        self.update_bounds();
    }

    fn update_bounds(&mut self) {
        self.norm.set_bounds(&self.data);
    }
}

impl ArtistDraw<Data> for Image {
    fn bounds(&mut self) -> Bounds<Data> {
        match &self.extent {
            Some(extent) => extent.clone(),
            None => {
                let (rows, cols) = (self.data.rows(), self.data.cols());

               Bounds::new(
                    Point(0.0, 0.),
                    Point(cols as f32, rows as f32),
                )
            }
        }
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        _style: &dyn PathOpt,
    ) -> Result<()> {
        //let to_canvas = to_canvas.translate(0., self.).scale(1., -1.);
        let extent = self.bounds();
        let bounds = Bounds::<Canvas>::new(
            to_canvas.transform_point(extent.p0()),
            to_canvas.transform_point(extent.p1()),
        );
    
        let norm = &self.norm;
        let colormap = &self.color_map;
    
        let mut colors = Vec::<u8>::new();
        for v in self.data.iter() {
            let v = norm.norm(*v);
            let color = colormap.map(v);
            colors.push(color.r8());
            colors.push(color.g8());
            colors.push(color.b8());
            colors.push(color.a8());
        }
    
        // todo [width, height, 4]
        let colors = Tensor::from(colors).reshape([self.data.rows(), self.data.cols(), 4]);
    
        // renderer.draw_image(bounds, &colors)
        todo!()
    }
}

impl Artist<Data> for Image {
    type Opt = ImageOpt;

    fn config(&mut self, _cfg: &ConfigArc) {
    }

    fn opt(&mut self, view: ArtistView<Data, Image>) -> Self::Opt {
        ImageOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(ImageOpt, Image);

impl ImageOpt {
    pub fn data(&mut self, data: impl Into<Tensor>) -> &mut Self {
        let data = data.into();
        assert!(data.rank() == 2, "Image data must be rank 2. Shape={:?}", data.shape());

        self.write(|artist| {
            artist.set_data(data);
        });

        self
    }

    pub fn extent(&mut self, extent: impl Into<Bounds<Data>>) -> &mut Self {
        self.write(|artist| {
            artist.extent = Some(extent.into());
        });

        self
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
            self.write(|artist| {
            artist.norm(norm);
        });

        self
    }

    pub fn color_map(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.write(|artist| {
           artist.color_map(cmap);
        });

        self
    }
}
