use essay_plot_base::Color;

use super::{ColorMap};

pub enum ColorMaps {
    Default,
    BlueOrange,
    BlueYellow,
    BlackWhite,
}

impl From<ColorMaps> for ColorMap {
    fn from(value: ColorMaps) -> Self {
        match value {
            ColorMaps::BlueOrange | ColorMaps::Default => {
                // Top 1% options: vermillion, red, bright red, tomato red
                // Bottom 1% options: navy, dark navy, ultramarine blue, night blue
                // royal blue, cobalt blue
                // TODO: possibly use hsv or lab (or msh) instead of color names
                ColorMap::from([
                    (0., "deep blue"),  // bottom 1% distinct
                    // cool, saturated blue to warm, unsaturated blue
                    (0.01, "cobalt blue"), (0.1, "blue"), (0.2, "azure"),
                    //(0.5, "white"),
                    (0.5, "#f0f0f0"), // tone down to be less distracting
                    // cool, unsaturated orange to warm, saturated orange
                    (0.8, "amber"), (0.9, "orange"), (0.99, "tomato red"), 
                    (1.0, "red") // top 1% distinct
                ])
            }

            ColorMaps::BlueYellow => {
                // Top 1% options: vermillion, red, bright red, tomato red
                // Bottom 1% options: navy, dark navy, ultramarine blue, night blue
                // royal blue, cobalt blue
                // TODO: possibly use hsv instead of color names

                ColorMap::from([
                    (0., "deep blue"),  // bottom 1% distinct
                    // cool, saturated blue to warm, unsaturated blue
                    (0.01, "cobalt blue"), (0.1, "blue"), (0.2, "azure"),
                    (0.5, "#a8a8a8"),
                    // cool, unsaturated orange to warm, saturated orange
                    (0.8, "amber"), (1., "yellow"), 
                ])
            }

            ColorMaps::BlackWhite => {
                ColorMap::from([
                    (0., "black"),
                    (1., "white"),
                ])
            }
        }
    }
}