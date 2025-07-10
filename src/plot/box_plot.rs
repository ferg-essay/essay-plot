use std::cmp::Ordering;

use essay_graphics::api::{
    renderer::{Canvas, Renderer, Result}, Bounds, Color, Path, PathOpt, Point
};
use essay_tensor::tensor::{IntoTensorList, Tensor};

use crate::{
    artist::{
        paths::{self}, Artist, ArtistDraw, ArtistView, Markers, PathCollection, Stale,
    }, 
    chart::{Chart, Data, LegendHandler}, 
    config::{ConfigArc, PathStyle},
    data_artist_option_struct, path_style_options, transform::ToCanvas 
};

pub fn box_plot(
    chart: &mut Chart, 
    data: impl IntoTensorList<f32>, 
) -> BoxPlotOpt {
    let mut vec = Vec::new();
    data.into_list(&mut vec);

    let plot = BoxPlot::new(vec);

    chart.artist(plot)
}

pub struct BoxPlot {
    whisker: f32,
    is_draw_outliers: bool,

    data: Vec<BoxPlotData>,

    style: PathStyle,

    draw: Option<Vec<BoxDraw>>,
    stale: Stale,
}

impl BoxPlot {
    fn new(data: Vec<Tensor>) -> Self {
        let style = PathStyle::new();

        let mut box_plot = Self {
            whisker: 1.5,
            is_draw_outliers: true,

            style,

            data: Default::default(),

            stale: Stale::stale(),
            draw: None,
        };

        box_plot.set_data(data);

        box_plot
    }

    fn set_data(&mut self, data: Vec<Tensor>) {
        self.data = data.iter().enumerate()
            .map(|(i, item)| {
                BoxPlotData::new(&self, (i + 1) as f32, item)
            }).collect();
        self.stale = Stale::stale();
    }

    fn resize(
        &mut self, 
        to_canvas: &ToCanvas<Data>,
    ) {
        if self.stale != to_canvas.stale() || self.draw.is_none() {
            self.stale = to_canvas.stale();

            let draw_vec = self.data.iter()
                .enumerate()
                .map(|(i, data)| {
                    let x = (i + 1) as f32;
                    let box_w = 0.25;
                    let path_box = paths::rect::<Data>(
                        Point(x - box_w, data.quartiles[1]),
                        Point(x + box_w, data.quartiles[3])
                    );
                    
                    let path_median = Path::<Data>::move_to(x - box_w, data.quartiles[2])
                        .line_to(x + box_w, data.quartiles[2])
                        .to_path();

                    let w = 0.1;
                    let path_bottom = Path::<Data>::move_to(x - w, data.quartiles[0])
                        .line_to(x + w, data.quartiles[0])
                        .to_path();
                    let path_bottom2 = Path::<Data>::move_to(x, data.quartiles[0])
                        .line_to(x, data.quartiles[1])
                        .to_path();
                    let path_top = Path::<Data>::move_to(x - w, data.quartiles[4])
                        .line_to(x + w, data.quartiles[4])
                        .to_path();
                    let path_top2 = Path::<Data>::move_to(x, data.quartiles[4])
                        .line_to(x, data.quartiles[3])
                        .to_path();

                    let outliers = if data.outliers.len() > 0 {
                        let marker = Markers::Circle.get_scaled_path(10.);

                        let xy = to_canvas.transform_tensor(&data.outliers);

                        let mut scatter = PathCollection::new(marker, xy);
                        scatter.style_mut().face_color(Color::black());

                        Some(scatter)
                    } else {
                        None
                    };

                    BoxDraw {
                        path_box: to_canvas.transform_path(&path_box),

                        path_median: to_canvas.transform_path(&path_median),

                        path_top: to_canvas.transform_path(&path_top),
                        path_top2: to_canvas.transform_path(&path_top2),
                        path_bottom: to_canvas.transform_path(&path_bottom),
                        path_bottom2: to_canvas.transform_path(&path_bottom2),

                        outliers,
                    }
                }
            ).collect::<Vec<BoxDraw>>();

            self.draw = Some(draw_vec);
        }
    }
}

