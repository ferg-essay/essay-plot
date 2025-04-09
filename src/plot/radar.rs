use essay_graphics::api::{renderer::{self, Renderer}, Bounds, Path, PathOpt};
use essay_tensor::{init::linspace, tensor::Tensor};

use crate::{artist::{Artist, ArtistDraw, ArtistView, Lines2d, LinesOpt}, chart::{Chart, Data, PolarChart}, config::{ConfigArc, PathStyle}, data_artist_option_struct, path_style_options, transform::ToCanvas};

pub fn radar(
    polar: &mut PolarChart, 
    y: impl Into<Tensor>, 
) -> RadarOpt {
    polar.radar(y)
}

impl PolarChart {
    pub fn radar(
        &mut self,
        y: impl Into<Tensor>,
    ) -> RadarOpt {
        let mut vec_y: Vec<f32> = y.into().iter().map(|v| *v).collect();

        vec_y.push(vec_y[0]);

        let x = linspace(0., vec_y.len() as f32 - 1., vec_y.len());

        let lines = Lines2d::from_xy(x, vec_y);

        let radar = Radar::new(lines);

        self.artist(radar)
    }
}

struct Radar {
    lines: Lines2d,
    fill: Path<Data>,

    style: PathStyle,
    fill_style: PathStyle,
}

impl Radar {
    fn new(lines: Lines2d) -> Self {
        let mut radar = Self {
            lines,
            fill: Path::move_to(0., 0.).to_path(),
            style: PathStyle::new(),
            fill_style: PathStyle::new(),
        };

        radar.fill_style.alpha(0.2);

        radar.fill();

        radar
    }

    fn set_y(&mut self, y: impl Into<Tensor>) {
        let mut vec_y: Vec<f32> = y.into().iter().map(|v| *v).collect();

        vec_y.push(vec_y[0]);

        let x = linspace(0., vec_y.len() as f32 - 1., vec_y.len());

        self.lines.set_xy(x, vec_y);

        self.fill();
    }

    fn fill(&mut self) {
        let xy = self.lines.get_xy();

        let mut path = Path::<Data>::move_to(xy[(0, 0)], xy[(0, 1)]);

        for row in xy.iter_row().skip(1) {
            path = path.line_to(row[0], row[1]);
        }

        self.fill = path.close_poly(xy[(0, 0)], xy[(0, 1)]).to_path();
    }
}

impl ArtistDraw<Data> for Radar {
    fn bounds(&mut self) -> Bounds<Data> {
        self.lines.bounds()
    }

    fn draw(
        &mut self, 
        ui: &mut dyn Renderer,
        to_canvas: &ToCanvas<Data>,
        style: &dyn PathOpt,
    ) -> renderer::Result<()> {
        self.lines.draw(ui, to_canvas, &self.style.push(style))?;

        let path = to_canvas.transform_path(&self.fill);
        ui.draw_path(&path, &self.fill_style.push(style))
    }
}

impl Artist<Data> for Radar {
    type Opt = RadarOpt;

    fn config(&mut self, cfg: &ConfigArc) {
        self.lines.config(cfg);
        self.style = PathStyle::from_config(cfg, "radar");
    }

    fn opt(&mut self, view: ArtistView<Data, Radar>) -> Self::Opt {
        RadarOpt::new(view)
    }
}


data_artist_option_struct!(RadarOpt, Radar);

impl RadarOpt {
    path_style_options!(style);

    pub fn fill_alpha(&mut self, alpha: f32) -> &mut Self {
        self.write(|artist| {
            artist.fill_style.alpha(alpha);
        });

        self
    }

    pub fn set_y(&mut self, y: impl Into<Tensor>) -> &mut Self {
        self.write(|artist| {
            artist.set_y(y);
        });

        self
    }
}
