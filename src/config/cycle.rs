use essay_graphics::api::{path_opt::Hatch, CapStyle, Color, JoinStyle, LineStyle, PathOpt, TextureId};

use crate::{palette::Palette, config::Config};


pub struct StyleCycle {
    colors: Option<Palette>,
    fill_colors: Option<Palette>,
    edge_colors: Option<Palette>,
    line_widths: Vec<Option<f32>>,
    line_styles: Vec<Option<LineStyle>>,
}

impl StyleCycle {
    pub fn new() -> Self {
        Self {
            colors: None,
            fill_colors: None,
            edge_colors: None,
            line_widths: Vec::new(),
            line_styles: Vec::new(),
        }
    }
    pub fn push<'a>(
        &'a self, 
        prev: &'a dyn PathOpt, 
        index: usize,
        n: usize,
    ) -> PropCycleChain<'a> {
        PropCycleChain::new(self, prev, index, n)
    }

    pub fn fill_colors(&mut self, colors: impl Into<Palette>) -> &mut Self {
        self.fill_colors = Some(colors.into());

        self
    }

    pub fn edge_colors(&mut self, colors: impl Into<Palette>) -> &mut Self {
        self.edge_colors = Some(colors.into());

        self
    }

    pub fn colors(&mut self, colors: impl Into<Palette>) -> &mut Self {
        self.colors = Some(colors.into());

        self
    }

    pub fn line_styles(&mut self, line_styles: impl Into<LineStyles>) -> &mut Self {
        let cycle: LineStyles = line_styles.into();

        self.line_styles = cycle.cycle;

        self
    }

    pub fn line_widths(&mut self, widths: &[f32]) -> &mut Self {
        let mut vec: Vec<Option<f32>> = Vec::new();

        for width in widths {
            vec.push(Some(*width));
        }

        self.line_widths = vec;

        self
    }

    fn get_fill_color(&self, i: usize, n: usize) -> Option<Color> {
        if let Some(cycle) = &self.fill_colors {
            Some(cycle.color(i, n))
        } else {
            None
        }
    }

    fn get_edge_color(&self, i: usize, n: usize) -> Option<Color> {
        if let Some(cycle) = &self.edge_colors {
            Some(cycle.color(i, n))
        } else {
            None
        }
    }

    fn get_color(&self, i: usize, n: usize) -> Option<Color> {
        if let Some(cycle) = &self.colors {
            Some(cycle.color(i, n))
        } else {
            None
        }
    }

    fn is_line_width_set(&self) -> bool {
        self.line_widths.len() > 0
    }

    fn get_line_width(&self, index: usize) -> &Option<f32> {
        let len = self.line_widths.len();
        assert!(len > 0);
        
        &self.line_widths[index % self.line_widths.len()]
    }

    fn is_line_style_set(&self) -> bool {
        self.line_styles.len() > 0
    }

    fn get_line_style(&self, index: usize) -> &Option<LineStyle> {
        let len = self.line_styles.len();
        assert!(len > 0);
        
        &self.line_styles[index % self.line_styles.len()]
    }

    pub(crate) fn from_config(cfg: &Config, prefix: &str) -> StyleCycle {
        let mut cycle = StyleCycle::new();

        if let Some(palette) = cfg.get_as_type::<Palette>(prefix, "colors")
            .or_else(|| Some(Palette::default())) {
            cycle.colors(palette);
        };

        cycle
    }
}

impl From<Palette> for StyleCycle {
    fn from(value: Palette) -> Self {
        let mut style_cycle = StyleCycle::new();

        style_cycle.colors = Some(value);

        style_cycle
    }
}

pub struct PropCycleChain<'a> {
    cycle: &'a StyleCycle,
    prev: &'a dyn PathOpt,
    index: usize,
    n: usize,
}

impl<'a> PropCycleChain<'a> {
    fn new(
        cycle: &'a StyleCycle,
        prev: &'a dyn PathOpt,
        index: usize,
        n: usize,
    ) -> Self {
        assert!(n > 0);

        Self {
            prev,
            cycle,
            index,
            n,
        }
    }
}

impl PathOpt for PropCycleChain<'_> {
    fn get_face_color(&self) -> Option<Color> {
        self.cycle.get_fill_color(self.index, self.n)
            .or(self.cycle.get_color(self.index, self.n))
            .or(self.prev.get_face_color())
    }

    fn get_edge_color(&self) -> Option<Color> {
        self.cycle.get_edge_color(self.index, self.n)
            .or(self.cycle.get_color(self.index, self.n))
            .or(self.prev.get_edge_color())
    }

    fn get_line_style(&self) -> Option<LineStyle> {
        if self.cycle.is_line_style_set() {
            self.cycle.get_line_style(self.index).clone()
        } else {
            self.prev.get_line_style()
        }
    }

    fn get_line_width(&self) -> Option<f32> {
        if self.cycle.is_line_width_set() {
            self.cycle.get_line_width(self.index).clone()
        } else {
            self.prev.get_line_width()
        }
    }

    fn get_join_style(&self) -> Option<JoinStyle> {
        self.prev.get_join_style()
    }

    fn get_cap_style(&self) -> Option<CapStyle> {
        self.prev.get_cap_style()
    }

    fn get_alpha(&self) -> Option<f32> {
        self.prev.get_alpha()
    }

    fn get_texture(&self) -> Option<TextureId> {
        self.prev.get_texture()
    }

    fn get_hatch(&self) -> Option<Hatch> {
        self.prev.get_hatch()
    }
}

pub struct LineStyles {
   cycle: Vec<Option<LineStyle>>,
}

impl<const N: usize> From<[LineStyle; N]> for LineStyles {
    fn from(value: [LineStyle; N]) -> Self {
        let mut vec = Vec::new();

        for style in value {
            vec.push(Some(style));
        }

        Self { cycle: vec }
    }
}

impl From<&[LineStyle]> for LineStyles {
    fn from(value: &[LineStyle]) -> Self {
        let mut vec = Vec::new();

        for style in value {
            vec.push(Some(style.clone()));
        }

        Self { cycle: vec }
    }
}

impl<const N: usize> From<[&str; N]> for LineStyles {
    fn from(value: [&str; N]) -> Self {
        let mut vec = Vec::new();

        for name in value {
            vec.push(Some(LineStyle::from(name)));
        }

        Self { cycle: vec }
    }
}

impl From<&[&str]> for LineStyles {
    fn from(value: &[&str]) -> Self {
        let mut vec = Vec::new();

        for name in value {
            vec.push(Some(LineStyle::from(*name)));
        }

        Self { cycle: vec }
    }
}

