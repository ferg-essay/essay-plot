use essay_graphics::layout::Layout;
use essay_graphics::wgpu::WgpuBackend;

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
            charts: ChartBuilder::new(Layout::new()),

            size: (6.4, 4.8),
            dpi: 200.,
        }
    }

    pub fn chart(&mut self, pos: impl Into<Bounds<Layout>>) -> Chart {
        self.charts.chart(pos)
    }

    pub fn show(self) {
        let layout = self.charts.into_layout();
        let mut device = self.backend;

        device.main_loop(Box::new(layout)).unwrap();
    }

    pub fn get_width(&self) -> f32 {
        self.size.0
    }

    pub fn get_height(&self) -> f32 {
        self.size.1
    }

    pub fn get_dpi(&self) -> f32 {
        self.dpi
    }

    pub fn save(&mut self, _path: impl AsRef<std::path::Path>, _dpi: f32) {
        todo!();
        /*
        crate::wgpu::draw_hardcopy(
            self.get_width() * dpi,
            self.get_height() * dpi,
            dpi,
            &mut self.layout, 
            path
        );
        */    
    }
}
