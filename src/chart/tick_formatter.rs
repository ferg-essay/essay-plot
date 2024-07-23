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
    // handle delta = 0.19999 vs delta = 0.2004
    let delta = delta + delta * 1e-2;
    let mut precision = (- delta.log10().floor()).max(0.) as usize;

    // handle delta = 0.25
    let p_digit = 10.0f32.powi(- (precision as i32));
    let rem = delta % p_digit;
    let rem = rem.min(p_digit - rem);

    if rem > 0. && -2. < rem.log10() && rem.log10() < -1. {
        precision += 1;
    }

    format!("{:-#.*}", precision, value)
}
