// https://github.com/vega/vega/blob/main/packages/vega-scale/src/palettes.js
pub(super) struct Vega;

impl Vega {
    // <div style="background-color: #4c78a8"></div>
    // <div style="background-color: #f58518"></div>
    // <div style="background-color: #e45756"></div>
    // <div style="background-color: #72b7b2"></div>
    // <div style="background-color: #54a24b"></div>
    // <div style="background-color: #eeca3b"></div>
    // <div style="background-color: #b279a2"></div>
    // <div style="background-color: #ff9da6"></div>
    // <div style="background-color: #9d755d"></div>
    // <div style="background-color: #bab0ac"></div>
    const TABLEAU_10: [u32; 10] = [
        0x4c78a8, 0xf58518, 0xe45756, 0x72b7b2, 0x54a24b,
        0xeeca3b, 0xb279a2, 0xff9da6, 0x9d755d, 0xbab0ac,
    ];
    
    /// <div style="background-color: #4c78a8"></div>
    /// <div style="background-color: #93cae9"></div>
    /// <div style="background-color: #f58518"></div>
    /// <div style="background-color: #ffbf79"></div>
    /// <div style="background-color: #54a24b"></div>
    /// <div style="background-color: #88d27a"></div>
    /// <div style="background-color: #b79a20"></div>
    /// <div style="background-color: #f2cf5b"></div>
    /// <div style="background-color: #439894"></div>
    /// <div style="background-color: #83bcb6"></div>
    /// <div style="background-color: #e45756"></div>
    /// <div style="background-color: #ff9d98"></div>
    /// <div style="background-color: #79706e"></div>
    /// <div style="background-color: #bab0ae"></div>
    /// <div style="background-color: #d67195"></div>
    /// <div style="background-color: #fcbfd2"></div>
    /// <div style="background-color: #b279a2"></div>
    /// <div style="background-color: #d6a5c9"></div>
    /// <div style="background-color: #9e765f"></div>
    /// <div style="background-color: #d8b5a5"></div>
    const TABLEAU_20 : [u32; 20] = [
        0x4c78a8, 0x9ecae9, 0xf58518, 0xffbf79, 0x54a24b,
        0x88d27a, 0xb79a20, 0xf2cf5b, 0x439894, 0x83bcb6,
        0xe45756, 0xff9d98, 0x79706e, 0xbab0ac, 0xd67195,
        0xfcbfd2, 0xb279a2, 0xd6a5c9, 0x9e765f, 0xd8b5a5,
    ];

    /// <div style="background-color: #1f77b4"></div>
    /// <div style="background-color: #ff7f0e"></div>
    /// <div style="background-color: #2ca02c"></div>
    /// <div style="background-color: #d62728ff"></div>
    /// <div style="background-color: #9467bdff"></div>
    /// <div style="background-color: #8c564b"></div>
    /// <div style="background-color: #e377c2"></div>
    /// <div style="background-color: #7f7f7f"></div>
    /// <div style="background-color: #bcbd22"></div>
    /// <div style="background-color: #f17bec"></div>
    const TABLEAU_A : [u32; 10] = [
        0x1f77b4, 0xff7f0e, 0x2ca02c, 0xd62728, 0x9467bd,
        0x8c564b, 0xe377c2, 0x7f7f7f, 0xbcbd22, 0xf17bec,
    ];
    
    // <div style="background-color: #4e79a7"></div>
    // <div style="background-color: #f28e2c"></div>
    // <div style="background-color: #e15759"></div>
    // <div style="background-color: #76b7b2"></div>
    // <div style="background-color: #59a14f"></div>
    // <div style="background-color: #edc949"></div>
    // <div style="background-color: #af7aa1"></div>
    // <div style="background-color: #ff9da7"></div>
    // <div style="background-color: #9c7557"></div>
    // <div style="background-color: #bab0ab"></div>
    pub const TABLEAU_B: [u32; 10] = [
        0x4e79a7, 0xf28e2c, 0xe15759, 0x76b7b2, 0x59a14f,
        0xedc949, 0xaf7aa1, 0xff9da7, 0x9c755f, 0xbab0ab,
    ];
    
