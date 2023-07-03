use essay_plot_base::{Canvas, Bounds, Point, Clip, PathOpt, Affine2d};
use essay_tensor::{Tensor};

use crate::{frame::Data, artist::{Norms, Norm}};

use super::{Artist, ColorMap, ColorMaps};

pub struct Image {
    data: Tensor,
    norm: Norm,
    color_map: ColorMap,
    image: Tensor<u32>,
}

impl Image {
    pub fn new(data: impl Into<Tensor>) -> Self {
        let data : Tensor = data.into();

        assert!(data.rank() == 2, "image requires 2d value {:?}", data.shape().as_slice());

        Self {
            data,
            norm: Norm::from(Norms::Linear),
            color_map: ColorMaps::Default.into(), // ColorMaps::Default.into(),
            image: Tensor::empty()
        }
    }

    pub fn norm(&mut self, norm: impl Into<Norm>) -> &mut Self {
        self.norm = norm.into();

        self
    }

    pub fn cmap(&mut self, cmap: impl Into<ColorMap>) -> &mut Self {
        self.color_map = cmap.into();

        self
    }

    pub(crate) fn set_data(&mut self, data: Tensor) {
        assert!(data.rank() == 2, "image requires 2d value {:?}", data.shape().as_slice());

        self.data = data;
    }
}

impl Artist<Data> for Image {
    fn update(&mut self, _canvas: &Canvas) {
        self.norm.set_bounds(&self.data);
    }
    
    fn get_extent(&mut self) -> Bounds<Data> {
        let (rows, cols) = (self.data.rows(), self.data.cols());

        Bounds::new(
            Point(0.0, 0.),
            Point(cols as f32, rows as f32),
        )
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn essay_plot_base::driver::Renderer,
        to_canvas: &Affine2d,
        clip: &Clip,
        _style: &dyn PathOpt,
    ) {
        //let to_canvas = to_canvas.translate(0., self.).scale(1., -1.);
        let extent = self.get_extent();
        let bounds = Bounds::new(
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
        let colors = Tensor::from(colors).reshape([self.data.rows(), 4 * self.data.cols()]);
    
        renderer.draw_image(&bounds, &colors, clip).unwrap();
    }
}
