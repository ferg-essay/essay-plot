use std::f32::consts::PI;

use essay_graphics::{
    api::{
        renderer::{Result, Canvas, Drawable, Renderer}, 
        Affine2d, Bounds, Color, PathOpt, Point, VertAlign 
    }, 
    layout::View
};

use crate::{
    artist::{
        patch::CanvasPatch, paths, ArtistDraw, Colorbar, TextCanvas, ToCanvas
    }, palette::Palette, config::{Config, ConfigArc, PathStyle}
};

use super::{
    axis::{Axis, AxisTicks, XAxis, YAxis}, 
    data_frame::DataFrame, 
    legend::Legend
};

pub struct CartesianFrame {
    pos: Bounds<Canvas>,

    config: ConfigArc,

    to_canvas: Affine2d,

    margins: FrameMargins,

    _is_share_x: bool,
    _is_share_y: bool,

    path_style: PathStyle,

    data: DataFrame,

    title: TextCanvas,

    bottom: BottomFrame,
    left: LeftFrame,
    top: TopFrame,
    right: RightFrame,

    //is_frame_visible: bool,
    legend: Legend,

    //is_stale: bool, 
    //aspect_ratio: Option<f32>,
    //box_aspect_ratio: Option<f32>,
    // is_visible
    // axis_locator
    // is_axis_below
    // label
    // adjustable (Box vs Data)
    // is_snap
    // transform
    // xbound (min, max)
    // xmargin
    // xscale - linear, log, symlog, logit
    // xticks - sets ticks and labels
    // ybound
    // ylabel
    // ylim
    // ymargin
    // yscale
    // yticks
    // zorder
}

impl CartesianFrame {
    pub(crate) fn new(cfg: &ConfigArc) -> Self {
        Self {
            config: cfg.clone(),

            pos: Bounds::none(),

            data: DataFrame::new(cfg, "frame"),

            title: TextCanvas::new(),

            margins: FrameMargins::new(cfg),

            bottom: BottomFrame::new(cfg),
            left: LeftFrame::new(cfg),
            top: TopFrame::new(),
            right: RightFrame::new(),

            path_style: PathStyle::default(),

            to_canvas: Affine2d::eye(),

            legend: Legend::new(cfg),

            _is_share_x: false,
            _is_share_y: false,
            //is_frame_visible: true,
            //aspect_ratio: None,
            //box_aspect_ratio: None,
        }
    }

    pub(crate) fn config(&self) -> &ConfigArc {
        &self.config
    }

    pub(crate) fn data_mut(&mut self) -> &mut DataFrame {
        &mut self.data
    }

    pub(crate) fn get_axis_mut(&mut self, artist: FrameArtist) -> &mut Axis {
        match artist {
            FrameArtist::X => self.bottom.axis_mut(),
            FrameArtist::Y => self.left.axis_mut(),

            _ => panic!("Invalid axis {:?}", artist)
        }
    }

    pub(crate) fn get_ticks_mut(&mut self, artist: FrameArtist) -> &mut AxisTicks {
        match artist {
            FrameArtist::XMajor => self.bottom.axis_mut().major_mut(),
            FrameArtist::XMinor => self.bottom.axis_mut().minor_mut(),
            FrameArtist::YMajor => self.left.axis_mut().major_mut(),
            FrameArtist::YMinor => self.left.axis_mut().minor_mut(),

            _ => panic!("Invalid axis-texts {:?}", artist)
        }
    }

    pub(crate) fn colorbar(&mut self) {
        self.right.colorbar();
    }

    pub(crate) fn color_cycle(&mut self, cycle: impl Into<Palette>) {
        self.data.color_cycle(cycle);
    }

    fn resize(&mut self, renderer: &mut dyn Renderer) {
        let pos = renderer.pos();

        let pos = Bounds::from([
            [pos.xmin() + pos.width() * self.margins.left,
            pos.ymin() + pos.height() * self.margins.top],
            [pos.xmin() + pos.width() * self.margins.right,
            pos.ymin() + pos.height() * self.margins.bottom]
        ]);
    
        self.pos = pos.clone();
    
        let title = self.title.bounds();
    
        // title exists outside the pos bounds
        self.title.update_pos(
            renderer,
            Bounds::from([
                [pos.xmin(), pos.ymax()], 
                [pos.xmax(), pos.ymax() + title.height()]
            ])
        );
    
        let pos_data = Bounds::<Canvas>::new(
            Point(pos.xmin(), pos.ymin()), 
            Point(pos.xmax(), pos.ymax()),
        );
    
        self.data.update_pos(renderer, &pos_data);
    
        let pos_data = self.data.get_pos().clone();
    
        let pos_top = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmax(), pos_data.ymax()),
        );
        self.top.resize(renderer, pos_top);
    
        let pos_right = Bounds::<Canvas>::new(
            Point(pos_data.xmax(), pos_data.ymin()),
            Point(pos_data.xmax(), pos_data.ymax()),
        );
        self.right.resize(renderer, &pos_right);
    
        let pos_legend = Bounds::<Canvas>::new(
            Point(pos_data.xmin(), pos_data.ymax()),
            Point(pos_data.xmin(), pos_data.ymax()),
        );
        self.legend.resize(renderer, &pos_legend);
    
        // TODO:
        self.bottom.update_axis(&self.data);
        self.bottom.resize(renderer, pos_data);
    
        self.left.update_axis(&self.data);
        self.left.resize(renderer, pos_data);
    
        self.top.resize(renderer, pos_data);
        self.right.resize(renderer, &pos_data);
    
        self.legend.update_handlers(&mut self.data);
    }
}