    // <div style="background-color: #1f77b4"></div>
    // <div style="background-color: #ff7f0e"></div>
    // <div style="background-color: #2ca02c"></div>
    // <div style="background-color: #d62728"></div>
    // <div style="background-color: #9467bd"></div>
    // <div style="background-color: #8c564b"></div>
    // <div style="background-color: #e377c2"></div>
    // <div style="background-color: #7f7f7f"></div>
    // <div style="background-color: #bcbd22"></div>
    // <div style="background-color: #17becf"></div>
    const CATEGORY_10: [u32; 10] = [
        0x1f77b4, 0xff7f0e, 0x2ca02c, 0xd62728, 0x9467bd,
        0x8c564b, 0xe377c2, 0x7f7f7f, 0xbcbd22, 0x17becf,
    ];
    
    // <div style="background-color: #4269d0"></div>
    // <div style="background-color: #efb118"></div>
    // <div style="background-color: #ff725c"></div>
    // <div style="background-color: #6cc5b0"></div>
    // <div style="background-color: #3ca951"></div>
    // <div style="background-color: #ff8ab7"></div>
    // <div style="background-color: #a463f2"></div>
    // <div style="background-color: #97bbf5"></div>
    // <div style="background-color: #9c6b4e"></div>
    // <div style="background-color: #9498a0"></div>
    const OBSERVABLE_10: [u32; 10] = [
        0x4269d0, 0xefb118, 0xff725c, 0x6cc5b0, 0x3ca951,
        0xff8ab7, 0xa463f2, 0x97bbf5, 0x9c6b4e, 0x9498a0,
    ];
    
    // <div style="background-color: #f17bec"></div>
    // <div style="background-color: #aec7e8"></div>
    // <div style="background-color: #ff7f0eff"></div>
    // <div style="background-color: #ffbb78"></div>
    // <div style="background-color: #2ca02c"></div>
    // <div style="background-color: #98df8a"></div>
    // <div style="background-color: #d62728"></div>
    // <div style="background-color: #ff9896"></div>
    // <div style="background-color: #9467bd"></div>
    // <div style="background-color: #c5b0d5"></div>
    // <div style="background-color: #8c564b"></div>
    // <div style="background-color: #c49c94"></div>
    // <div style="background-color: #e377c2"></div>
    // <div style="background-color: #f7b5d2"></div>
    // <div style="background-color: #7f7f7f"></div>
    // <div style="background-color: #c7c7c7"></div>
    // <div style="background-color: #bcbd22"></div>
    // <div style="background-color: #bdbd8d"></div>
    // <div style="background-color: #17becf"></div>
    // <div style="background-color: #9edae5"></div>
    const CATEGORY_20 : [u32; 20] = [
        0x1f77b4, 0xaec7e8, 0xff7f0e, 0xffbb78, 0x2ca02c, 
        0x98df8a, 0xd62728, 0xff9896, 0x9467bd, 0xc5b0d5, 
        0x8c564b, 0xc49c94, 0xe377c2, 0xf7b6d2, 0x7f7f7f, 
        0xc7c7c7, 0xbcbd22, 0xdbdb8d, 0x17becf, 0x9edae5,
    ];
    
    // <div style="background-color: #393b79"></div>
    // <div style="background-color: #5254a3"></div>
    // <div style="background-color: #6b6ecf"></div>
    // <div style="background-color: #9c9ede"></div>
    // <div style="background-color: #637939"></div>
    // <div style="background-color: #8ca252"></div>
    // <div style="background-color: #b5cf6b"></div>
    // <div style="background-color: #cedb9c"></div>
    // <div style="background-color: #8c6d31"></div>
    // <div style="background-color: #bd9e39"></div>
    // <div style="background-color: #e7ba52"></div>
    // <div style="background-color: #e7cb94"></div>
    // <div style="background-color: #843c39"></div>
    // <div style="background-color: #ad494a"></div>
    // <div style="background-color: #d6616b"></div>
    // <div style="background-color: #e7969c"></div>
    // <div style="background-color: #7b4173"></div>
    // <div style="background-color: #a55194"></div>
    // <div style="background-color: #ce6dbd"></div>
    // <div style="background-color: #de9ed6"></div>
    const CATEGORY_20B : [u32; 20] = [
        0x393b79, 0x5254a3, 0x6b6ecf, 0x9c9ede, 0x637939,
        0x8ca252, 0xb5cf6b, 0xcedb9c, 0x8c6d31, 0xbd9e39,
        0xe7ba52, 0xe7cb94, 0x843c39, 0xad494a, 0xd6616b,
        0xe7969c, 0x7b4173, 0xa55194, 0xce6dbd, 0xde9ed6,
    ];
    
