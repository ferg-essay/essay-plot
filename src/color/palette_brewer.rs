
// Cynthia Brewer https://colorbrewer2.org/
pub(super) struct BrewerSequential;

impl BrewerSequential {
    /// <div style="background-color: #ffffe5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f7fcb9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d9f0a3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #addd8e; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #78c679; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #41ab5d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #238443; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #006837; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #004529; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YL_GN : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xf7fcb9, 0xaddd8e, 0x31a354],
        [0xffffcc, 0xc2e699, 0x78c679, 0x238443],
        [0xffffcc, 0xc2e699, 0x78c679, 0x31a354, 0x006837],
        [0xffffcc, 0xd9f0a3, 0xaddd8e, 0x78c679, 0x31a354, 0x006837],
        [0xffffcc, 0xd9f0a3, 0xaddd8e, 0x78c679, 0x41ab5d, 0x238443, 0x005a32],
        [0xffffe5, 0xf7fcb9, 0xd9f0a3, 0xaddd8e, 0x78c679, 0x41ab5d, 0x238443, 0x005a32],
        [0xffffe5, 0xf7fcb9, 0xd9f0a3, 0xaddd8e, 0x78c679, 0x41ab5d, 0x238443, 0x006837, 0x004529]
    );

    /// <div style="background-color: #ffffd9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #edf8b1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c7e9b4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7fcdbb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #41b6c4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #1d91c0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #225ea8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #253494; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #081d58; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YL_GN_BU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xedf8b1, 0x7fcdbb, 0x2c7fb8],
        [0xffffcc, 0xa1dab4, 0x41b6c4, 0x225ea8],
        [0xffffcc, 0xa1dab4, 0x41b6c4, 0x2c7fb8, 0x253494],
        [0xffffcc, 0xc7e9b4, 0x7fcdbb, 0x41b6c4, 0x2c7fb8, 0x253494],
        [0xffffcc, 0xc7e9b4, 0x7fcdbb, 0x41b6c4, 0x1d91c0, 0x225ea8, 0x0c2c84],
        [0xffffd9, 0xedf8b1, 0xc7e9b4, 0x7fcdbb, 0x41b6c4, 0x1d91c0, 0x225ea8, 0x0c2c84],
        [0xffffd9, 0xedf8b1, 0xc7e9b4, 0x7fcdbb, 0x41b6c4, 0x1d91c0, 0x225ea8, 0x253494, 0x081d58]
    );

    /// <div style="background-color: #f7fcf0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e0f3db; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ccebc5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a8ddb5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7bccc4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4eb3d3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #2b8cbe; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #0868ac; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #084081; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const GN_BU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xe0f3db, 0xa8ddb5, 0x43a2ca],
        [0xf0f9e8, 0xbae4bc, 0x7bccc4, 0x2b8cbe],
        [0xf0f9e8, 0xbae4bc, 0x7bccc4, 0x43a2ca, 0x0868ac],
        [0xf0f9e8, 0xccebc5, 0xa8ddb5, 0x7bccc4, 0x43a2ca, 0x0868ac],
        [0xf0f9e8, 0xccebc5, 0xa8ddb5, 0x7bccc4, 0x4eb3d3, 0x2b8cbe, 0x08589e],
        [0xf7fcf0, 0xe0f3db, 0xccebc5, 0xa8ddb5, 0x7bccc4, 0x4eb3d3, 0x2b8cbe, 0x08589e],
        [0xf7fcf0, 0xe0f3db, 0xccebc5, 0xa8ddb5, 0x7bccc4, 0x4eb3d3, 0x2b8cbe, 0x0868ac, 0x084081]
    );

    /// <div style="background-color: #f7fcfd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e5f5f9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ccece6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #99d8c9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #66c2a4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #41ae76; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #238b45; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #006d2c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #00441b; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BU_GN : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xe5f5f9, 0x99d8c9, 0x2ca25f],
        [0xedf8fb, 0xb2e2e2, 0x66c2a4, 0x238b45],
        [0xedf8fb, 0xb2e2e2, 0x66c2a4, 0x2ca25f, 0x006d2c],
        [0xedf8fb, 0xccece6, 0x99d8c9, 0x66c2a4, 0x2ca25f, 0x006d2c],
        [0xedf8fb, 0xccece6, 0x99d8c9, 0x66c2a4, 0x41ae76, 0x238b45, 0x005824],
        [0xf7fcfd, 0xe5f5f9, 0xccece6, 0x99d8c9, 0x66c2a4, 0x41ae76, 0x238b45, 0x005824],
        [0xf7fcfd, 0xe5f5f9, 0xccece6, 0x99d8c9, 0x66c2a4,0x41ae76, 0x238b45, 0x006d2c, 0x00441b]
    );

    /// <div style="background-color: #fff7fb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ece2f0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d0d1e6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a6bddb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #67a9cf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #3690c0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #02818a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #016c59; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #014636; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PU_BU_GN : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xece2f0, 0xa6bddb, 0x1c9099],
        [0xf6eff7, 0xbdc9e1, 0x67a9cf, 0x02818a],
        [0xf6eff7, 0xbdc9e1, 0x67a9cf, 0x1c9099, 0x016c59],
        [0xf6eff7, 0xd0d1e6, 0xa6bddb, 0x67a9cf, 0x1c9099, 0x016c59],
        [0xf6eff7, 0xd0d1e6, 0xa6bddb, 0x67a9cf, 0x3690c0, 0x02818a, 0x016450],
        [0xfff7fb, 0xece2f0, 0xd0d1e6, 0xa6bddb, 0x67a9cf, 0x3690c0, 0x02818a, 0x016450],
        [0xfff7fb, 0xece2f0, 0xd0d1e6, 0xa6bddb, 0x67a9cf, 0x3690c0, 0x02818a, 0x016c59, 0x014636]
    );

    /// <div style="background-color: #fff7fb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ece7f2; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d0d1e6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a6bddb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #74a9cf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #3690c0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #0570b0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #045a8d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #023858; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PU_BU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xece7f2, 0xa6bddb, 0x2b8cbe],
        [0xf1eef6, 0xbdc9e1, 0x74a9cf, 0x0570b0],
        [0xf1eef6, 0xbdc9e1, 0x74a9cf, 0x2b8cbe, 0x045a8d],
        [0xf1eef6, 0xd0d1e6, 0xa6bddb, 0x74a9cf, 0x2b8cbe, 0x045a8d],
        [0xf1eef6, 0xd0d1e6, 0xa6bddb, 0x74a9cf, 0x3690c0, 0x0570b0, 0x034e7b],
        [0xfff7fb, 0xece7f2, 0xd0d1e6, 0xa6bddb, 0x74a9cf, 0x3690c0, 0x0570b0, 0x034e7b],
        [0xfff7fb, 0xece7f2, 0xd0d1e6, 0xa6bddb, 0x74a9cf, 0x3690c0, 0x0570b0, 0x045a8d, 0x023858]
    );

    /// <div style="background-color: #f7fcfd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e0ecf4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bfd3e6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #9ebcda; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #8c96c6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #8c6bb1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #88419d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #810f7c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4d004b; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BU_PU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xe0ecf4, 0x9ebcda, 0x8856a7],
        [0xedf8fb, 0xb3cde3, 0x8c96c6, 0x88419d],
        [0xedf8fb, 0xb3cde3, 0x8c96c6, 0x8856a7, 0x810f7c],
        [0xedf8fb, 0xbfd3e6, 0x9ebcda, 0x8c96c6, 0x8856a7, 0x810f7c],
        [0xedf8fb, 0xbfd3e6, 0x9ebcda, 0x8c96c6, 0x8c6bb1, 0x88419d, 0x6e016b],
        [0xf7fcfd, 0xe0ecf4, 0xbfd3e6, 0x9ebcda, 0x8c96c6, 0x8c6bb1, 0x88419d, 0x6e016b],
        [0xf7fcfd, 0xe0ecf4, 0xbfd3e6, 0x9ebcda, 0x8c96c6, 0x8c6bb1, 0x88419d, 0x810f7c, 0x4d004b]
    );

    /// <div style="background-color: #fff7f3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fde0dd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fcc5c0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fa9fb5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f768a1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #dd3497; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ae017e; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7a0177; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #49006a; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RD_PU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xfde0dd, 0xfa9fb5, 0xc51b8a],
        [0xfeebe2, 0xfbb4b9, 0xf768a1, 0xae017e],
        [0xfeebe2, 0xfbb4b9, 0xf768a1, 0xc51b8a, 0x7a0177],
        [0xfeebe2, 0xfcc5c0, 0xfa9fb5, 0xf768a1, 0xc51b8a, 0x7a0177],
        [0xfeebe2, 0xfcc5c0, 0xfa9fb5, 0xf768a1, 0xdd3497, 0xae017e, 0x7a0177],
        [0xfff7f3, 0xfde0dd, 0xfcc5c0, 0xfa9fb5, 0xf768a1, 0xdd3497, 0xae017e, 0x7a0177],
        [0xfff7f3, 0xfde0dd, 0xfcc5c0, 0xfa9fb5, 0xf768a1, 0xdd3497, 0xae017e, 0x7a0177, 0x49006a]
    );

    /// <div style="background-color: #f7f4f9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e7e1ef; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d4b9da; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c994c7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #df65b0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e7298a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ce1256; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #980043; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #67001f; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PU_RD : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xe7e1ef, 0xc994c7, 0xdd1c77],
        [0xf1eef6, 0xd7b5d8, 0xdf65b0, 0xce1256],
        [0xf1eef6, 0xd7b5d8, 0xdf65b0, 0xdd1c77, 0x980043],
        [0xf1eef6, 0xd4b9da, 0xc994c7, 0xdf65b0, 0xdd1c77, 0x980043],
        [0xf1eef6, 0xd4b9da, 0xc994c7, 0xdf65b0, 0xe7298a, 0xce1256, 0x91003f],
        [0xf7f4f9, 0xe7e1ef, 0xd4b9da, 0xc994c7, 0xdf65b0, 0xe7298a, 0xce1256, 0x91003f],
        [0xf7f4f9, 0xe7e1ef, 0xd4b9da, 0xc994c7, 0xdf65b0, 0xe7298a, 0xce1256, 0x980043, 0x67001f]
    );

    /// <div style="background-color: #fff7ec; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee8c8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdd49e; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdbb84; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fc8d59; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ef6548; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d7301f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b30000; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7f0000; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const OR_RD : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xfee8c8, 0xfdbb84, 0xe34a33],
        [0xfef0d9, 0xfdcc8a, 0xfc8d59, 0xd7301f],
        [0xfef0d9, 0xfdcc8a, 0xfc8d59, 0xe34a33, 0xb30000],
        [0xfef0d9, 0xfdd49e, 0xfdbb84, 0xfc8d59, 0xe34a33, 0xb30000],
        [0xfef0d9, 0xfdd49e, 0xfdbb84, 0xfc8d59, 0xef6548, 0xd7301f, 0x990000],
        [0xfff7ec, 0xfee8c8, 0xfdd49e, 0xfdbb84, 0xfc8d59, 0xef6548, 0xd7301f, 0x990000],
        [0xfff7ec, 0xfee8c8, 0xfdd49e, 0xfdbb84, 0xfc8d59, 0xef6548, 0xd7301f, 0xb30000, 0x7f0000]
    );

    /// <div style="background-color: #ffffcc; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffeda0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fed976; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #feb24c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fd8d3c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fc4e2a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e31a1c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bd0026; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #800026; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YL_OR_RD : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xffeda0, 0xfeb24c, 0xf03b20],
        [0xffffb2, 0xfecc5c, 0xfd8d3c, 0xe31a1c],
        [0xffffb2, 0xfecc5c, 0xfd8d3c, 0xf03b20, 0xbd0026],
        [0xffffb2, 0xfed976, 0xfeb24c, 0xfd8d3c, 0xf03b20, 0xbd0026],
        [0xffffb2, 0xfed976, 0xfeb24c, 0xfd8d3c, 0xfc4e2a, 0xe31a1c, 0xb10026],
        [0xffffcc, 0xffeda0, 0xfed976, 0xfeb24c, 0xfd8d3c, 0xfc4e2a, 0xe31a1c, 0xb10026],
        [0xffffcc, 0xffeda0, 0xfed976, 0xfeb24c, 0xfd8d3c, 0xfc4e2a, 0xe31a1c, 0xbd0026, 0x800026]
    );

    /// <div style="background-color: #ffffe5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fff7bc; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee391; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fec44f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fe9929; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ec7014; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #cc4c02; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #993404; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #662506; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const YL_OR_BR : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xfff7bc, 0xfec44f, 0xd95f0e],
        [0xffffd4, 0xfed98e, 0xfe9929, 0xcc4c02],
        [0xffffd4, 0xfed98e, 0xfe9929, 0xd95f0e, 0x993404],
        [0xffffd4, 0xfee391, 0xfec44f, 0xfe9929, 0xd95f0e, 0x993404],
        [0xffffd4, 0xfee391, 0xfec44f, 0xfe9929, 0xec7014, 0xcc4c02, 0x8c2d04],
        [0xffffe5, 0xfff7bc, 0xfee391, 0xfec44f, 0xfe9929, 0xec701f, 0xcc4c02, 0x8c2d04],
        [0xffffe5, 0xfff7bc, 0xfee391, 0xfec44f, 0xfe9929, 0xec7014, 0xcc4c02, 0x993404, 0x662506]
    );

    /// <div style="background-color: #fcfbfd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #efedf5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #dadaeb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bdbddc; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #9e9ac8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #807dba; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #6a51a3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #54278f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #3f007d; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PURPLES : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xefedf5, 0xbcbddc, 0x756bb1],
        [0xf2f0f7, 0xcbc9e2, 0x9e9ac8, 0x6a51a3],
        [0xf2f0f7, 0xcbc9e2, 0x9e9ac8, 0x756bb1, 0x54278f],
        [0xf2f0f7, 0xdadaeb, 0xbcbddc, 0x9e9ac8, 0x756bb1, 0x54278f],
        [0xf2f0f7, 0xdadaeb, 0xbcbddc, 0x9e9ac8, 0x807dba, 0x6a51a3, 0x4a1486],
        [0xfcfbfd, 0xefedf5, 0xdadaeb, 0xbcbddc, 0x9e9ac8, 0x807dba, 0x6a51a3, 0x4a1486],
        [0xfcfbfd, 0xefedf5, 0xdadaeb, 0xbcbddc, 0x9e9ac8, 0x807dba, 0x6a51a3, 0x54278f, 0x3f007d]
    );

    /// <div style="background-color: #f7fbff; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #deebf7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c6dbef; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #9ecae1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #6baed6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4292c6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #2171b5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #08519c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #08306b; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BLUES : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xdeebf7, 0x9ecae1, 0x3182bd],
        [0xeff3ff, 0xbdd7e7, 0x6baed6, 0x2171b5],
        [0xeff3ff, 0xbdd7e7, 0x6baed6, 0x3182bd, 0x08519c],
        [0xeff3ff, 0xc6dbef, 0x9ecae1, 0x6baed6, 0x3182bd, 0x08519c],
        [0xeff3ff, 0xc6dbef, 0x9ecae1, 0x6baed6, 0x4292c6, 0x2171b5, 0x084594],
        [0xf7fbff, 0xdeebf7, 0xc6dbef, 0x9ecae1, 0x6baed6, 0x4292c6, 0x2171b5, 0x084594],
        [0xf7fbff, 0xdeebf7, 0xc6dbef, 0x9ecae1, 0x6baed6, 0x4292c6, 0x2171b5, 0x08519c, 0x08306b]
    );

    /// <div style="background-color: #f7fcf5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e5f5e0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c7e9c0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a1d99b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #74c476; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #41ab5d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #238b45; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #006d2c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #00441b; width: 10px; padding: 10px; border: 1px solid;"></div>
    const GREENS : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xe5f5e0, 0xa1d99b, 0x31a354],
        [0xedf8e9, 0xbae4b3, 0x74c476, 0x238b45],
        [0xedf8e9, 0xbae4b3, 0x74c476, 0x31a354, 0x006d2c],
        [0xedf8e9, 0xc7e9c0, 0xa1d99b, 0x74c476, 0x31a354, 0x006d2c],
        [0xedf8e9, 0xc7e9c0, 0xa1d99b, 0x74c476, 0x41ab5d, 0x238b45, 0x005a32],
        [0xf7fcf5, 0xe5f5e0, 0xc7e9c0, 0xa1d99b, 0x74c476, 0x41ab5d, 0x238b45, 0x005a32],
        [0xf7fcf5, 0xe5f5e0, 0xc7e9c0, 0xa1d99b, 0x74c476, 0x41ab5d, 0x238b45, 0x006d2c, 0x00441b]
    );

    /// <div style="background-color: #fff5eb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee6ce; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdd0a2; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdae6b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fd8d3c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f16913; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d94801; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a63603; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7f2704; width: 10px; padding: 10px; border: 1px solid;"></div>
    const ORANGES : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xfee6ce, 0xfdae6b, 0xe6550d],
        [0xfeedde, 0xfdbe85, 0xfd8d3c, 0xd94701],
        [0xfeedde, 0xfdbe85, 0xfd8d3c, 0xe6550d, 0xa63603],
        [0xfeedde, 0xfdd0a2, 0xfdae6b, 0xfd8d3c, 0xe6550d, 0xa63603],
        [0xfeedde, 0xfdd0a2, 0xfdae6b, 0xfd8d3c, 0xf16913, 0xd94801, 0x8c2d04],
        [0xfff5eb, 0xfee6ce, 0xfdd0a2, 0xfdae6b, 0xfd8d3c, 0xf16913, 0xd94801, 0x8c2d04],
        [0xfff5eb, 0xfee6ce, 0xfdd0a2, 0xfdae6b, 0xfd8d3c, 0xf16913, 0xd94801, 0xa63603, 0x7f2704]
    );

    /// <div style="background-color: #fff5f0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee0d2; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fcbba1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fc9272; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fb6a4a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ef3b2c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #cb181d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a50f15; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #67000d; width: 10px; padding: 10px; border: 1px solid;"></div>
    const REDS : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xfee0d2, 0xfc9272, 0xde2d26],
        [0xfee5d9, 0xfcae91, 0xfb6a4a, 0xcb181d],
        [0xfee5d9, 0xfcae91, 0xfb6a4a, 0xde2d26, 0xa50f15],
        [0xfee5d9, 0xfcbba1, 0xfc9272, 0xfb6a4a, 0xde2d26, 0xa50f15],
        [0xfee5d9, 0xfcbba1, 0xfc9272, 0xfb6a4a, 0xef3b2c, 0xcb181d, 0x99000d],
        [0xfff5f0, 0xfee0d2, 0xfcbba1, 0xfc9272, 0xfb6a4a, 0xef3b2c, 0xcb181d, 0x99000d],
        [0xfff5f0, 0xfee0d2, 0xfcbba1, 0xfc9272, 0xfb6a4a, 0xef3b2c, 0xcb181d, 0xa50f15, 0x67000d]
    );

    /// <div style="background-color: #ffffff; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f0f0f0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d9d9d9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bdbdbd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #969696; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #737373; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #525252; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #252525; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #000000; width: 10px; padding: 10px; border: 1px solid;"></div>
    const GREYS : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9]) = (
        [0xf0f0f0, 0xbdbdbd, 0x636363],
        [0xf7f7f7, 0xcccccc, 0x969696, 0x525252],
        [0xf7f7f7, 0xcccccc, 0x969696, 0x636363, 0x252525],
        [0xf7f7f7, 0xd9d9d9, 0xbdbdbd, 0x969696, 0x636363, 0x252525],
        [0xf7f7f7, 0xd9d9d9, 0xbdbdbd, 0x969696, 0x737373, 0x525252, 0x252525],
        [0xffffff, 0xf0f0f0, 0xd9d9d9, 0xbdbdbd, 0x969696, 0x737373, 0x525252, 0x252525],
        [0xffffff, 0xf0f0f0, 0xd9d9d9, 0xbdbdbd, 0x969696, 0x737373, 0x525252, 0x252525, 0x000000]
    );
}

