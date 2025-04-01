use std::{ops::Index, str::FromStr};

use essay_graphics::api::{path_opt::StyleErr, Color};
use super::Category;

#[derive(Clone)]
pub struct Palette {
    colors: Vec<Vec<Color>>,
}

impl Palette {
    pub fn new(colors: &Vec<Vec<Color>>) -> Self {
        let mut cycle_group = Vec::new();

        let mut n = 0;

        for color_cycle in colors {
            for i in n..color_cycle.len() {
                let mut cycle = Vec::new();

                for j in 0..i + 1 {
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

    // cycle color
    pub fn color(&self, i: usize, n: usize) -> Color {
        assert!(n > 0);

        let n = n.min(self.colors.last().unwrap().len());

        self.colors[n - 1][i % n]
    }

    pub fn colors(&self) -> &Vec<Color> {
        self.colors.last().unwrap()
    }
}

impl Index<usize> for Palette {
    type Output = Color;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let len = self.colors.last().unwrap().len();

        &self.colors.last().unwrap()[index % len]
    }
}

impl Default for Palette {
    fn default() -> Self {
        Category::Tableau.into()
    }
}

impl From<&[Color]> for Palette {
    fn from(value: &[Color]) -> Self {
        Palette::new(&vec![Vec::from(value)])
    }
}

impl<const N: usize> From<&[Color; N]> for Palette {
    fn from(value: &[Color; N]) -> Self {
        Palette::new(&vec![Vec::from(value)])
    }
}

impl<const N: usize> From<&[u32; N]> for Palette {
    fn from(value: &[u32; N]) -> Self {
        Palette::new(&vec![colors_from(value)])
    }
}

impl<const N: usize> From<&[&str; N]> for Palette {
    fn from(value: &[&str; N]) -> Self {
        let colors: Vec<Color> = value.iter().map(|v| Color::from(*v)).collect();

        Palette::new(&vec![colors])
    }
}

impl From<&[&str]> for Palette {
    fn from(value: &[&str]) -> Self {
        let colors: Vec<Color> = value.iter().map(|v| Color::from(*v)).collect();

        Palette::new(&vec![colors])
    }
}

impl From<&([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9])> for Palette {
    fn from(value: &([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9])) -> Self {
        Palette::new(&vec![
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

impl From<&([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11])> for Palette {
    fn from(value: &([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11])) -> Self {
        Palette::new(&vec![
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

// simplified: top 5, top 10
impl From<&([u32; 5], [u32; 10])> for Palette {
    fn from(value: &([u32; 5], [u32; 10])) -> Self {
        Palette::new(&vec![
            colors_from(&value.0),
            colors_from(&value.1),
        ])
    }
}

// simplified: top 3, top 5, top 10, top 20
impl From<&([u32; 5], [u32; 10], [u32; 20])> for Palette {
    fn from(value: &([u32; 5], [u32; 10], [u32; 20])) -> Self {
        Palette::new(&vec![
            colors_from(&value.0),
            colors_from(&value.1),
            colors_from(&value.2),
        ])
    }
}

impl FromStr for Palette {
    type Err = StyleErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec : Vec<&str> = Vec::new();

        for item in s.split(",") {
            let item = item.trim();

            // TODO: also trim quotes
            vec.push(item);
        };

        Ok(Palette::from(vec.as_slice()))
    }
}

fn colors_from<const N: usize>(value: &[u32; N]) -> Vec<Color> {
    value.iter().map(|v| Color::from(*v)).collect()
}