    // <div style="background-color: #3182bd"></div>
    // <div style="background-color: #6baed6"></div>
    // <div style="background-color: #9ecae1"></div>
    // <div style="background-color: #c6dbef"></div>
    // <div style="background-color: #e6550d"></div>
    // <div style="background-color: #fd8d3c"></div>
    // <div style="background-color: #fdae6c"></div>
    // <div style="background-color: #fdd0a2"></div>
    // <div style="background-color: #31a354"></div>
    // <div style="background-color: #74c476"></div>
    // <div style="background-color: #a1d99b"></div>
    // <div style="background-color: #c7e9c0"></div>
    // <div style="background-color: #756bb1"></div>
    // <div style="background-color: #9e9ac8"></div>
    // <div style="background-color: #bcbddc"></div>
    // <div style="background-color: #dadaeb"></div>
    // <div style="background-color: #636363"></div>
    // <div style="background-color: #969696"></div>
    // <div style="background-color: #bdbdbd"></div>
    // <div style="background-color: #d9d9d9"></div>
    const CATEGORY_20C: [u32; 20] = [
        0x3182bd, 0x6baed6, 0x9ecae1, 0xc6dbef, 0xe6550d,
        0xfd8d3c, 0xfdae6b, 0xfdd0a2, 0x31a354, 0x74c476,
        0xa1d99b, 0xc7e9c0, 0x756bb1, 0x9e9ac8, 0xbcbddc,
        0xdadaeb, 0x636363, 0x969696, 0xbdbdbd, 0xd9d9d9,
    ];

    // #440154 #443a83 #31688e #21918d #35b779 #8fd744 #fde725
    const VIRIDIS: [u32; 31] = [
        0x440154, 0x470e61, 0x481a6c, 0x482575, 0x472f7d,
        0x443a83, 0x414487, 0x3d4e8a, 0x39568c, 0x35608d,
        0x31688e, 0x2d708e, 0x2a788e, 0x27818e, 0x23888e,
        0x21918d, 0x1f988b, 0x1fa088, 0x22a884, 0x2ab07f,
        0x35b779, 0x43bf71, 0x54c568, 0x66cc5d, 0x7ad151,
        0x8fd744, 0xa5db36, 0xbcdf27, 0xd2e21b, 0xe9e51a,
        0xfde725,
    ];

    // #440154 #443a83 #31688e #21918d #35b779 #8fd744 #fde725
    const MAGMA: [u32; 31] = [
        0x000004, 0x040413, 0x0b0924, 0x150e37, 0x20114b,
        0x2c1160, 0x3b0f70, 0x4a1079, 0x57157e, 0x651a80,
        0x721f81, 0x7f2482, 0x8c2981, 0x9a2e80, 0xa8327d,
        0xb6377a, 0xc43c75, 0xd1426f, 0xde4968, 0xe95462,
        0xf1605d, 0xf76f5c, 0xfa7f5e, 0xfc8f65, 0xfe9f6d,
        0xfeaf78, 0xfebf84, 0xfece91, 0xfddea0, 0xfcedaf,
        0xfcfdbf,
    ];

    // #000004 #330a5f #781c6d #bb3755 #ed6925 #fcb519 #fcffa4
    const INFERNO: [u32; 31] = [
        0x000004, 0x040313, 0x0c0826, 0x170c3b, 0x240c4f,
        0x330a5f, 0x420a68, 0x500d6c, 0x5d126e, 0x6b176e,
        0x781c6d, 0x86216b, 0x932667, 0xa12b62, 0xae305c,
        0xbb3755, 0xc73e4c, 0xd24644, 0xdd513a, 0xe65c30,
        0xed6925, 0xf3771a, 0xf8850f, 0xfb9506, 0xfca50a,
        0xfcb519, 0xfac62d, 0xf6d645, 0xf2e661, 0xf3f484,
        0xfcffa4,
    ];

    // #0d0887 #5d01a6 #9c179e #cb4779 #ed7953 #fdb32f #f0f921
    const PLASMA: [u32; 31] = [
        0x0d0887, 0x230690, 0x330597, 0x42039d, 0x5002a2,
        0x5d01a6, 0x6a00a8, 0x7801a8, 0x8405a7, 0x900da4,
        0x9c179e, 0xa72198, 0xb12a90, 0xba3488, 0xc33d80,
        0xcb4779, 0xd35171, 0xda5a69, 0xe16462, 0xe76e5b,
        0xed7953, 0xf2834c, 0xf68f44, 0xfa9a3d, 0xfca636,
        0xfdb32f, 0xfec029, 0xfcce25, 0xf9dc24, 0xf5ea27,
        0xf0f921,
    ];