// Cynthia Brewer https://colorbrewer2.org/
pub struct BrewerDiverging;

impl BrewerDiverging {
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
    pub const OR_PU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xf1a340, 0xf7f7f7, 0x998ec3],
        [0xe66101, 0xfdb863, 0xb2abd2, 0x5e3c99],
        [0xe66101, 0xfdb863, 0xf7f7f7, 0xb2abd2, 0x5e3c99],
        [0xb35806, 0xf1a340, 0xfee0b6, 0xd8daeb, 0x998ec3, 0x542788],
        [0xb35806, 0xf1a340, 0xfee0b6, 0xf7f7f7, 0xd8daeb, 0x998ec3, 0x542788],
        [0xb35806, 0xe08214, 0xfdb863, 0xfee0b6, 0xd8daeb, 0xb2abd2, 0x8073ac, 0x542788],
        [0xb35806, 0xe08214, 0xfdb863, 0xfee0b6, 0xf7f7f7, 0xd8daeb, 0xb2abd2, 0x8073ac, 0x542788],
        [0x7f3b08, 0xb35806, 0xe08214, 0xfdb863, 0xfee0b6, 0xd8daeb, 0xb2abd2, 0x8073ac, 0x542788, 0x2d004b],
        [0x7f3b08, 0xb35806, 0xe08214, 0xfdb863, 0xfee0b6, 0xf7f7f7, 0xd8daeb, 0xb2abd2, 0x8073ac, 0x542788, 0x2d004b]
    );

    /// <div style="background-color: #543005; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #8c510a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bf812d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #dfc27d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f6e8c3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f5f5f5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c7eae5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #80cdc1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #35978f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #01665e; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #003c30; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const BR_BG : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xd8b365, 0xf5f5f5, 0x5ab4ac],
        [0xa6611a, 0xdfc27d, 0x80cdc1, 0x018571],
        [0xa6611a, 0xdfc27d, 0xf5f5f5, 0x80cdc1, 0x018571],
        [0x8c510a, 0xd8b365, 0xf6e8c3, 0xc7eae5, 0x5ab4ac, 0x01665e],
        [0x8c510a, 0xd8b365, 0xf6e8c3, 0xf5f5f5, 0xc7eae5, 0x5ab4ac, 0x01665e],
        [0x8c510a, 0xbf812d, 0xdfc27d, 0xf6e8c3, 0xc7eae5, 0x80cdc1, 0x35978f, 0x01665e],
        [0x8c510a, 0xbf812d, 0xdfc27d, 0xf6e8c3, 0xf5f5f5, 0xc7eae5, 0x80cdc1, 0x35978f, 0x01665e],
        [0x543005, 0x8c510a, 0xbf812d, 0xdfc27d, 0xf6e8c3, 0xc7eae5, 0x80cdc1, 0x35978f, 0x01665e, 0x003c30],
        [0x543005, 0x8c510a, 0xbf812d, 0xdfc27d, 0xf6e8c3, 0xf5f5f5, 0xc7eae5, 0x80cdc1, 0x35978f, 0x01665e, 0x003c30]
    );

    /// <div style="background-color: #40004b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #762a83; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #9970ab; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c2a5cf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e7d4e8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f7f7f7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d9f0d3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a6dba0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #5aae61; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #1b7837; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #00441b; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PU_GN : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xaf8dc3, 0xf7f7f7, 0x7fbf7b],
        [0x7b3294, 0xc2a5cf, 0xa6dba0, 0x008837],
        [0x7b3294, 0xc2a5cf, 0xf7f7f7, 0xa6dba0, 0x008837],
        [0x762a83, 0xaf8dc3, 0xe7d4e8, 0xd9f0d3, 0x7fbf7b, 0x1b7837],
        [0x762a83, 0xaf8dc3, 0xe7d4e8, 0xf7f7f7, 0xd9f0d3, 0x7fbf7b, 0x1b7837],
        [0x762a83, 0x9970ab, 0xc2a5cf, 0xe7d4e8, 0xd9f0d3, 0xa6dba0, 0x5aae61, 0x1b7837],
        [0x762a83, 0x9970ab, 0xc2a5cf, 0xe7d4e8, 0xf7f7f7, 0xd9f0d3, 0xa6dba0, 0x5aae61, 0x1b7837],
        [0x40004b, 0x762a83, 0x9970ab, 0xc2a5cf, 0xe7d4e8, 0xd9f0d3, 0xa6dba0, 0x5aae61, 0x1b7837, 0x00441b],
        [0x40004b, 0x762a83, 0x9970ab, 0xc2a5cf, 0xe7d4e8, 0xf7f7f7, 0xd9f0d3, 0xa6dba0, 0x5aae61, 0x1b7837, 0x00441b]
    );

    /// <div style="background-color: #8e0152; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #c51b7d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #de77ae; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f1b6da; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fde0ef; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f7f7f7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e6f5d0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b8e186; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7fbc41; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4d9221; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #276419; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const PI_YG : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xe9a3c9, 0xf7f7f7, 0xa1d76a],
        [0xd01c8b, 0xf1b6da, 0xb8e186, 0x4dac26],
        [0xd01c8b, 0xf1b6da, 0xf7f7f7, 0xb8e186, 0x4dac26],
        [0xc51b7d, 0xe9a3c9, 0xfde0ef, 0xe6f5d0, 0xa1d76a, 0x4d9221],
        [0xc51b7d, 0xe9a3c9, 0xfde0ef, 0xf7f7f7, 0xe6f5d0, 0xa1d76a, 0x4d9221],
        [0xc51b7d, 0xde77ae, 0xf1b6da, 0xfde0ef, 0xe6f5d0, 0xb8e186, 0x7fbc41, 0x4d9221],
        [0xc51b7d, 0xde77ae, 0xf1b6da, 0xfde0ef, 0xf7f7f7, 0xe6f5d0, 0xb8e186, 0x7fbc41, 0x4d9221],
        [0x8e0152, 0xc51b7d, 0xde77ae, 0xf1b6da, 0xfde0ef, 0xe6f5d0, 0xb8e186, 0x7fbc41, 0x4d9221, 0x276419],
        [0x8e0152, 0xc51b7d, 0xde77ae, 0xf1b6da, 0xfde0ef, 0xf7f7f7, 0xe6f5d0, 0xb8e186, 0x7fbc41, 0x4d9221, 0x276419]
    );

    /// <div style="background-color: #67001f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b2182b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d6604d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f4a582; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fddbc7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f7f7f7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d1e5f0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #92c5de; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4393c3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #2166ac; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #053061; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RD_BU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xef8a62, 0xf7f7f7, 0x67a9cf],
        [0xca0020, 0xf4a582, 0x92c5de, 0x0571b0],
        [0xca0020, 0xf4a582, 0xf7f7f7, 0x92c5de, 0x0571b0],
        [0xb2182b, 0xef8a62, 0xfddbc7, 0xd1e5f0, 0x67a9cf, 0x2166ac],
        [0xb2182b, 0xef8a62, 0xfddbc7, 0xf7f7f7, 0xd1e5f0, 0x67a9cf, 0x2166ac],
        [0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xd1e5f0, 0x92c5de, 0x4393c3, 0x2166ac],
        [0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xf7f7f7, 0xd1e5f0, 0x92c5de, 0x4393c3, 0x2166ac],
        [0x67001f, 0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xd1e5f0, 0x92c5de, 0x4393c3, 0x2166ac, 0x053061],
        [0x67001f, 0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xf7f7f7, 0xd1e5f0, 0x92c5de, 0x4393c3, 0x2166ac, 0x053061]
    );

    /// <div style="background-color: #67001f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b2182b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d6604d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f4a582; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fddbc7; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffffff; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e0e0e0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bababa; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #878787; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4d4d4d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #1a1a1a; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RD_GY : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xef8a62, 0xffffff, 0x999999],
        [0xca0020, 0xf4a582, 0xbababa, 0x404040],
        [0xca0020, 0xf4a582, 0xffffff, 0xbababa, 0x404040],
        [0xb2182b, 0xef8a62, 0xfddbc7, 0xe0e0e0, 0x999999, 0x4d4d4d],
        [0xb2182b, 0xef8a62, 0xfddbc7, 0xffffff, 0xe0e0e0, 0x999999, 0x4d4d4d],
        [0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xe0e0e0, 0xbababa, 0x878787, 0x4d4d4d],
        [0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xffffff, 0xe0e0e0, 0xbababa, 0x878787, 0x4d4d4d],
        [0x67001f, 0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xe0e0e0, 0xbababa, 0x878787, 0x4d4d4d, 0x1a1a1a],
        [0x67001f, 0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xffffff, 0xe0e0e0, 0xbababa, 0x878787, 0x4d4d4d, 0x1a1a1a]
    );

    /// <div style="background-color: #a50026; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d73027; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f46d43; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdae61; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee090; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffffbf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e0f3f8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #abd9e9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #74add1; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4575b4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #313694; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RD_YL_BU : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xfc8d59, 0xffffbf, 0x91bfdb],
        [0xd7191c, 0xfdae61, 0xabd9e9, 0x2c7bb6],
        [0xd7191c, 0xfdae61, 0xffffbf, 0xabd9e9, 0x2c7bb6],
        [0xd73027, 0xfc8d59, 0xfee090, 0xe0f3f8, 0x91bfdb, 0x4575b4],
        [0xd73027, 0xfc8d59, 0xfee090, 0xffffbf, 0xe0f3f8, 0x91bfdb, 0x4575b4],
        [0xd73027, 0xf46d43, 0xfdae61, 0xfee090, 0xe0f3f8, 0xabd9e9, 0x74add1, 0x4575b4],
        [0xd73027, 0xf46d43, 0xfdae61, 0xfee090, 0xffffbf, 0xe0f3f8, 0xabd9e9, 0x74add1, 0x4575b4],
        [0xa50026, 0xd73027, 0xf46d43, 0xfdae61, 0xfee090, 0xe0f3f8, 0xabd9e9, 0x74add1, 0x4575b4, 0x313695],
        [0xa50026, 0xd73027, 0xf46d43, 0xfdae61, 0xfee090, 0xffffbf, 0xe0f3f8, 0xabd9e9, 0x74add1, 0x4575b4, 0x313695]
    );

    /// <div style="background-color: #a50026; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d73027; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f46d43; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdae61; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fee08b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffffbf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d9ef8b; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a6d96a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #66bd63; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #1a9850; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #006837; width: 10px; padding: 10px; border: 1px solid;"></div>
    pub const RD_YL_GN : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xfc8d59, 0xffffbf, 0x91cf60],
        [0xd7191c, 0xfdae61, 0xa6d96a, 0x1a9641],
        [0xd7191c, 0xfdae61, 0xffffbf, 0xa6d96a, 0x1a9641],
        [0xd73027, 0xfc8d59, 0xfee08b, 0xd9ef8b, 0x91cf60, 0x1a9850],
        [0xd73027, 0xfc8d59, 0xfee08b, 0xffffbf, 0xd9ef8b, 0x91cf60, 0x1a9850],
        [0xd73027, 0xf46d43, 0xfdae61, 0xfee08b, 0xd9ef8b, 0xa6d96a, 0x66bd63, 0x1a9850],
        [0xd73027, 0xf46d43, 0xfdae61, 0xfee08b, 0xffffbf, 0xd9ef8b, 0xa6d96a, 0x66bd63, 0x1a9850],
        [0xa50026, 0xd73027, 0xf46d43, 0xfdae61, 0xfee08b, 0xd9ef8b, 0xa6d96a, 0x66bd63, 0x1a9850, 0x006837],
        [0xa50026, 0xd73027, 0xf46d43, 0xfdae61, 0xfee08b, 0xffffbf, 0xd9ef8b, 0xa6d96a, 0x66bd63, 0x1a9850, 0x006837]
    );
    
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
    pub const SPECTRAL : ([u32; 3], [u32; 4], [u32; 5], [u32; 6], [u32; 7], [u32; 8], [u32; 9], [u32; 10], [u32; 11]) = (
        [0xfc8d59, 0xffffbf, 0x99d594],
        [0xd7191c, 0xfdae61, 0xabdda4, 0x2b83ba],
        [0xd7191c, 0xfdae61, 0xffffbf, 0xabdda4, 0x2b83ba],
        [0xd53e4f, 0xfc8d59, 0xfee08b, 0xe6f598, 0x99d594, 0x3288bd],
        [0xd53e4f, 0xfc8d59, 0xfee08b, 0xffffbf, 0xe6f598, 0x99d594, 0x3288bd],
        [0xd53e4f, 0xf46d43, 0xfdae61, 0xfee08b, 0xe6f598, 0xabdda4, 0x66c2a5, 0x3288bd],
        [0xd53e4f, 0xf46d43, 0xfdae61, 0xfee08b, 0xffffbf, 0xe6f598, 0xabdda4, 0x66c2a5, 0x3288bd],
        [0x9e0142, 0xd53e4f, 0xf46d43, 0xfdae61, 0xfee08b, 0xe6f598, 0xabdda4, 0x66c2a5, 0x3288bd, 0x5e4fa2],
        [0x9e0142, 0xd53e4f, 0xf46d43, 0xfdae61, 0xfee08b, 0xffffbf, 0xe6f598, 0xabdda4, 0x66c2a5, 0x3288bd, 0x5e4fa2]    
    );
}

