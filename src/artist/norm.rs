use essay_tensor::Tensor;

pub struct Norm {
    vmin: Option<f32>,
    vmax: Option<f32>,

    min: f32,
    max: f32,

    scale: Box<dyn Fn(f32) -> f32>,
}

impl Norm {
    pub fn new(fun: impl Fn(f32) -> f32 + 'static) -> Self {
        Self {
            vmin: None,
            vmax: None,
            min: -1.,
            max: 1.,
            scale: Box::new(fun),
        }
    }

    #[inline]
    fn scale(&self, value: f32) -> f32 {
        (self.scale)(value)
    }

    pub fn set_bounds(&mut self, values: &Tensor<f32>) {
        let (mut min, mut max) = (f32::MAX, f32::MIN);

        for value in values.iter() {
            min = min.min(*value);
            max = max.max(*value);
        }

        if min == max {
            min -= 1.;
            max += 1.;
        }

        self.min = min;
        self.max = max;

        if let Some(min) = self.vmin {
            self.min = min;
        }

        if let Some(max) = self.vmax {
            self.max = max;
        }
    }

    pub fn vmin(&mut self, min: f32) -> &mut Self {
        self.vmin = Some(min);

        self
    }

    pub fn vmax(&mut self, max: f32) -> &mut Self {
        self.vmax = Some(max);

        self
    }

    #[inline]
    pub fn norm(&self, value: f32) -> f32 {
        let value = self.scale(value);

        (value - self.min) / (self.max - self.min)
    }
}

pub enum Norms {
    Linear,
}

impl From<Norms> for Norm {
    fn from(value: Norms) -> Self {
        match value {
            Norms::Linear => Norm::new(scale_linear),
        }
    }
}

fn scale_linear(value: f32) -> f32 {
    value
}