    // #002051 #1c3c6e #51586e #797673 #9a9478 #c5b66d #f3da4f
    const CIVIDIS: [u32; 64] = [
        0x002051, 0x002358, 0x00265d, 0x002961, 0x012b65,
        0x042e67, 0x083169, 0x0d346b, 0x11366c, 0x16396d,
        0x1c3c6e, 0x213f6e, 0x26426e, 0x2c456e, 0x31476e,
        0x374a6e, 0x3c4d6e, 0x42506e, 0x47536d, 0x4c566d,
        0x51586e, 0x555b6e, 0x5a5e6e, 0x5e616e, 0x62646f,
        0x66676f, 0x6a6a70, 0x6e6d71, 0x727071, 0x757372,
        0x797673, 0x7c7974, 0x7f7c75, 0x827f75, 0x868276,
        0x898577, 0x8c8877, 0x908b78, 0x938e78, 0x969178,
        0x9a9478, 0x9e9778, 0xa19b78, 0xa59e77, 0xa9a177,
        0xaea575, 0xb2a874, 0xb6ab73, 0xbbaf71, 0xc0b26f,
        0xc5b66d, 0xc9b96a, 0xcebd68, 0xd3c065, 0xd8c462,
        0xddc85f, 0xe2cb5c, 0xe7cf58, 0xebd355, 0xf0d652,
        0xf3da4f, 0xf7de4c, 0xfae249, 0xfce647,	
    ];

    // #eedbbd #ecca96 #e9b97a #e4a865 #dc9856
    // #d18954 #c7784c #c0673f #b85536 #ad4433 #9f3632
    const BROWNS: [u32; 11] = [
        0xeedbbd, 0xecca96, 0xe9b97a, 0xe4a865, 0xdc9856,
        0xd18954, 0xc7784c, 0xc0673f, 0xb85536, 0xad4433,
        0x9f3632,
    ];

    // #bce4d8 #9dd3d1 #81c3cb #65b3c2 #45a2b9
    // #368fae #347da0 #306a93 #2c5985
    const TEAL_BLUES: [u32; 9] = [
        0xbce4d8, 0x9dd3d1, 0x81c3cb, 0x65b3c2, 0x45a2b9,
        0x368fae, 0x347da0, 0x306a93, 0x2c5985,
    ];

    // #bbdfdf #a2d4d5 #8ac9c9 #75bcbb #61b0af
    // #4da5a4 #379998 #2b8b8c #1e7f7f #127273
    const TEALS: [u32; 11] = [
        0xbbdfdf, 0xa2d4d5, 0x8ac9c9, 0x75bcbb, 0x61b0af,
        0x4da5a4, 0x379998, 0x2b8b8c, 0x1e7f7f, 0x127273,
        0x006667,
    ];

    // #dcd4d0 #cec5c1 #c0b8b4 #b3aaa7 #a59c99
    // #98908c #8b827f #7e7673 #726866 #665c5a #59504e
    const WARM_GREYS: [u32; 11] = [
        0xdcd4d0, 0xcec5c1, 0xc0b8b4, 0xb3aaa7, 0xa59c99,
        0x98908c, 0x8b827f, 0x7e7673, 0x726866, 0x665c5a,
        0x59504e,
    ];

    // #f4d166 #d5ca60 #b6c35c #98bb59 #7cb257
    // #60a656 #4b9c53 #3f8f4f #33834a #257740 #146c36
    const GOLD_GREEN: [u32; 11] = [
        0xf4d166, 0xd5ca60, 0xb6c35c, 0x98bb59, 0x7cb257,
        0x60a656, 0x4b9c53, 0x3f8f4f, 0x33834a, 0x257740,
        0x146c36,
    ];

    // #f4d166 #f8be5c, #f8aa4c, #f5983b, #f3852a,
    // #ef701b #e2621f, #d65322, #c54923, #b14223,
    // #9e3a26
    const GOLD_ORANGE: [u32; 11] = [
        0xf4d166, 0xf8be5c, 0xf8aa4c, 0xf5983b, 0xf3852a,
        0xef701b, 0xe2621f, 0xd65322, 0xc54923, 0xb14223,
        0x9e3a26,
    ];

    // #f4d166, #f6be59, #f9aa51, #fc964e, #f6834b,
    // #ee734a, #e56249, #db5247, #cf4244, #c43141,
    // #b71d3e,
    const GOLD_RED: [u32; 11] = [
        0xf4d166, 0xf6be59, 0xf9aa51, 0xfc964e, 0xf6834b,
        0xee734a, 0xe56249, 0xdb5247, 0xcf4244, 0xc43141,
        0xb71d3e,
    ];