impl ArtistDraw<Data> for BoxPlot {
    fn bounds(&mut self) -> Bounds<Data> {
        let (mut min, mut max) = (f32::MAX, f32::MIN);

        for item in &self.data {
            min = min.min(item.quartiles[0]);
            max = max.max(item.quartiles[4]);

            if self.is_draw_outliers {
                min = min.min(item.min);
                max = max.max(item.max);
            }
        }

        Bounds::new(Point(0.5, min), Point(self.data.len() as f32 + 0.5, max))
    }

    fn draw(
        &mut self, 
        ui: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        base_style: &dyn PathOpt,
    ) -> Result<()> {
        self.resize(to_canvas);

        if let Some(box_draw_list) = &mut self.draw {
            let style = self.style.push(base_style);

            let mut median_style = PathStyle::new();
            if let Some(line_width) = self.style.get_line_width() {
                median_style.line_width(line_width);
            }

            for box_draw in box_draw_list {
                ui.draw_path(&box_draw.path_box, &style)?;

                let median_style = median_style.push(base_style);
                ui.draw_path(&box_draw.path_median, &median_style)?;

                ui.draw_path(&box_draw.path_top, &style)?;
                ui.draw_path(&box_draw.path_top2, &style)?;
                ui.draw_path(&box_draw.path_bottom, &style)?;
                ui.draw_path(&box_draw.path_bottom2, &style)?;

                if let Some(outliers) = &mut box_draw.outliers {
                    outliers.draw(ui, to_canvas, base_style)?;
                }
            }
        }

        // self.collection.draw(ui, to_canvas, &style)

        Ok(())
    }
}

struct BoxPlotData {
    quartiles: [f32; 5],

    min: f32,
    max: f32,
    outliers: Tensor,
}

impl BoxPlotData {
    fn new(box_plot: &BoxPlot, x: f32, data: &Tensor) -> Self {
        let mut data = Vec::from(data.as_slice());
        data.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if a < b {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let len = data.len();
        let mean = data[len / 2];
        let first = data[len / 4];
        let third = data[3 * len / 4];

        let bot_whisker = first - box_plot.whisker * (third - first);
        let top_whisker = third + box_plot.whisker * (third - first);

        let (mut bot, mut top) = (f32::MAX, f32::MIN);
        let (mut min, mut max) = (f32::MAX, f32::MIN);

        let mut outliers = Vec::<[f32; 2]>::new();

        for value in data {
            if value <= top_whisker {
                top = top.max(value);
            } else {
                outliers.push([x, value]);
            }

            if bot_whisker <= value {
                bot = bot.min(value);
            } else {
                outliers.push([x, value]);
            }

            min = min.min(value);
            max = max.max(value);
        }
        
        Self {
            quartiles: [
                bot,
                first,
                mean,
                third,
                top,
            ],
            min,
            max,
            outliers: outliers.into(),
        }
    }
}

struct BoxDraw {
    path_box: Path<Canvas>,

    path_median: Path<Canvas>,

    path_top: Path<Canvas>,
    path_top2: Path<Canvas>,
    path_bottom: Path<Canvas>,
    path_bottom2: Path<Canvas>,

    outliers: Option<PathCollection>,
}

impl Artist<Data> for BoxPlot {
    type Opt = BoxPlotOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.style = PathStyle::from_config(cfg, "box_plot");
    }

    fn opt(&mut self, view: ArtistView<Data, BoxPlot>) -> Self::Opt {
        BoxPlotOpt::new(view)
    }

    fn get_legend(&self) -> Option<LegendHandler> {
        None
    }
}

data_artist_option_struct!(BoxPlotOpt, BoxPlot);

impl BoxPlotOpt {
    path_style_options!(style);

    pub fn data(&mut self, data: impl IntoTensorList<f32>) -> &mut Self {
        self.write(|plot| {
            let mut vec = Vec::new();
            data.into_list(&mut vec);
            plot.set_data(vec);
        });

        self
    }
}
