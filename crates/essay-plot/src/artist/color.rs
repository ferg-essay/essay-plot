use core::fmt;
use std::{ops::Index, f32::consts::TAU};

use essay_plot_base::Color;


pub struct ColorCycle {
    colors: Vec<(String, Color)>,
}

impl ColorCycle {
    pub fn new(colors: &Vec<(&str, Color)>) -> Self {
        let mut vec = Vec::<(String, Color)>::new();

        for (name, color) in colors {
            vec.push((name.to_string(), *color));
        }

        Self {
            colors: vec
        }
    }
}

impl Index<usize> for ColorCycle {
    type Output = Color;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let len = self.colors.len();

        &self.colors[index % len].1
    }
}

impl ColorCycle {
    pub fn tableau() -> ColorCycle {
        Self::new(&vec![
            ("tab:blue", Color(0x1f77b4ff)),
            ("tab:orange", Color(0xff7f0eff)),
            ("tab:green", Color(0x2ca02cff)),
            ("tab:red", Color(0xd62728ff)),
            ("tab:purple", Color(0x9467bdff)),
            ("tab:brown", Color(0x8c564bff)),
            ("tab:pink", Color(0xe377c2ff)),
            ("tab:gray", Color(0x7f7f7fff)),
            ("tab:olive", Color(0xbcbd22ff)),
            ("tab:cyan", Color(0xf17becff)),
        ])
    }
}

impl Default for ColorCycle {
    fn default() -> Self {
        ColorCycle::tableau()
    }
}

pub struct Lab(f32, f32, f32);

impl Lab {
    #[inline]
    pub fn l(&self) -> f32 {
        self.0
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.1
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.2
    }
}

impl fmt::Debug for Lab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Lab").field(&self.0).field(&self.1).field(&self.2).finish()
    }
}


impl From<Lab> for Color {
    fn from(value: Lab) -> Self {
        Color::from_lab(value.l(), value.a(), value.b())
    }
}

impl From<Color> for Lab {
    fn from(color: Color) -> Self {
        let [l, a, b] = color.to_lab();

        Self(l, a, b)
    }
}