    // #efe9e6, #e1dad7, #d5cbc8, #c8bdb9, #bbaea9
    // #cd967d, #dc7b43, #e15f19, #df4011, #dc000b
    const LIGHT_GREY_RED: [u32; 10] = [
        0xefe9e6, 0xe1dad7, 0xd5cbc8, 0xc8bdb9, 0xbbaea9,
        0xcd967d, 0xdc7b43, 0xe15f19, 0xdf4011, 0xdc000b,
    ];

    // #e4eaea, #d6dcdd, #c8ced2, #b7c2c7, #a6b4bc
    // #64b0bf, #22a6c3, #2295c1, #1f85be, #1876bc
    const LIGHT_GREY_TEAL: [u32; 10] = [
        0xe4eaea, 0xd6dcdd, 0xc8ced2, 0xb7c2c7, 0xa6b4bc,
        0x64b0bf, 0x22a6c3, 0x2295c1, 0x1f85be, 0x1876bc,
    ];

    // #e0f1f2, #c4e9d0, #b0de9f, #d0e181, #f6e072,
    // #f6c053, #f3993e, #f77440, #ef4a3c,	
    const LIGHT_MULTI: [u32; 9] = [
        0xe0f1f2, 0xc4e9d0, 0xb0de9f, 0xd0e181, 0xf6e072,
        0xf6c053, 0xf3993e, 0xf77440, 0xef4a3c,	
    ];

    // #f2e7da, #f7d5ba, #f9c499, #fab184, #fa9c73,
    // #f68967, #ef7860, #e8645b, #de515b, #d43d5b,
    const LIGHT_ORANGE: [u32; 10] = [
        0xf2e7da, 0xf7d5ba, 0xf9c499, 0xfab184, 0xfa9c73,
        0xf68967, 0xef7860, 0xe8645b, 0xde515b, 0xd43d5b,
    ];

    // #e3e9e0, #c0dccf, #9aceca, #7abfc8, #59afc0,
    // #389fb9, #328dad, #2f7ca0, #276b95, #255988,
    const LIGHT_TEAL_BLUE: [u32; 10] = [
        0xe3e9e0, 0xc0dccf, 0x9aceca, 0x7abfc8, 0x59afc0,
        0x389fb9, 0x328dad, 0x2f7ca0, 0x276b95, 0x255988,
    ];

    // #323232, #2d4668, #1a5c93, #0074af, #008cbf,
    // #05a7ce, #25c0dd, #38daed, #50f3fa, #ffffff,
    const DARK_BLUE: [u32; 10] = [
        0x323232, 0x2d4668, 0x1a5c93, 0x0074af, 0x008cbf,
        0x05a7ce, 0x25c0dd, 0x38daed, 0x50f3fa, 0xffffff,
    ];

    // #3c3c3c, #584b37, #725e34, #8c7631, #ae8b2b,
    // #cfa424, #ecc31e, #f9de30, #fff184, #ffffff,
    const DARK_GOLD: [u32; 10] = [
        0x3c3c3c, 0x584b37, 0x725e34, 0x8c7631, 0xae8b2b,
        0xcfa424, 0xecc31e, 0xf9de30, 0xfff184, 0xffffff,
    ];

    // #3a3a3a, #215748, #006f4d, #048942, #489e42
    // #76b340, #a6c63d, #d2d836, #ffeb2c, #ffffaa
    const DARK_GREEN: [u32; 10] = [
        0x3a3a3a, 0x215748, 0x006f4d, 0x048942, 0x489e42,
        0x76b340, 0xa6c63d, 0xd2d836, 0xffeb2c, 0xffffaa,
    ];

    // #373737, #1f5287, #197d8c, #29a869, #95ce3f,
    // #ffe800, #ffffff, 
    const DARK_MULTI: [u32; 7] = [
        0x373737, 0x1f5287, 0x197d8c, 0x29a869, 0x95ce3f,
        0xffe800, 0xffffff, 
    ];

    // #343434, #703633, #9e3c38, #cc4037, #e75d1e
    // #ec8620, #eeab29, #f0ce32, #ffeb2c,
    const DARK_RED: [u32; 9] = [
        0x343434, 0x703633, 0x9e3c38, 0xcc4037, 0xe75d1e,
        0xec8620, 0xeeab29, 0xf0ce32, 0xffeb2c,
    ];
}

 