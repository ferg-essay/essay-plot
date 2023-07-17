pub trait TickFormatter : Send {
    fn format(&self, value: f32, axis_min: f32, axis_max: f32) -> String;
}

pub enum Formatter {
    Plain,
}

impl TickFormatter for Formatter {
    fn format(&self, value: f32, axis_min: f32, axis_max: f32) -> String {
        match self {
            Formatter::Plain => {
                format_tick(value, axis_min, axis_max)
            }
        }
    }
}

fn format_tick(value: f32, min: f32, max: f32) -> String {
    let values = (max - min) * 1e-4;
    /*
    if values.fract() == 0. {
        format!("{}", value.round() as u64)
    } else if value.fract() == 0. {
        format!("{}", value.round() as u64)
    } else if (10. * value).fract() < values {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
    }
    */
    format!("{:.2}", value)
}