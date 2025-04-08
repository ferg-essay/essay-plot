use std::sync::OnceLock;

use essay_graphics::api::Color;

use super::{ColorMap, Palette};

/// https://colorcet.com
/// Peter Kovesi. Good Colour Maps: How to Design Them.
/// arXiv:1509.03700 [cs.GR] 2015
#[derive(Clone, Debug, PartialEq)]
pub enum Colorcet {
    /// Cyclic Magenta-Red-Yellow-Blue. Allows four phase visualization
    C1,
    C1s,
    /// Cyclic Magenta-Yellow-Green-Blue. Allows four phase visualization
    C2,
    C2s,
    /// Cyclic White-Red-Black-Blue
    C3,
    C3s,
    /// Cyclic White-Red-White-Blue
    C4,
    C4s,
    /// Cyclic Greyscale
    C5,
    C5s,
    /// Six-color cyclic
    C6,
    C6s,
    /// Cyclic Yellow-Magenta-Cyan-Green-Yellow
    C7,
    C7s,

    /// Cyclic 4-phase map Blue-White-Yellow-Black for color blind
    Cbc1,
    /// Cyclic 2-phase map White-Yellow-White-Blue for color blind
    Cbc2,
    /// Diverging map for color blind
    Cbd1,
    /// Linear map for color blind
    Cbl1,
    /// Linear map with maximal chroma
    Cbl2,

    /// Diverging Blue-White-Red
    D01,
    /// Diverging Blue-White-Red with darker endpoint
    D01a,
    /// Diverging Green-White-Violet
    D02,
    /// Diverging Green-White-Red
    D03,
    /// Diverging Blue-Black-Red
    D04,
    /// Diverging Blue-Black-Yellow
    D06,
    /// Diverging Blue-Grey-Yellow
    D07,
    /// Linear diverging Blue-Grey-Red
    D08,
    /// Diverging Blue-White-Red low contrast
    D09,
    /// Diverging Cyan-White-Magenta low contrast
    D10,
    /// Diverging isoluminant LightBlue-LightGrey-Orange
    D11,
    /// Diverging isoluminant LightBlue-LightGrey-Pink
    D12,
    /// Diverging Blue-White-Green low contrast
    D13,

    /// Isoluminant blue-pink
    I1,
    /// Isoluminant blue-grey-orange
    I2,
    /// Isoluminant blue-grey-pink
    I3,

    /// Grey scale
    L01,
    /// Grey scale with reduced contrast
    L02,
    /// Black-Red-Yellow-White heat map
    L03,
    /// Black-Red-Yellow heat map
    L04,
    /// Green edge of CIELAB color map
    L05,
    /// Blue edge of CIELAB color map
    L06,
    /// Blue-Pink-Light Pink color map
    L07,
    /// Blue-Magenta-Yellow saturated color map
    L08,
    // Blue-Green-Yellow color map
    L09,
    // Geographical color map
    L10,
    // Light geographical color map
    L11,
    // Water depth color map
    L12,
    // Red color map for ternary
    L13,
    // Green color map for ternary
    L14,
    // Blue color map for ternary
    L15,
    // Black-Blue-Green-Yellow-White color map
    L16,
    // White-Orange-Red-Blue map with increasing saturation
    L17,
    // White-Yellow-Orange-Red map with increasing saturation
    L18,
    // White-Cyan-Magenta-Red map with increasing saturation
    L19,
    // Black-Blue-Green-Orange-Yellow map
    L20,

    R1,
    R2,
    /// Diverging rainbow
    R3,
    R4,
}

macro_rules! palette_csv {
    ($e:expr) => {
        PaletteCsv::read_palette(
            include_bytes!(concat!("../../assets/palettes/", $e))
        )
    }
}

macro_rules! palette_csv_cache {
    ($id:ident, $e:expr) => {
        $id.get_or_init(|| PaletteCsv::read_palette(
            include_bytes!(concat!("../../assets/palettes/", $e))
        )).clone()
    }
}

