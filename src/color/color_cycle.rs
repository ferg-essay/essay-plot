use core::fmt;
use std::ops::Index;

use essay_graphics::api::Color;
use super::{palette_brewer::{BrewerDiverging, BrewerQualitative, BrewerSequential}, palette_vega::Vega};

pub struct ColorCycle {
    colors: Vec<Vec<Color>>,
}

impl ColorCycle {
    pub fn new(colors: &Vec<Vec<Color>>) -> Self {
        let mut cycle_group = Vec::new();

        let mut n = 0;

        for color_cycle in colors {
            for i in n..color_cycle.len() {
                let mut cycle = Vec::new();

                for j in 0..i {
                    cycle.push(color_cycle[j]);
                }

                cycle_group.push(cycle);
            }

            n = color_cycle.len();
        }

        assert!(n > 0);

        Self {
            colors: cycle_group,
        }
    }

    pub fn color(&self, i: usize, n: usize) -> Color {
        assert!(n > 0);

        let n = n.min(self.colors.last().unwrap().len());

        self.colors[n][i % n]
    }
}

impl Index<usize> for ColorCycle {
    type Output = Color;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let len = self.colors.last().unwrap().len();

        &self.colors.last().unwrap()[index % len]
    }
}

impl Default for ColorCycle {
    fn default() -> Self {
        Qualitative::Tableau.into()
    }
}

impl From<&[Color]> for ColorCycle {
    fn from(value: &[Color]) -> Self {
        ColorCycle::new(&vec![Vec::from(value)])
    }
}

impl<const N: usize> From<&[Color; N]> for ColorCycle {
    fn from(value: &[Color; N]) -> Self {
        ColorCycle::new(&vec![Vec::from(value)])
    }
}

impl<const N: usize> From<&[u32; N]> for ColorCycle {
    fn from(value: &[u32; N]) -> Self {
        ColorCycle::new(&vec![colors_from(value)])
    }
}

impl<const N: usize> From<&[&str; N]> for ColorCycle {
    fn from(value: &[&str; N]) -> Self {
        let colors: Vec<Color> = value.iter().map(|v| Color::from(*v)).collect();

        ColorCycle::new(&vec![colors])
    }
}

impl From<&([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9])> for ColorCycle {
    fn from(value: &([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9])) -> Self {
        ColorCycle::new(&vec![
            colors_from(&value.0),
            colors_from(&value.1),
            colors_from(&value.2),
            colors_from(&value.3),
            colors_from(&value.4),
            colors_from(&value.5),
            colors_from(&value.6),
        ])
    }
}

impl From<&([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11])> for ColorCycle {
    fn from(value: &([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11])) -> Self {
        ColorCycle::new(&vec![
            colors_from(&value.0),
            colors_from(&value.1),
            colors_from(&value.2),
            colors_from(&value.3),
            colors_from(&value.4),
            colors_from(&value.5),
            colors_from(&value.6),
            colors_from(&value.7),
            colors_from(&value.8),
        ])
    }
}

fn colors_from<const N: usize>(value: &[u32; N]) -> Vec<Color> {
    value.iter().map(|v| Color::from(*v)).collect()
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sequential {
    PuBuGn,
    YlGn,
    Blues,
}

impl From<Sequential> for ColorCycle {
    fn from(value: Sequential) -> Self {
        match value {
            Sequential::PuBuGn => ColorCycle::from(&BrewerSequential::PU_BU_GN),
            Sequential::YlGn => ColorCycle::from(&BrewerSequential::YL_GN),
            Sequential::Blues => ColorCycle::from(&BrewerSequential::BLUES),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Diverging {
    /// <div style="background-color: #7f3b08; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b35806; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e08214; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdb863; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee0b6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f7f7f7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d8daeb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b2abd2; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #8073ac; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #542788; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #2d004b; width: 10px; padding: 10px; border: 1px solid;"></div>
    OrPu,

    /// <div style="background-color: #9e0142; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d53e4f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f46d43; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdae61; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee08b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffffbf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e6f598; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #abdda4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #66c2a5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #3288bd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #5e4fa2; width: 10px; padding: 10px; border: 1px solid;"></div>
    Spectral,
}

impl From<Diverging> for ColorCycle {
    fn from(value: Diverging) -> Self {
        match value {
            Diverging::OrPu => ColorCycle::from(&BrewerDiverging::OR_PU),
            Diverging::Spectral => ColorCycle::from(&BrewerDiverging::SPECTRAL),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Qualitative {
    Tableau,
    /// <div style="background-color: #8dd3c7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffffb3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bebada; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fb8072; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #80b1d3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdb462; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b3de69; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fccde5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d9d9d9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bc80bd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ccebc5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffed6f; width: 10px; padding: 10px; border: 1px solid;"></div>
    Set3,
}

impl From<Qualitative> for ColorCycle {
    fn from(value: Qualitative) -> Self {
        match value {
            Qualitative::Tableau => ColorCycle::from(&Vega::TABLEAU_B),
            Qualitative::Set3 => ColorCycle::from(&BrewerQualitative::SET3),
        }
    }
}