// Cynthia Brewer https://colorbrewer2.org/
pub struct BrewerQualitative;

impl BrewerQualitative {
    /// <div style="background-color: #7fc97f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #beaed4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdc086; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffff99; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #386cb0; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f0027f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #bf5b17; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #666666; width: 10px; padding: 10px; border: 1px solid;"></div>
    const ACCENT: [u32; 8] = [
        0x7fc97f, 0xbeaed4, 0xfdc086, 0xffff99, 0x386cb0, 0xf0027f, 0xbf5b17, 0x666666
    ];

    /// <div style="background-color: #1b9e77; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #d95f02; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #7570b3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e7298a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #66a61e; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e6ab02; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a6761d; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #666666; width: 10px; padding: 10px; border: 1px solid;"></div>
    const DARK2: [u32; 8] = [
        0x1b9e77, 0xd95f02, 0x7570b3,0xe7298a, 0x66a61e, 0xe6ab02, 0xa6761d, 0x666666
    ];

    /// <div style="background-color: #a6cee3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #1f78b4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b2df8a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #33a02c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fb9a99; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e31a1c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdbf6f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ff7f00; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #cab2d6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #6a3d9a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffff99; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b15928; width: 10px; padding: 10px; border: 1px solid;"></div>
    const PAIRED: [u32; 12] = [
        0xa6cee3, 0x1f78b4, 0xb2df8a, 0x33a02c, 0xfb9a99, 0xe31a1c, 0xfdbf6f, 0xff7f00, 0xcab2d6, 0x6a3d9a, 0xffff99, 0xb15928
    ];