impl Drawable for CartesianFrame {
    fn draw(&mut self, renderer: &mut dyn Renderer) -> Result<()> {
        if self.pos != renderer.pos() {
            self.resize(renderer);
        }

        let frame_to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.to_canvas.clone()
        );

        let to_canvas = ToCanvas::new(
            self.pos.clone(), 
            self.data.get_canvas_transform().clone()
        );

        self.title.draw(renderer, &to_canvas, &self.path_style)?;

        self.bottom.draw(renderer, &self.data, &frame_to_canvas, &self.path_style)?;
        self.left.draw(renderer, &self.data, &frame_to_canvas, &self.path_style)?;

        self.top.draw(renderer, &frame_to_canvas, &self.path_style)?;
        self.right.draw(renderer,  &frame_to_canvas, &self.path_style)?;

        // self.data.draw(renderer, &to_canvas, &self.path_style)?;

        renderer.draw_with_closure(self.data.get_pos(), Box::new(|r| {
            self.data.draw(r, &to_canvas, &self.path_style)
        }))?;

        self.legend.draw(renderer, &frame_to_canvas, &self.path_style)?;

        Ok(())
    }
}

impl FrameWithTextArtist for CartesianFrame {
    fn get_text_mut(&mut self, artist: FrameArtist) -> &mut TextCanvas {
        match artist {
            FrameArtist::Title => &mut self.title,
            FrameArtist::XLabel => &mut self.bottom.title,
            FrameArtist::YLabel => &mut self.left.title,

            _ => panic!("Invalid text {:?}", artist)
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FrameArtist {
    Title,
    X,
    Y,
    XMajor,
    XMinor,
    YMajor,
    YMinor,
    XLabel,
    YLabel,
}

pub struct FrameSizes {
    label_pad: f32,
}

impl FrameSizes {
    fn new(cfg: &Config) -> Self {
        Self {
            label_pad: match cfg.get_as_type("frame", "label_pad") {
                Some(pad) => pad,
                None => 0.,
            },
        }
    }
}

//
// Top Frame
//

pub struct TopFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<CanvasPatch>,
}

impl TopFrame {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(0., 20.)),
            pos: Bounds::none(),
            spine: Some(CanvasPatch::new(paths::line(Point(0., 0.), Point(1., 0.)))),
        }
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymin()),
                Point(pos.xmax(), pos.ymin() + 1.),
            ))
        }
    }

    fn resize(&mut self, _renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.set_pos(pos);
    }
}

impl ArtistDraw<Canvas> for TopFrame {
    fn bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, style)?;
        }

        Ok(())
        
    }
}

//
// Bottom frame
//

pub struct BottomFrame {
    sizes: FrameSizes,

    x_axis: XAxis,

    title: TextCanvas,
}

impl BottomFrame {
    pub fn new(cfg: &Config) -> Self {
        let mut frame = Self {
            sizes: FrameSizes::new(cfg),

            x_axis: XAxis::new(cfg, "x_axis"),

            title: TextCanvas::new(),
        };

        frame.title.text_style_mut().valign(VertAlign::Top);

        frame
    }

    pub fn update_axis(&mut self, data: &DataFrame) {
        self.x_axis.update_axis(data); 
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataFrame,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let mut y = self.x_axis.draw(renderer, data, to_canvas, style)?;
        y -= renderer.to_px(self.sizes.label_pad);

        self.title.update_pos(renderer, Bounds::new(
            Point(data.get_pos().xmin(), y),
            Point(data.get_pos().xmax(), y),
        ));
    
        self.title.draw(renderer, to_canvas, style)
    }

    fn _title(&mut self, text: &str) -> &mut TextCanvas {
        self.title.label(text)
    }

    fn resize(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.title.update_pos(renderer, pos);
        self.x_axis.resize(renderer, pos);
    }

    fn axis_mut(&mut self) -> &mut Axis {
        self.x_axis.axis_mut()
    }
}

//
// Left frame
//

pub struct LeftFrame {
    sizes: FrameSizes,

    y_axis: YAxis,

    title: TextCanvas,
}

