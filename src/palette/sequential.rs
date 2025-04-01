use super::{palette::Palette, palette_brewer::BrewerSequential, palette_vega::Vega, ColorMap};


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Sequential {
    // #f7fcfd #e5f5f9 #ccece6 #99d8c9 #66c2a4
    // #41ae76 #238b45 #006d2c #00441b
    BlueGreen,
    // #f7fcfd #e0ecf4 #bfd3e6 #9ebcda #8c96c6
    // #8c6bb1 #88419d #810f7c #4d004b
    BluePurple,
    // #f7fcf0 #e0f3db #ccebc5 #a8ddb5 #7bccc4
    // #4eb3d3 #2b8cbe #0868ac #084081
    GreenBlue,
    // #f4d166 #d5ca60 #b6c35c #98bb59 #7cb257 #60a656
    // #4b9c53 #3f8f4f #33834a #257740 #146c36
    GoldGreen,
    // #f4d166 #f8be5c #f8aa4c #f5983b #f3852a #ef701b
    // #e2621f #d65322 #c54923 #b14223 #9e3a26
    GoldOrange,
    // #f4d166 #f6be59 #f9aa51 #fc964e #f6834b #ee734a
    // #e56249 #db5247 #cf4244 #c43141 #b71d3e
    GoldRed,
    // #fff7ec #fee8c8 #fdd49e #fdbb84 #fc8d59
    // #ef6548 #d7301f #b30000 #7f0000
    OrangeRed,
    // #fff7fb #ece7f2 #d0d1e6 #a6bddb #74a9cf
    // #3690c0 #0570b0 #045a8d #023858
    PurpleBlue,
    // #fff7fb #ece2f0 #d0d1e6 #a6bddb #67a9cf
    // #3690c0 #02818a #016c59 #014636
    PurpleBlueGreen,
    // #f7f4f9 #e7e1ef #d4b9da #c994c7 #df65b0
    // #e7298a #ce1256 #980043 #67001f]
    PurpleRed,
    // #fff7f3 #fde0dd #fcc5c0 #fa9fb5 #f768a1
    // #dd3497 #ae017e #7a0177 #49006a
    RedPurple,
    // #ffffe5 #f7fcb9 #d9f0a3 #addd8e #78c679
    // #41ab5d #238443 #006837 #004529
    YellowGreen,
    // #ffffd9 #edf8b1 #c7e9b4 #7fcdbb #41b6c4
    // #1d91c0 #225ea8 #253494 #081d58
    YellowGreenBlue,
    // #ffffcc #ffeda0 #fed976 #feb24c #fd8d3c
    // #fc4e2a #e31a1c #bd0026 #800026
    YellowOrangeRed,
    // #ffffe5 #fff7bc #fee391 #fec44f #fe9929
    // #ec7014 #cc4c02 #993404 #662506
    YellowOrangeBrown,

    // #efe9e6, #e1dad7, #d5cbc8, #c8bdb9, #bbaea9
    // #cd967d, #dc7b43, #e15f19, #df4011, #dc000b
    LightGreyRed,

    // #e4eaea, #d6dcdd, #c8ced2, #b7c2c7, #a6b4bc
    // #64b0bf, #22a6c3, #2295c1, #1f85be, #1876bc
    LightGreyTeal,

    // #e0f1f2, #c4e9d0, #b0de9f, #d0e181, #f6e072,
    // #f6c053, #f3993e, #f77440, #ef4a3c,	
    LightMulti,

    // #f2e7da, #f7d5ba, #f9c499, #fab184, #fa9c73,
    // #f68967, #ef7860, #e8645b, #de515b, #d43d5b,
    LightOrange,

    // #e3e9e0, #c0dccf, #9aceca, #7abfc8, #59afc0,
    // #389fb9, #328dad, #2f7ca0, #276b95, #255988,
    LightTealBlue,

    // #323232, #2d4668, #1a5c93, #0074af, #008cbf,
    // #05a7ce, #25c0dd, #38daed, #50f3fa, #ffffff,
    DarkBlue,

    // #3c3c3c, #584b37, #725e34, #8c7631, #ae8b2b,
    // #cfa424, #ecc31e, #f9de30, #fff184, #ffffff,
    DarkGold,

    // #3a3a3a, #215748, #006f4d, #048942, #489e42
    // #76b340, #a6c63d, #d2d836, #ffeb2c, #ffffaa
    DarkGreen,

    // #373737, #1f5287, #197d8c, #29a869, #95ce3f,
    // #ffe800, #ffffff, 
    DarkMulti,

    // #343434, #703633, #9e3c38, #cc4037, #e75d1e
    // #ec8620, #eeab29, #f0ce32, #ffeb2c,
    DarkRed,

    // #f7fbff #deebf7 #c6dbef #9ecae1 #6baed6
    // #4292c6 #2171b5 #08519c #08306b
    Blues,
    // #eedbbd #ecca96 #e9b97a #e4a865 #dc9856
    // #d18954 #c7784c #c0673f #b85536 #ad4433 #9f3632
    Browns,
    // #f7fcf5 #e5f5e0 #c7e9c0 #a1d99b #74c476
    // #41ab5d #238b45 #006d2c #00441b
    Greens,
    // #ffffff #f0f0f0 #d9d9d9 #bdbdbd #969696
    // #737373 #525252 #252525 #000000
    Greys,
    // #fff5eb #fee6ce #fdd0a2 #fdae6b #fd8d3c
    // #f16913 #d94801 #a63603 #7f2704]
    Oranges,
    // #fcfbfd #efedf5 #dadaeb #bcbddc #9e9ac8
    // #807dba #6a51a3 #54278f #3f007d]
    Purples,
    // #fff5f0 #fee0d2 #fcbba1 #fc9272 #fb6a4a
    // #ef3b2c #cb181d #a50f15 #67000d
    Reds,
    // #bbdfdf #a2d4d5 #8ac9c9 #75bcbb #61b0af
    // #4da5a4 #379998 #2b8b8c #1e7f7f #127273
    Teals,
    // #bce4d8 #9dd3d1 #81c3cb #65b3c2 #45a2b9
    // #368fae #347da0 #306a93 #2c5985
    TealBlues,
    // #dcd4d0 #cec5c1 #c0b8b4 #b3aaa7 #a59c99
    // #98908c #8b827f #7e7673 #726866 #665c5a #59504e
    WarmGreys,

    // #002051 #1c3c6e #51586e #797673 #9a9478 #c5b66d #f3da4f
    Cividis,
    // #000004 #330a5f #781c6d #bb3755 #ed6925 #fcb519 #fcffa4
    Inferno,
    // #000004 #2c1160 #721f81 #b6377a #f1605d #feaf78 #fcfdbf
    Magma,
    // #0d0887 #5d01a6 #9c179e #cb4779 #ed7953 #fdb32f #f0f921
    Plasma,
    // #440154 #443a83 #31688e #21918d #35b779 #8fd744 #fde725
    Viridis,
}