    /// <div style="background-color: #fbb4ae; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b3cde3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ccebc5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #decbe4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fed9a6; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffffcc; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e5d8bd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fddaec; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f2f2f2; width: 10px; padding: 10px; border: 1px solid;"></div>
    const PASTEL1: [u32; 9] = [
        0xfbb4ae, 0xb3cde3, 0xccebc5, 0xdecbe4, 0xfed9a6, 0xffffcc, 0xe5d8bd, 0xfddaec, 0xf2f2f2
    ];

    /// <div style="background-color: #b3e2cd; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fdcdac; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #cbd5e8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f4cae4; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e6f5c9; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fff2ae; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f1e2cc; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #cccccc; width: 10px; padding: 10px; border: 1px solid;"></div>
    const PASTEL2: [u32; 8] = [
        0xb3e2cd, 0xfdcdac, 0xcbd5e8, 0xf4cae4, 0xe6f5c9, 0xfff2ae, 0xf1e2cc, 0xcccccc
    ];

    /// <div style="background-color: #e41a1c; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #377eb8; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #4daf4a; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #984ea3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ff7f00; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffff33; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a65628; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #f781bf; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #999999; width: 10px; padding: 10px; border: 1px solid;"></div>
    const SET1: [u32; 9] = [
        0xe41a1c, 0x377eb8, 0x4daf4a, 0x984ea3, 0xff7f00, 0xffff33, 0xa65628, 0xf781bf, 0x999999
    ];

    /// <div style="background-color: #66c2a5; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #fc8d62; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #8da0cb; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e78ac3; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #a6d854; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #ffd92f; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #e5c494; width: 10px; padding: 10px; border: 1px solid;"></div>
    /// <div style="background-color: #b3b3b3; width: 10px; padding: 10px; border: 1px solid;"></div>
    const SET2: [u32; 8] = [
        0x66c2a5, 0xfc8d62, 0x8da0cb, 0xe78ac3, 0xa6d854, 0xffd92f, 0xe5c494, 0xb3b3b3
    ];

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
    pub const SET3: [u32; 12] = [
        0x8dd3c7, 0xffffb3, 0xbebada, 0xfb8072, 0x80b1d3, 0xfdb462, 0xb3de69, 0xfccde5, 0xd9d9d9, 0xbc80bd, 0xccebc5, 0xffed6f
    ];
}

    