impl LeftFrame {
    pub fn new(cfg: &Config) -> Self {
        let mut label = TextCanvas::new();
        label.angle(PI / 2.);

        let mut frame = Self {
            sizes: FrameSizes::new(cfg),

            y_axis: YAxis::new(cfg, "y_axis"),

            title: label,
        };

        frame.title.text_style_mut().valign(VertAlign::BaselineBottom);

        frame
    }

    pub fn update_axis(&mut self, data: &DataFrame) {
        self.y_axis.update_axis(data);
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        data: &DataFrame,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        let mut x = self.y_axis.draw(renderer, data, to_canvas, style)?;
        x -= renderer.to_px(self.sizes.label_pad);

        self.title.update_pos(renderer, Bounds::new(
            Point(x, data.get_pos().ymid()),
            Point(x, data.get_pos().ymid()),
        ));

        self.title.draw(renderer, to_canvas, style)
    }

    fn _label(&mut self, text: &str) -> &mut TextCanvas {
        self.title.label(text)
    }

    fn resize(&mut self, renderer: &mut dyn Renderer, pos: Bounds<Canvas>) {
        self.title.update_pos(renderer, pos);
        self.y_axis.update(renderer, pos);
    }

    fn axis_mut(&mut self) -> &mut Axis {
        self.y_axis.axis_mut()
    }
}

//
// Right frame
//

pub struct RightFrame {
    bounds: Bounds<Canvas>,
    pos: Bounds<Canvas>,
    spine: Option<CanvasPatch>,
    colorbar: Option<Colorbar>,
}

impl RightFrame {
    pub fn new() -> Self {
        Self {
            bounds: Bounds::new(Point(0., 0.), Point(20., 0.)),
            pos: Bounds::none(),
            spine: Some(CanvasPatch::new(paths::line(Point(0., 0.), Point(0., 1.)))),
            colorbar: None,
        }
    }

    pub fn set_pos(&mut self, pos: &Bounds<Canvas>) {
        self.pos = pos.clone();

        if let Some(spine) = &mut self.spine {
            spine.set_pos(Bounds::new(
                Point(pos.xmin(), pos.ymin()),
                Point(pos.xmin() + 1., pos.ymax()),
            ))
        }

        if let Some(colorbar) = &mut self.colorbar {
            colorbar.set_pos(Bounds::new(
                Point(pos.xmin() + 40., pos.ymin()),
                Point(pos.xmin() + 80., pos.ymax()),
            ))
        }
    }

    pub fn colorbar(&mut self) {
        self.colorbar = Some(Colorbar::new());
    }

    fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        self.set_pos(pos);

        if let Some(colorbar) = &mut self.colorbar {
            colorbar.resize(renderer, pos);
        }
    }
}

impl ArtistDraw<Canvas> for RightFrame {
    fn bounds(&mut self) -> Bounds<Canvas> {
        self.bounds.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        to_canvas: &ToCanvas,
        style: &dyn PathOpt,
    ) -> Result<()> {
        if let Some(patch) = &mut self.spine {
            patch.draw(renderer, to_canvas, style)?;
        }

        if let Some(colorbar) = &mut self.colorbar {
            colorbar.draw(renderer, to_canvas, style)?;
        }

        Ok(())
    }
}

pub struct FrameTextOpt<F: FrameWithTextArtist> {
    view: View<F>,
    artist: FrameArtist,
}

impl<F: FrameWithTextArtist> FrameTextOpt<F> {
    pub(crate) fn new(view: View<F>, artist: FrameArtist) -> Self {
        Self {
            view,
            artist,
        }
    }

    fn write(&mut self, fun: impl FnOnce(&mut TextCanvas)) {
        self.view.write(|frame| {
            fun(frame.get_text_mut(self.artist))
        })
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.write(|text| { text.label(label); });
        self
    }

    pub fn color(&mut self, color: impl Into<Color>) -> &mut Self {
        self.write(|text| { text.path_style_mut().color(color); });
        self
    }

    pub fn size(&mut self, size: f32) -> &mut Self {
        self.write(|text| { text.text_style_mut().size(size); });
        self
    }
}

pub trait FrameWithTextArtist: Drawable + Send + 'static {
    fn get_text_mut(&mut self, artist: FrameArtist) -> &mut TextCanvas;
}

pub(super) struct FrameMargins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl FrameMargins {
    pub fn new(cfg: &Config) -> Self {
        let bottom = cfg.get_as_type("figure.subplot", "bottom")
            .unwrap_or(0.);
        let top = cfg.get_as_type("figure.subplot", "top")
            .unwrap_or(1.);
        let left = cfg.get_as_type("figure.subplot", "left")
            .unwrap_or(0.);
        let right = cfg.get_as_type("figure.subplot", "right")
            .unwrap_or(1.);

        Self {
            bottom,
            top, 
            left,
            right, 
        }
    }
}

