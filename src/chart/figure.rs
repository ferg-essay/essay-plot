use essay_graphics::layout::{Page, PageBuilder};
use essay_graphics::wgpu::{WgpuBackend, WgpuHardcopy};

use essay_graphics::api::renderer::Backend;

use crate::chart::Chart; // , frame::{Layout, LayoutArc}};

use super::ConfigArc;

pub struct Figure {
    size: (f32, f32),
    dpi: f32,

    backend: Box<dyn Backend>,

    config: ConfigArc,
    page: PageBuilder,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            backend: Box::new(WgpuBackend::new()),
            config: ConfigArc::default(),
            page: Page::builder(),

            size: (6.4, 4.8),
            dpi: 200.,
        }
    }

    pub fn chart(&mut self) -> Chart {
        let chart = Chart::new(&self.config);

        self.page.view(chart.view().clone());

        chart
    }

    pub fn horizontal(&mut self) -> SubFigure {
        SubFigure {
            config: &self.config,
            sub_page: self.page.horizontal(),
        }
    }

    pub fn vertical(&mut self) -> SubFigure {
        SubFigure {
            config: &self.config,
            sub_page: self.page.vertical(),
        }
    }

    pub fn show(self) {
        let layout = self.page.build();
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
        hardcopy.draw(&mut self.page.build());
        hardcopy.save(surface, path, dpi as usize);
    }
}

pub struct SubFigure<'a> {
    config: &'a ConfigArc,
    sub_page: &'a mut PageBuilder,
}

impl SubFigure<'_> {
    pub fn chart(&mut self) -> Chart {
        let chart = Chart::new(&self.config);

        self.sub_page.view(chart.view().clone());

        chart
    }

    pub fn horizontal(&mut self) -> SubFigure {
        SubFigure {
            config: &self.config,
            sub_page: self.sub_page.horizontal(),
        }
    }

    pub fn vertical(&mut self) -> SubFigure {
        SubFigure {
            config: &self.config,
            sub_page: self.sub_page.vertical(),
        }
    }
}
