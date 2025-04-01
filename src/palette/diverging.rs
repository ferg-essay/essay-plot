use super::{palette_brewer::BrewerDiverging, ColorMap, Palette};


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Diverging {
    // Brewer diverging

    // #7f3b08 #b35806 #e08214 #fdb863 #fee0b6  #f7f7f7 
    // #d8daeb #b2abd2 #8073ac #542788 #2d004b
    OrangePurple,
    // #543005 #8c510a #bf812d #dfc27d #f6e8c3 #f5f5f5
    // #c7eae5 #80cdc1 #35978f #01665e #003c30
    BrownTeal,
    // #40004b #762a83 #9970ab #c2a5cf #e7d4e8 #f7f7f7
    // #d9f0d3 #a6dba0 #5aae61 #1b7837 #00441b
    PurpleGreen,
    // #8e0152 #c51b7d #de77ae #f1b6da #fde0ef #f7f7f7
    // #e6f5d0 #b8e186 #7fbc41 #4d9221 #276419
    PinkGreen,
    // #67001f #b2182b #d6604d #f4a582 #fddbc7 #f7f7f7
    // #d1e5f0 #92c5de #4393c3 #2166ac #053061
    RedBlue,
    // #67001f #b2182b #d6604d #f4a582 #fddbc7 #ffffff
    // #e0e0e0 #bababa #878787 #4d4d4d #1a1a1a
    RedGrey,
    // #a50026 #d73027 #f46d43 #fdae61 #fee090 #ffffbf
    // #e0f3f8 #abd9e9 #74add1 #4575b4 #313695
    RedYellowBlue,
    // #a50026 #d73027 #f46d43 #fdae61 #fee08b  #ffffbf
    // #d9ef8b #a6d96a #66bd63 #1a9850 #006837
    RedYellowGreen,
    // #9e0142 #d53e4f #f46d43 #fdae61 #fee08b #ffffbf
    // #e6f598 #abdda4 #66c2a5 #3288bd #5e4fa2
    Spectral,
}

impl From<Diverging> for Palette {
    fn from(value: Diverging) -> Self {
        match value {
            Diverging::OrangePurple => {
                Palette::from(&BrewerDiverging::OR_PU)
            }
            Diverging::BrownTeal => {
                Palette::from(&BrewerDiverging::BR_BG)
            }
            Diverging::PurpleGreen => {
                Palette::from(&BrewerDiverging::PU_GN)
            }
            Diverging::PinkGreen => {
                Palette::from(&BrewerDiverging::PI_YG)
            }
            Diverging::RedBlue => {
                Palette::from(&BrewerDiverging::RD_BU)
            }
            Diverging::RedGrey => {
                Palette::from(&BrewerDiverging::RD_GY)
            }
            Diverging::RedYellowBlue => {
                Palette::from(&BrewerDiverging::RD_YL_BU)
            }
            Diverging::RedYellowGreen => {
                Palette::from(&BrewerDiverging::RD_YL_GN)
            }
            Diverging::Spectral => {
                Palette::from(&BrewerDiverging::SPECTRAL)
            }
        }
    }
}

impl From<Diverging> for ColorMap {
    fn from(value: Diverging) -> Self {
        ColorMap::from(Palette::from(value))
    }
}
