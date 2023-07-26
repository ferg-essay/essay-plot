pub trait TickFormatter : Send {
    fn format(&self, value: f32, delta: f32) -> String;
}

pub enum Formatter {
    Plain,
}

impl TickFormatter for Formatter {
    fn format(&self, value: f32, delta: f32) -> String {
        match self {
            Formatter::Plain => {
                format_tick(value, delta)
            }
        }
    }
}

fn format_tick(value: f32, delta: f32) -> String {
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
    let precision = (- delta.log10().floor()).max(0.) as usize;
    format!("{:-#.*}", precision, value)
}
