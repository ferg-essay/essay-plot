use core::fmt;

use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, Bounds, Path, PathOpt, Point
};

use crate::{
    chart::{Data, LegendHandler}, 
    config::{ConfigArc, PathStyle},
    data_artist_option_struct, path_style_options
};

use super::{
    Artist, ArtistDraw, ArtistView, ToCanvas
};

pub struct HorizontalLine {
    x_min: f32,
    x_max: f32,
    y: f32,

    style: PathStyle,

    is_visible: bool,
    z_order: f32,

    pos: Bounds<Canvas>,
    is_stale: bool,
}

impl HorizontalLine {
    pub fn new(x_min: f32, x_max: f32, y: f32) -> Self {
        assert!(0. <= x_min && x_min <= 1.);
        assert!(0. <= x_max && x_max <= 1.);

        Self {
            x_min,
            x_max,
            y,

            style: PathStyle::new(),

            is_visible: true,
            z_order: 0.,

            pos: Bounds::none(),
            is_stale: false,
        }
    }
}

impl ArtistDraw<Data> for HorizontalLine {
    fn bounds(&mut self) -> Bounds<Data> {
        Bounds::none()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer, 
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        if ! self.is_visible {
            return Ok(());
        }

        let y = to_canvas.transform_point(Point(0., self.y)).y();

        let pos = self.pos.clone();
        let path = Path::move_to(
            pos.xmin() + self.x_min * pos.width(),
            y
        ).line_to(
            pos.xmin() + self.x_max * pos.width(),
            y
        ).to_path();

        let style = self.style.push(style);

        renderer.draw_path(&path, &style)?;

        Ok(())
    }
}

impl Artist<Data> for HorizontalLine {
    type Opt = HorizontalLineOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.style = PathStyle::from_config(cfg, "span");

        // self.style.line_style(":");
    }

    fn opt(&mut self, view: ArtistView<Data, HorizontalLine>) -> Self::Opt {
        HorizontalLineOpt::new(view)
    }
    
    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(HorizontalLineOpt, HorizontalLine);

impl HorizontalLineOpt {
    path_style_options!(style);

    pub fn visible(&mut self, visible: bool) -> &mut Self {
        self.write(|artist| {
            artist.is_visible = visible;
            artist.is_stale = true;
        });

        self
    }

    pub fn z_order(&mut self, order: f32) -> &mut Self {
        self.write(|artist| {
            artist.z_order = order;
            artist.is_stale = true;
        });

        self
    }
}

//impl PathStyleArtist for Lines2d {
//    fn style_mut(&mut self) -> &mut PathStyle {
//        &mut self.style
//    }
//}

impl fmt::Debug for HorizontalLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HorizontalLine[{:?}]", self.y)
    }
}
