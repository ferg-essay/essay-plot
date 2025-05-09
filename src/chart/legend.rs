use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, 
    Bounds, HorizAlign, Path, PathCode, PathOpt, Point, TextStyle, VertAlign
};

use crate::{
    artist::ArtistDraw, 
    config::{Config, PathStyle, StyleCycle}, transform::ToCanvas,
};

use super::data_frame::DataFrame;

pub struct Legend {
    pos: Bounds<Canvas>,
    extent: Bounds<Canvas>,

    handlers: Vec<LegendHandler>,

    path_style: PathStyle,
    text_style: TextStyle,
    style_cycle: StyleCycle,

    glyph_size: f32,
}

impl Legend {
    pub fn new(cfg: &Config) -> Self {
        let mut legend = Self {
            pos: Bounds::zero(),
            extent: Bounds::zero(),

            path_style: PathStyle::new(),
            text_style: TextStyle::new(),
            style_cycle: StyleCycle::from_config(cfg, "frame.cycle"),

            handlers: Vec::new(),

            glyph_size: 0.,
        };

        legend.path_style.face_color("white").edge_color("#b0b0b0").line_width(1.);
        legend.text_style.valign(VertAlign::Top);
        legend.text_style.halign(HorizAlign::Left);

        legend
    }

    pub fn set_pos(&mut self, pos: Bounds<Canvas>) {
        self.pos = pos;
    }

    pub(crate) fn update_handlers(&mut self, data: &mut DataFrame) {
        let handlers = data.get_handlers();
        self.handlers = handlers;
    }

    pub(super) fn resize(&mut self, renderer: &mut dyn Renderer, pos: &Bounds<Canvas>) {
        let font_size = match self.text_style.get_size() {
            Some(size) => *size,
            None => 10.,
        };

        self.glyph_size = renderer.to_px(font_size);
        self.pos = pos.clone();
    }
}

impl ArtistDraw<Canvas> for Legend {
    fn bounds(&mut self) -> Bounds<Canvas> {
        self.extent.clone()
    }

    fn draw(
        &mut self, 
        renderer: &mut dyn Renderer,
        _to_canvas: &ToCanvas<Canvas>,
        style: &dyn PathOpt,
    ) -> Result<()> {
        if self.handlers.len() == 0 {
            return Ok(());
        }

        let n_chars = match self.handlers.iter().map(|h| h.get_label().len()).max() {
            Some(n_chars) => n_chars,
            None => 0,
        };

        let dh = self.glyph_size;
        // TODO: functions for label width
        let label_width = self.glyph_size * 0.5 * n_chars as f32;

        let pos = &self.pos;
        let rect_width = 80.;
        //let rect_height = dh;
        let pad_x = 15.;
        let pad_y = 5.;
        let margin = 20.;

        let w = label_width + pad_x + rect_width + 2. * margin;
        let h = dh * self.handlers.len() as f32 + 2. * margin;

        let x0 = pos.x0() + margin;
        let y0 = pos.y0() - margin;

        let path = Path::<Canvas>::new(vec![
            PathCode::MoveTo(Point(x0, y0)),
            PathCode::LineTo(Point(x0, y0 - h)),
            PathCode::LineTo(Point(x0 + w, y0 - h)),
            PathCode::ClosePoly(Point(x0 + w, y0)),
        ]);

        renderer.draw_path(&path, &self.path_style)?;

        let x_symbol = x0 + margin;
        let x_t = x_symbol + rect_width + pad_x;

        let n = self.handlers.len();

        for (i, handler) in self.handlers.iter().enumerate() {
            let y = y0 - i as f32 * (dh + pad_y) - margin;
            renderer.draw_text(
                Point(x_t, y),
                handler.get_label(),
                0.,
                style,
                &self.text_style,
            )?;

            let rect = Bounds::<Canvas>::new(
                Point(x_symbol, y), 
                Point(x_symbol + rect_width, y - dh)
             );

            let style = self.style_cycle.push(style, i, n);
             //let color = self.cycle.
             
             handler.draw(renderer, &style, &rect)?;
        }

        Ok(())
    }
}

pub struct LegendHandler {
    label: String,
    draw: Box<dyn Fn(&mut dyn Renderer, &dyn PathOpt, &Bounds<Canvas>) -> Result<()> + Send>,
}

impl LegendHandler {
    pub fn new(
        label: String,
        draw: impl Fn(&mut dyn Renderer, &dyn PathOpt, &Bounds<Canvas>) -> Result<()> + Send + 'static
    ) -> Self {
        Self {
            label,
            draw: Box::new(draw),
        }
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn draw(
        &self, 
        renderer: &mut dyn Renderer, 
        style: &dyn PathOpt,
        rect: &Bounds<Canvas>
    ) -> Result<()> {
        (self.draw)(renderer, style, rect)
    }
}