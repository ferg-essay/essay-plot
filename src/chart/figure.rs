use essay_graphics::layout::Page;
use essay_graphics::wgpu::{WgpuBackend, WgpuHardcopy};

use essay_graphics::api::{
    renderer::Backend,
    Bounds,
};

use crate::chart::Chart; // , frame::{Layout, LayoutArc}};

use super::chart::ChartBuilder;

pub struct Figure {
    size: (f32, f32),
    dpi: f32,

    backend: Box<dyn Backend>,
    charts: ChartBuilder,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            backend: Box::new(WgpuBackend::new()),
            charts: ChartBuilder::new(Page::new()),

            size: (6.4, 4.8),
            dpi: 200.,
        }
    }

    pub fn chart(&mut self, pos: impl Into<Bounds<Page>>) -> Chart {
        self.charts.chart(pos)
    }

    pub fn show(self) {
        let layout = self.charts.into_page();
        let mut device = self.backend;

        device.main_loop(Box::new(layout)).unwrap();
    }

    #[inline]
    pub fn get_width(&self) -> f32 {
        self.size.0
    }

    #[inline]
    pub fn get_height(&self) -> f32 {
        self.size.1
    }

    #[inline]
    pub fn get_dpi(&self) -> f32 {
        self.dpi
    }

    pub fn save(&mut self, path: impl AsRef<std::path::Path>, dpi: f32) {
        let width = self.get_width() * dpi;
        let height = self.get_height() * dpi;
        let mut hardcopy = WgpuHardcopy::new(width as u32, height as u32);
        hardcopy.scale_factor(dpi / 100.);

        let surface = hardcopy.add_surface();
        hardcopy.draw(self.charts.get_page_mut());
        hardcopy.save(surface, path, dpi as usize);
    }
}