impl From<Colorcet> for Palette {
    fn from(value: Colorcet) -> Self {
        match value {
            Colorcet::C1 => palette_csv!("CET-C1.csv"),
            Colorcet::C1s => palette_csv!("CET-C1s.csv"),
            Colorcet::C2 => palette_csv!("CET-C2.csv"),
            Colorcet::C2s => palette_csv!("CET-C2s.csv"),
            Colorcet::C3 => palette_csv!("CET-C3.csv"),
            Colorcet::C3s => palette_csv!("CET-C3s.csv"),
            Colorcet::C4 => palette_csv!("CET-C4.csv"),
            Colorcet::C4s => palette_csv!("CET-C4s.csv"),
            Colorcet::C5 => palette_csv!("CET-C5.csv"),
            Colorcet::C5s => palette_csv!("CET-C5s.csv"),
            Colorcet::C6 => palette_csv!("CET-C6.csv"),
            Colorcet::C6s => palette_csv!("CET-C6s.csv"),
            Colorcet::C7 => palette_csv!("CET-C7.csv"),
            Colorcet::C7s => palette_csv!("CET-C7s.csv"),

            Colorcet::Cbc1 => palette_csv!("CET-CBC1.csv"),
            Colorcet::Cbc2 => palette_csv!("CET-CBC2.csv"),
            Colorcet::Cbd1 => palette_csv!("CET-CBD1.csv"),
            Colorcet::Cbl1 => palette_csv!("CET-CBL1.csv"),
            Colorcet::Cbl2 => palette_csv!("CET-CBL2.csv"),

            Colorcet::I1 => palette_csv!("CET-I1.csv"),
            Colorcet::I2 => palette_csv!("CET-I2.csv"),
            Colorcet::I3 => palette_csv!("CET-I3.csv"),

            Colorcet::D01 => palette_csv!("CET-D01.csv"),
            Colorcet::D01a => palette_csv!("CET-D01A.csv"),
            Colorcet::D02 => palette_csv!("CET-D02.csv"),
            Colorcet::D03 => palette_csv!("CET-D03.csv"),
            Colorcet::D04 => palette_csv!("CET-D04.csv"),
            Colorcet::D06 => palette_csv!("CET-D06.csv"),
            Colorcet::D07 => palette_csv!("CET-D07.csv"),
            Colorcet::D08 => palette_csv!("CET-D08.csv"),
            Colorcet::D09 => palette_csv!("CET-D09.csv"),
            Colorcet::D10 => palette_csv!("CET-D10.csv"),
            Colorcet::D11 => palette_csv!("CET-D11.csv"),
            Colorcet::D12 => palette_csv!("CET-D12.csv"),
            Colorcet::D13 => palette_csv!("CET-D13.csv"),

            Colorcet::L01 => palette_csv_cache!(L01, "CET-L01.csv"),
            Colorcet::L02 => palette_csv_cache!(L02, "CET-L02.csv"),
            Colorcet::L03 => palette_csv_cache!(L03, "CET-L03.csv"),
            Colorcet::L04 => palette_csv!("CET-L04.csv"),
            Colorcet::L05 => palette_csv!("CET-L05.csv"),
            Colorcet::L06 => palette_csv!("CET-L06.csv"),
            Colorcet::L07 => palette_csv!("CET-L07.csv"),
            Colorcet::L08 => palette_csv!("CET-L08.csv"),
            Colorcet::L09 => palette_csv!("CET-L09.csv"),
            Colorcet::L10 => palette_csv!("CET-L10.csv"),
            Colorcet::L11 => palette_csv!("CET-L11.csv"),
            Colorcet::L12 => palette_csv!("CET-L12.csv"),
            Colorcet::L13 => palette_csv!("CET-L13.csv"),
            Colorcet::L14 => palette_csv!("CET-L14.csv"),
            Colorcet::L15 => palette_csv!("CET-L15.csv"),
            Colorcet::L16 => palette_csv!("CET-L16.csv"),
            Colorcet::L17 => palette_csv!("CET-L17.csv"),
            Colorcet::L18 => palette_csv!("CET-L18.csv"),
            Colorcet::L19 => palette_csv!("CET-L19.csv"),
            Colorcet::L20 => palette_csv!("CET-L20.csv"),

            Colorcet::R1 => palette_csv!("CET-R1.csv"),
            Colorcet::R2 => palette_csv!("CET-R2.csv"),
            Colorcet::R3 => palette_csv!("CET-R3.csv"),
            Colorcet::R4 => palette_csv!("CET-R4.csv"),
        }
    }
}

impl From<Colorcet> for ColorMap {
    fn from(value: Colorcet) -> Self {
        ColorMap::from(Palette::from(value))
    }
}

struct PaletteCsv<'a> {
    data: &'a [u8],
    i: usize,
}

impl<'a> PaletteCsv<'a> {
    fn read_palette(data: &[u8]) -> Palette {
        let mut colors = Vec::new();
        let mut cursor = PaletteCsv::new(data);
    
        while let Some(color) = cursor.read_color() {
            colors.push(color);
        }
    
        assert!(colors.len() > 0);
        
        Palette::from(colors)
    }

    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            i: 0,
        }
    }

    fn read_color(&mut self) -> Option<Color> {
        if let Some(r) = self.read_number() {
            self.skip_comma();

            if let Some(g) = self.read_number() {
                self.skip_comma();

                if let Some(b) = self.read_number() {
                    self.skip_to_eol();

                    return Some(Color::from_rgb(
                        r as f32 / 256., 
                        g as f32 / 256., 
                        b as f32 / 256.)
                    )
                }
            }
        }

        None
    }

    fn read_number(&mut self) -> Option<usize> {
        let mut n = 0;

        while let Some(d) = self.read() {
            if '0' <= d as char && d as char <= '9' {
                n = 10 * n + d as usize - '0' as usize;
            } else {
                self.unread();

                return Some(n);
            }
        }

        None
    }
    
    fn skip_comma(&mut self) {
        while let Some(d) = self.read() {
            match d as char {
                ' ' | '\t' | '\r' | ',' => {}
                _ => { 
                    self.unread();
                    return;
                }
            }
        }
    }
    
    fn skip_to_eol(&mut self) {
        while let Some(d) = self.read() {
            if d as char == '\n' {
                return;
            }
        }
    }
    
    fn unread(&mut self) {
        self.i -= 1;
    }

    fn read(&mut self) -> Option<u8> {
        let i = self.i;

        if i < self.data.len() {
            self.i = i + 1;

            Some(self.data[i])
        } else {
            None
        }
    }
}

macro_rules! palette_cache {
    ($($id:ident)*) => {
        $(
            static $id: OnceLock<Palette> = OnceLock::new();
        )*
    }
}

palette_cache!(L01 L02 L03);