impl From<Sequential> for Palette {
    fn from(value: Sequential) -> Self {
        match value {
            // Brewer
            Sequential::YellowGreen => {
                Palette::from(&BrewerSequential::YL_GN)
            }
            Sequential::YellowGreenBlue => {
                Palette::from(&BrewerSequential::YL_GN_BU)
            }
            Sequential::GreenBlue => {
                Palette::from(&BrewerSequential::GN_BU)
            }
            Sequential::BlueGreen => {
                Palette::from(&BrewerSequential::BU_GN)
            }
            Sequential::PurpleBlue => {
                Palette::from(&BrewerSequential::PU_BU)
            }
            Sequential::PurpleBlueGreen => {
                Palette::from(&BrewerSequential::PU_BU_GN)
            }
            Sequential::BluePurple => {
                Palette::from(&BrewerSequential::BU_PU)
            }
            Sequential::RedPurple => {
                Palette::from(&BrewerSequential::RD_PU)
            }
            Sequential::PurpleRed => {
                Palette::from(&BrewerSequential::PU_RD)
            }
            Sequential::OrangeRed => {
                Palette::from(&BrewerSequential::OR_RD)
            }
            Sequential::YellowOrangeRed => {
                Palette::from(&BrewerSequential::YL_OR_RD)
            }
            Sequential::YellowOrangeBrown => {
                Palette::from(&BrewerSequential::YL_OR_BR)
            }
            Sequential::Blues => {
                Palette::from(&BrewerSequential::BLUES)
            }
            Sequential::Purples => {
                Palette::from(&BrewerSequential::PURPLES)
            }
            Sequential::Greens => {
                Palette::from(&BrewerSequential::GREENS)
            }
            Sequential::Oranges => {
                Palette::from(&BrewerSequential::ORANGES)
            }
            Sequential::Reds => {
                Palette::from(&BrewerSequential::REDS)
            }
            Sequential::Greys => {
                Palette::from(&BrewerSequential::GREYS)
            }

            // Vega
            Sequential::GoldGreen => {
                Palette::from(&Vega::GOLD_GREEN)
            }
            Sequential::GoldOrange => {
                Palette::from(&Vega::GOLD_ORANGE)
            }
            Sequential::GoldRed => {
                Palette::from(&Vega::GOLD_RED)
            }
            Sequential::LightGreyRed => {
                Palette::from(&Vega::LIGHT_GREY_RED)
            }
            Sequential::LightGreyTeal => {
                Palette::from(&Vega::LIGHT_GREY_TEAL)
            }
            Sequential::LightMulti => {
                Palette::from(&Vega::LIGHT_MULTI)
            }
            Sequential::LightOrange => {
                Palette::from(&Vega::LIGHT_ORANGE)
            }
            Sequential::LightTealBlue => {
                Palette::from(&Vega::LIGHT_TEAL_BLUE)
            }
            Sequential::DarkBlue => {
                Palette::from(&Vega::DARK_BLUE)
            }
            Sequential::DarkGold => {
                Palette::from(&Vega::DARK_GOLD)
            }
            Sequential::DarkGreen => {
                Palette::from(&Vega::DARK_GREEN)
            }
            Sequential::DarkMulti => {
                Palette::from(&Vega::DARK_MULTI)
            }
            Sequential::DarkRed => {
                Palette::from(&Vega::DARK_RED)
            }

            // unicolor
            Sequential::Browns => {
                Palette::from(&Vega::BROWNS)
            }
            Sequential::TealBlues => {
                Palette::from(&Vega::TEAL_BLUES)
            }
            Sequential::Teals => {
                Palette::from(&Vega::TEALS)
            }
            Sequential::WarmGreys => {
                Palette::from(&Vega::WARM_GREYS)
            }

            // isoluminant
            Sequential::Cividis => {
                Palette::from(&Vega::CIVIDIS)
            }
            Sequential::Inferno => {
                Palette::from(&Vega::INFERNO)
            }
            Sequential::Magma => {
                Palette::from(&Vega::MAGMA)
            }
            Sequential::Plasma => {
                Palette::from(&Vega::PLASMA)
            }
            Sequential::Viridis => {
                Palette::from(&Vega::VIRIDIS)
            }
        }
    }
}

impl From<Sequential> for ColorMap {
    fn from(value: Sequential) -> Self {
        ColorMap::from(Palette::from(value))
    }
}
