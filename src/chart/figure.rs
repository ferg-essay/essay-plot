use essay_graphics::layout::{Page, PageBuilder};
use essay_graphics::wgpu::{WgpuBackend, WgpuHardcopy};

use essay_graphics::api::renderer::Backend;

use crate::chart::Chart;
use crate::config::ConfigArc;

use super::polar_chart::PolarChart; // , frame::{Layout, LayoutArc}};

pub struct Figure {
    size: (f32, f32),
    dpi: f32,

    backend: Box<dyn Backend>,

    config: ConfigArc,
    page: Option<Page>,
}

impl Figure {
    pub fn new() -> Self {
        Self {
            backend: Box::new(WgpuBackend::new()),
            config: ConfigArc::default(),
            page: None,

            size: (6.4, 4.8),
            dpi: 200.,
        }
    }

    pub fn chart(&mut self) -> Chart {
        let chart = Chart::new(&self.config);

        self.page = Some(Page::new(chart.clone()));

        chart
    }

    pub fn polar(&mut self) -> PolarChart {
        let chart = PolarChart::new(&self.config);

        self.page = Some(Page::new(chart.clone()));

        chart
    }

    pub fn multichart<R>(&mut self, f: impl FnOnce(&mut SubFigure) -> R) -> R {
        let mut page_builder = PageBuilder::new();

        let mut builder = SubFigure {
            config: &self.config,
            sub_page: &mut page_builder,
        };
        
        let result = (f)(&mut builder);

        self.page = Some(page_builder.build());

        result
    }

    pub fn show(self) {
        let mut own = self;
        if let Some(page) = own.page.take() {
            own.backend.main_loop(Box::new(page)).unwrap();
        }
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
        if let Some(page) = &mut self.page {
            hardcopy.draw(page);
        }
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

        self.sub_page.view(chart.clone());

        chart
    }

    pub fn horizontal<R>(&mut self, f: impl FnOnce(&mut SubFigure) -> R) -> R {
        self.sub_page.horizontal(|page_builder| {
            let mut sub = SubFigure {
                config: &self.config,
                sub_page: page_builder,
            };

            (f)(&mut sub)
        })
    }

    pub fn vertical<R>(&mut self, f: impl FnOnce(&mut SubFigure) -> R) -> R {
        self.sub_page.vertical(|page_builder| {
            let mut sub = SubFigure {
                config: &self.config,
                sub_page: page_builder,
            };

            (f)(&mut sub)
        })
    }
}
