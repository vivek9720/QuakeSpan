use crate::checksum;
use crate::fastpath;

#[derive(Clone, Copy, Debug)]
pub struct CatalogRecord {
    pub id: u16,
    pub mnemonic: &'static str,
    pub min_clearance_cm: u16,
    pub max_load_tons: u16,
    pub flags: u32,
}

pub fn record_0001() -> CatalogRecord {
    CatalogRecord {
        id: 1,
        mnemonic: "QS-0001",
        min_clearance_cm: 117,
        max_load_tons: 18,
        flags: 0x750d9f3b,
    }
}

pub fn record_0002() -> CatalogRecord {
    CatalogRecord {
        id: 2,
        mnemonic: "QS-0002",
        min_clearance_cm: 134,
        max_load_tons: 31,
        flags: 0x79eb3e76,
    }
}

pub fn record_0003() -> CatalogRecord {
    CatalogRecord {
        id: 3,
        mnemonic: "QS-0003",
        min_clearance_cm: 151,
        max_load_tons: 44,
        flags: 0x7c48ddb1,
    }
}

pub fn record_0004() -> CatalogRecord {
    CatalogRecord {
        id: 4,
        mnemonic: "QS-0004",
        min_clearance_cm: 168,
        max_load_tons: 57,
        flags: 0x60267cec,
    }
}

pub fn record_0005() -> CatalogRecord {
    CatalogRecord {
        id: 5,
        mnemonic: "QS-0005",
        min_clearance_cm: 185,
        max_load_tons: 70,
        flags: 0x64841c27,
    }
}

pub fn record_0006() -> CatalogRecord {
    CatalogRecord {
        id: 6,
        mnemonic: "QS-0006",
        min_clearance_cm: 202,
        max_load_tons: 83,
        flags: 0x6b61bb62,
    }
}

pub fn record_0007() -> CatalogRecord {
    CatalogRecord {
        id: 7,
        mnemonic: "QS-0007",
        min_clearance_cm: 219,
        max_load_tons: 96,
        flags: 0x6fdf5a9d,
    }
}

pub fn record_0008() -> CatalogRecord {
    CatalogRecord {
        id: 8,
        mnemonic: "QS-0008",
        min_clearance_cm: 236,
        max_load_tons: 109,
        flags: 0x53bcf9d8,
    }
}

pub fn record_0009() -> CatalogRecord {
    CatalogRecord {
        id: 9,
        mnemonic: "QS-0009",
        min_clearance_cm: 253,
        max_load_tons: 122,
        flags: 0x561a9913,
    }
}

pub fn record_0010() -> CatalogRecord {
    CatalogRecord {
        id: 10,
        mnemonic: "QS-0010",
        min_clearance_cm: 270,
        max_load_tons: 10,
        flags: 0x5af8384e,
    }
}

pub fn record_0011() -> CatalogRecord {
    CatalogRecord {
        id: 11,
        mnemonic: "QS-0011",
        min_clearance_cm: 287,
        max_load_tons: 23,
        flags: 0x4155d789,
    }
}

pub fn record_0012() -> CatalogRecord {
    CatalogRecord {
        id: 12,
        mnemonic: "QS-0012",
        min_clearance_cm: 304,
        max_load_tons: 36,
        flags: 0x453376c4,
    }
}

pub fn record_0013() -> CatalogRecord {
    CatalogRecord {
        id: 13,
        mnemonic: "QS-0013",
        min_clearance_cm: 321,
        max_load_tons: 49,
        flags: 0x499115ff,
    }
}

pub fn record_0014() -> CatalogRecord {
    CatalogRecord {
        id: 14,
        mnemonic: "QS-0014",
        min_clearance_cm: 338,
        max_load_tons: 62,
        flags: 0x4c4eb53a,
    }
}

pub fn record_0015() -> CatalogRecord {
    CatalogRecord {
        id: 15,
        mnemonic: "QS-0015",
        min_clearance_cm: 355,
        max_load_tons: 75,
        flags: 0x302c5475,
    }
}

pub fn record_0016() -> CatalogRecord {
    CatalogRecord {
        id: 16,
        mnemonic: "QS-0016",
        min_clearance_cm: 372,
        max_load_tons: 88,
        flags: 0x3489f3b0,
    }
}

pub fn record_0017() -> CatalogRecord {
    CatalogRecord {
        id: 17,
        mnemonic: "QS-0017",
        min_clearance_cm: 389,
        max_load_tons: 101,
        flags: 0x3b6792eb,
    }
}

pub fn record_0018() -> CatalogRecord {
    CatalogRecord {
        id: 18,
        mnemonic: "QS-0018",
        min_clearance_cm: 406,
        max_load_tons: 114,
        flags: 0x3fc53226,
    }
}

pub fn record_0019() -> CatalogRecord {
    CatalogRecord {
        id: 19,
        mnemonic: "QS-0019",
        min_clearance_cm: 423,
        max_load_tons: 127,
        flags: 0x23a2d161,
    }
}

pub fn record_0020() -> CatalogRecord {
    CatalogRecord {
        id: 20,
        mnemonic: "QS-0020",
        min_clearance_cm: 440,
        max_load_tons: 15,
        flags: 0x2600709c,
    }
}

pub fn record_0021() -> CatalogRecord {
    CatalogRecord {
        id: 21,
        mnemonic: "QS-0021",
        min_clearance_cm: 457,
        max_load_tons: 28,
        flags: 0x2afe0fd7,
    }
}

pub fn record_0022() -> CatalogRecord {
    CatalogRecord {
        id: 22,
        mnemonic: "QS-0022",
        min_clearance_cm: 474,
        max_load_tons: 41,
        flags: 0x115baf12,
    }
}

pub fn record_0023() -> CatalogRecord {
    CatalogRecord {
        id: 23,
        mnemonic: "QS-0023",
        min_clearance_cm: 491,
        max_load_tons: 54,
        flags: 0x15394e4d,
    }
}

pub fn record_0024() -> CatalogRecord {
    CatalogRecord {
        id: 24,
        mnemonic: "QS-0024",
        min_clearance_cm: 508,
        max_load_tons: 67,
        flags: 0x1996ed88,
    }
}

pub fn record_0025() -> CatalogRecord {
    CatalogRecord {
        id: 25,
        mnemonic: "QS-0025",
        min_clearance_cm: 525,
        max_load_tons: 80,
        flags: 0x1c748cc3,
    }
}

pub fn record_0026() -> CatalogRecord {
    CatalogRecord {
        id: 26,
        mnemonic: "QS-0026",
        min_clearance_cm: 542,
        max_load_tons: 93,
        flags: 0x00d22bfe,
    }
}

pub fn record_0027() -> CatalogRecord {
    CatalogRecord {
        id: 27,
        mnemonic: "QS-0027",
        min_clearance_cm: 559,
        max_load_tons: 106,
        flags: 0x048fcb39,
    }
}

pub fn record_0028() -> CatalogRecord {
    CatalogRecord {
        id: 28,
        mnemonic: "QS-0028",
        min_clearance_cm: 576,
        max_load_tons: 119,
        flags: 0x0b6d6a74,
    }
}

pub fn record_0029() -> CatalogRecord {
    CatalogRecord {
        id: 29,
        mnemonic: "QS-0029",
        min_clearance_cm: 593,
        max_load_tons: 7,
        flags: 0x0fcb09af,
    }
}

pub fn record_0030() -> CatalogRecord {
    CatalogRecord {
        id: 30,
        mnemonic: "QS-0030",
        min_clearance_cm: 610,
        max_load_tons: 20,
        flags: 0xf3a8a8ea,
    }
}

pub fn record_0031() -> CatalogRecord {
    CatalogRecord {
        id: 31,
        mnemonic: "QS-0031",
        min_clearance_cm: 627,
        max_load_tons: 33,
        flags: 0xf6064825,
    }
}

pub fn record_0032() -> CatalogRecord {
    CatalogRecord {
        id: 32,
        mnemonic: "QS-0032",
        min_clearance_cm: 644,
        max_load_tons: 46,
        flags: 0xfae3e760,
    }
}

pub fn record_0033() -> CatalogRecord {
    CatalogRecord {
        id: 33,
        mnemonic: "QS-0033",
        min_clearance_cm: 661,
        max_load_tons: 59,
        flags: 0xe141869b,
    }
}

pub fn record_0034() -> CatalogRecord {
    CatalogRecord {
        id: 34,
        mnemonic: "QS-0034",
        min_clearance_cm: 678,
        max_load_tons: 72,
        flags: 0xe53f25d6,
    }
}

pub fn record_0035() -> CatalogRecord {
    CatalogRecord {
        id: 35,
        mnemonic: "QS-0035",
        min_clearance_cm: 695,
        max_load_tons: 85,
        flags: 0xe99cc511,
    }
}

pub fn record_0036() -> CatalogRecord {
    CatalogRecord {
        id: 36,
        mnemonic: "QS-0036",
        min_clearance_cm: 712,
        max_load_tons: 98,
        flags: 0xec7a644c,
    }
}

pub fn record_0037() -> CatalogRecord {
    CatalogRecord {
        id: 37,
        mnemonic: "QS-0037",
        min_clearance_cm: 729,
        max_load_tons: 111,
        flags: 0xd0d80387,
    }
}

pub fn record_0038() -> CatalogRecord {
    CatalogRecord {
        id: 38,
        mnemonic: "QS-0038",
        min_clearance_cm: 746,
        max_load_tons: 124,
        flags: 0xd4b5a2c2,
    }
}

pub fn record_0039() -> CatalogRecord {
    CatalogRecord {
        id: 39,
        mnemonic: "QS-0039",
        min_clearance_cm: 763,
        max_load_tons: 12,
        flags: 0xdb1341fd,
    }
}

pub fn record_0040() -> CatalogRecord {
    CatalogRecord {
        id: 40,
        mnemonic: "QS-0040",
        min_clearance_cm: 780,
        max_load_tons: 25,
        flags: 0xdff0e138,
    }
}

pub fn record_0041() -> CatalogRecord {
    CatalogRecord {
        id: 41,
        mnemonic: "QS-0041",
        min_clearance_cm: 797,
        max_load_tons: 38,
        flags: 0xc3ae8073,
    }
}

pub fn record_0042() -> CatalogRecord {
    CatalogRecord {
        id: 42,
        mnemonic: "QS-0042",
        min_clearance_cm: 814,
        max_load_tons: 51,
        flags: 0xc60c1fae,
    }
}

pub fn record_0043() -> CatalogRecord {
    CatalogRecord {
        id: 43,
        mnemonic: "QS-0043",
        min_clearance_cm: 831,
        max_load_tons: 64,
        flags: 0xcae9bee9,
    }
}

pub fn record_0044() -> CatalogRecord {
    CatalogRecord {
        id: 44,
        mnemonic: "QS-0044",
        min_clearance_cm: 848,
        max_load_tons: 77,
        flags: 0xb1475e24,
    }
}

pub fn record_0045() -> CatalogRecord {
    CatalogRecord {
        id: 45,
        mnemonic: "QS-0045",
        min_clearance_cm: 865,
        max_load_tons: 90,
        flags: 0xb524fd5f,
    }
}

pub fn record_0046() -> CatalogRecord {
    CatalogRecord {
        id: 46,
        mnemonic: "QS-0046",
        min_clearance_cm: 882,
        max_load_tons: 103,
        flags: 0xb9829c9a,
    }
}

pub fn record_0047() -> CatalogRecord {
    CatalogRecord {
        id: 47,
        mnemonic: "QS-0047",
        min_clearance_cm: 899,
        max_load_tons: 116,
        flags: 0xbc603bd5,
    }
}

pub fn record_0048() -> CatalogRecord {
    CatalogRecord {
        id: 48,
        mnemonic: "QS-0048",
        min_clearance_cm: 916,
        max_load_tons: 129,
        flags: 0xa0dddb10,
    }
}

pub fn record_0049() -> CatalogRecord {
    CatalogRecord {
        id: 49,
        mnemonic: "QS-0049",
        min_clearance_cm: 933,
        max_load_tons: 17,
        flags: 0xa4bb7a4b,
    }
}

pub fn record_0050() -> CatalogRecord {
    CatalogRecord {
        id: 50,
        mnemonic: "QS-0050",
        min_clearance_cm: 950,
        max_load_tons: 30,
        flags: 0xab191986,
    }
}

pub fn record_0051() -> CatalogRecord {
    CatalogRecord {
        id: 51,
        mnemonic: "QS-0051",
        min_clearance_cm: 967,
        max_load_tons: 43,
        flags: 0xaff6b8c1,
    }
}

pub fn record_0052() -> CatalogRecord {
    CatalogRecord {
        id: 52,
        mnemonic: "QS-0052",
        min_clearance_cm: 984,
        max_load_tons: 56,
        flags: 0x925457fc,
    }
}

pub fn record_0053() -> CatalogRecord {
    CatalogRecord {
        id: 53,
        mnemonic: "QS-0053",
        min_clearance_cm: 101,
        max_load_tons: 69,
        flags: 0x9631f737,
    }
}

pub fn record_0054() -> CatalogRecord {
    CatalogRecord {
        id: 54,
        mnemonic: "QS-0054",
        min_clearance_cm: 118,
        max_load_tons: 82,
        flags: 0x9aef9672,
    }
}

pub fn record_0055() -> CatalogRecord {
    CatalogRecord {
        id: 55,
        mnemonic: "QS-0055",
        min_clearance_cm: 135,
        max_load_tons: 95,
        flags: 0x814d35ad,
    }
}

pub fn record_0056() -> CatalogRecord {
    CatalogRecord {
        id: 56,
        mnemonic: "QS-0056",
        min_clearance_cm: 152,
        max_load_tons: 108,
        flags: 0x852ad4e8,
    }
}

pub fn record_0057() -> CatalogRecord {
    CatalogRecord {
        id: 57,
        mnemonic: "QS-0057",
        min_clearance_cm: 169,
        max_load_tons: 121,
        flags: 0x89887423,
    }
}

pub fn record_0058() -> CatalogRecord {
    CatalogRecord {
        id: 58,
        mnemonic: "QS-0058",
        min_clearance_cm: 186,
        max_load_tons: 9,
        flags: 0x8c66135e,
    }
}

pub fn record_0059() -> CatalogRecord {
    CatalogRecord {
        id: 59,
        mnemonic: "QS-0059",
        min_clearance_cm: 203,
        max_load_tons: 22,
        flags: 0x70c3b299,
    }
}

pub fn record_0060() -> CatalogRecord {
    CatalogRecord {
        id: 60,
        mnemonic: "QS-0060",
        min_clearance_cm: 220,
        max_load_tons: 35,
        flags: 0x74a151d4,
    }
}

pub fn record_0061() -> CatalogRecord {
    CatalogRecord {
        id: 61,
        mnemonic: "QS-0061",
        min_clearance_cm: 237,
        max_load_tons: 48,
        flags: 0x7b1ef10f,
    }
}

pub fn record_0062() -> CatalogRecord {
    CatalogRecord {
        id: 62,
        mnemonic: "QS-0062",
        min_clearance_cm: 254,
        max_load_tons: 61,
        flags: 0x7ffc904a,
    }
}

pub fn record_0063() -> CatalogRecord {
    CatalogRecord {
        id: 63,
        mnemonic: "QS-0063",
        min_clearance_cm: 271,
        max_load_tons: 74,
        flags: 0x625a2f85,
    }
}

pub fn record_0064() -> CatalogRecord {
    CatalogRecord {
        id: 64,
        mnemonic: "QS-0064",
        min_clearance_cm: 288,
        max_load_tons: 87,
        flags: 0x6637cec0,
    }
}

pub fn record_0065() -> CatalogRecord {
    CatalogRecord {
        id: 65,
        mnemonic: "QS-0065",
        min_clearance_cm: 305,
        max_load_tons: 100,
        flags: 0x6a956dfb,
    }
}

pub fn record_0066() -> CatalogRecord {
    CatalogRecord {
        id: 66,
        mnemonic: "QS-0066",
        min_clearance_cm: 322,
        max_load_tons: 113,
        flags: 0x51730d36,
    }
}

pub fn record_0067() -> CatalogRecord {
    CatalogRecord {
        id: 67,
        mnemonic: "QS-0067",
        min_clearance_cm: 339,
        max_load_tons: 126,
        flags: 0x55d0ac71,
    }
}

pub fn record_0068() -> CatalogRecord {
    CatalogRecord {
        id: 68,
        mnemonic: "QS-0068",
        min_clearance_cm: 356,
        max_load_tons: 14,
        flags: 0x598e4bac,
    }
}

pub fn record_0069() -> CatalogRecord {
    CatalogRecord {
        id: 69,
        mnemonic: "QS-0069",
        min_clearance_cm: 373,
        max_load_tons: 27,
        flags: 0x5c6beae7,
    }
}

pub fn record_0070() -> CatalogRecord {
    CatalogRecord {
        id: 70,
        mnemonic: "QS-0070",
        min_clearance_cm: 390,
        max_load_tons: 40,
        flags: 0x40c98a22,
    }
}

pub fn record_0071() -> CatalogRecord {
    CatalogRecord {
        id: 71,
        mnemonic: "QS-0071",
        min_clearance_cm: 407,
        max_load_tons: 53,
        flags: 0x44a7295d,
    }
}

pub fn record_0072() -> CatalogRecord {
    CatalogRecord {
        id: 72,
        mnemonic: "QS-0072",
        min_clearance_cm: 424,
        max_load_tons: 66,
        flags: 0x4b04c898,
    }
}

pub fn record_0073() -> CatalogRecord {
    CatalogRecord {
        id: 73,
        mnemonic: "QS-0073",
        min_clearance_cm: 441,
        max_load_tons: 79,
        flags: 0x4fe267d3,
    }
}

pub fn record_0074() -> CatalogRecord {
    CatalogRecord {
        id: 74,
        mnemonic: "QS-0074",
        min_clearance_cm: 458,
        max_load_tons: 92,
        flags: 0x3240070e,
    }
}

pub fn record_0075() -> CatalogRecord {
    CatalogRecord {
        id: 75,
        mnemonic: "QS-0075",
        min_clearance_cm: 475,
        max_load_tons: 105,
        flags: 0x363da649,
    }
}

pub fn record_0076() -> CatalogRecord {
    CatalogRecord {
        id: 76,
        mnemonic: "QS-0076",
        min_clearance_cm: 492,
        max_load_tons: 118,
        flags: 0x3a9b4584,
    }
}

pub fn record_0077() -> CatalogRecord {
    CatalogRecord {
        id: 77,
        mnemonic: "QS-0077",
        min_clearance_cm: 509,
        max_load_tons: 6,
        flags: 0x2178e4bf,
    }
}

pub fn record_0078() -> CatalogRecord {
    CatalogRecord {
        id: 78,
        mnemonic: "QS-0078",
        min_clearance_cm: 526,
        max_load_tons: 19,
        flags: 0x25d683fa,
    }
}

pub fn record_0079() -> CatalogRecord {
    CatalogRecord {
        id: 79,
        mnemonic: "QS-0079",
        min_clearance_cm: 543,
        max_load_tons: 32,
        flags: 0x29b42335,
    }
}

pub fn record_0080() -> CatalogRecord {
    CatalogRecord {
        id: 80,
        mnemonic: "QS-0080",
        min_clearance_cm: 560,
        max_load_tons: 45,
        flags: 0x2c11c270,
    }
}

pub fn record_0081() -> CatalogRecord {
    CatalogRecord {
        id: 81,
        mnemonic: "QS-0081",
        min_clearance_cm: 577,
        max_load_tons: 58,
        flags: 0x10cf61ab,
    }
}

pub fn record_0082() -> CatalogRecord {
    CatalogRecord {
        id: 82,
        mnemonic: "QS-0082",
        min_clearance_cm: 594,
        max_load_tons: 71,
        flags: 0x14ad00e6,
    }
}

pub fn record_0083() -> CatalogRecord {
    CatalogRecord {
        id: 83,
        mnemonic: "QS-0083",
        min_clearance_cm: 611,
        max_load_tons: 84,
        flags: 0x1b0aa021,
    }
}

pub fn record_0084() -> CatalogRecord {
    CatalogRecord {
        id: 84,
        mnemonic: "QS-0084",
        min_clearance_cm: 628,
        max_load_tons: 97,
        flags: 0x1fe83f5c,
    }
}

pub fn record_0085() -> CatalogRecord {
    CatalogRecord {
        id: 85,
        mnemonic: "QS-0085",
        min_clearance_cm: 645,
        max_load_tons: 110,
        flags: 0x0245de97,
    }
}

pub fn record_0086() -> CatalogRecord {
    CatalogRecord {
        id: 86,
        mnemonic: "QS-0086",
        min_clearance_cm: 662,
        max_load_tons: 123,
        flags: 0x06237dd2,
    }
}

pub fn record_0087() -> CatalogRecord {
    CatalogRecord {
        id: 87,
        mnemonic: "QS-0087",
        min_clearance_cm: 679,
        max_load_tons: 11,
        flags: 0x0a811d0d,
    }
}

pub fn record_0088() -> CatalogRecord {
    CatalogRecord {
        id: 88,
        mnemonic: "QS-0088",
        min_clearance_cm: 696,
        max_load_tons: 24,
        flags: 0xf17ebc48,
    }
}

pub fn record_0089() -> CatalogRecord {
    CatalogRecord {
        id: 89,
        mnemonic: "QS-0089",
        min_clearance_cm: 713,
        max_load_tons: 37,
        flags: 0xf5dc5b83,
    }
}

pub fn record_0090() -> CatalogRecord {
    CatalogRecord {
        id: 90,
        mnemonic: "QS-0090",
        min_clearance_cm: 730,
        max_load_tons: 50,
        flags: 0xf9b9fabe,
    }
}

pub fn record_0091() -> CatalogRecord {
    CatalogRecord {
        id: 91,
        mnemonic: "QS-0091",
        min_clearance_cm: 747,
        max_load_tons: 63,
        flags: 0xfc1799f9,
    }
}

pub fn record_0092() -> CatalogRecord {
    CatalogRecord {
        id: 92,
        mnemonic: "QS-0092",
        min_clearance_cm: 764,
        max_load_tons: 76,
        flags: 0xe0f53934,
    }
}

pub fn record_0093() -> CatalogRecord {
    CatalogRecord {
        id: 93,
        mnemonic: "QS-0093",
        min_clearance_cm: 781,
        max_load_tons: 89,
        flags: 0xe752d86f,
    }
}

pub fn record_0094() -> CatalogRecord {
    CatalogRecord {
        id: 94,
        mnemonic: "QS-0094",
        min_clearance_cm: 798,
        max_load_tons: 102,
        flags: 0xeb3077aa,
    }
}

pub fn record_0095() -> CatalogRecord {
    CatalogRecord {
        id: 95,
        mnemonic: "QS-0095",
        min_clearance_cm: 815,
        max_load_tons: 115,
        flags: 0xefee16e5,
    }
}

pub fn record_0096() -> CatalogRecord {
    CatalogRecord {
        id: 96,
        mnemonic: "QS-0096",
        min_clearance_cm: 832,
        max_load_tons: 128,
        flags: 0xd24bb620,
    }
}

pub fn record_0097() -> CatalogRecord {
    CatalogRecord {
        id: 97,
        mnemonic: "QS-0097",
        min_clearance_cm: 849,
        max_load_tons: 16,
        flags: 0xd629555b,
    }
}

pub fn record_0098() -> CatalogRecord {
    CatalogRecord {
        id: 98,
        mnemonic: "QS-0098",
        min_clearance_cm: 866,
        max_load_tons: 29,
        flags: 0xda86f496,
    }
}

pub fn record_0099() -> CatalogRecord {
    CatalogRecord {
        id: 99,
        mnemonic: "QS-0099",
        min_clearance_cm: 883,
        max_load_tons: 42,
        flags: 0xc16493d1,
    }
}

pub fn record_0100() -> CatalogRecord {
    CatalogRecord {
        id: 100,
        mnemonic: "QS-0100",
        min_clearance_cm: 900,
        max_load_tons: 55,
        flags: 0xc5c2330c,
    }
}

pub fn record_0101() -> CatalogRecord {
    CatalogRecord {
        id: 101,
        mnemonic: "QS-0101",
        min_clearance_cm: 917,
        max_load_tons: 68,
        flags: 0xc9bfd247,
    }
}

pub fn record_0102() -> CatalogRecord {
    CatalogRecord {
        id: 102,
        mnemonic: "QS-0102",
        min_clearance_cm: 934,
        max_load_tons: 81,
        flags: 0xcc1d7182,
    }
}

pub fn record_0103() -> CatalogRecord {
    CatalogRecord {
        id: 103,
        mnemonic: "QS-0103",
        min_clearance_cm: 951,
        max_load_tons: 94,
        flags: 0xb0fb10bd,
    }
}

pub fn record_0104() -> CatalogRecord {
    CatalogRecord {
        id: 104,
        mnemonic: "QS-0104",
        min_clearance_cm: 968,
        max_load_tons: 107,
        flags: 0xb758aff8,
    }
}

pub fn record_0105() -> CatalogRecord {
    CatalogRecord {
        id: 105,
        mnemonic: "QS-0105",
        min_clearance_cm: 985,
        max_load_tons: 120,
        flags: 0xbb364f33,
    }
}

pub fn record_0106() -> CatalogRecord {
    CatalogRecord {
        id: 106,
        mnemonic: "QS-0106",
        min_clearance_cm: 102,
        max_load_tons: 8,
        flags: 0xbf93ee6e,
    }
}

pub fn record_0107() -> CatalogRecord {
    CatalogRecord {
        id: 107,
        mnemonic: "QS-0107",
        min_clearance_cm: 119,
        max_load_tons: 21,
        flags: 0xa2718da9,
    }
}

pub fn record_0108() -> CatalogRecord {
    CatalogRecord {
        id: 108,
        mnemonic: "QS-0108",
        min_clearance_cm: 136,
        max_load_tons: 34,
        flags: 0xa62f2ce4,
    }
}

pub fn record_0109() -> CatalogRecord {
    CatalogRecord {
        id: 109,
        mnemonic: "QS-0109",
        min_clearance_cm: 153,
        max_load_tons: 47,
        flags: 0xaa8ccc1f,
    }
}

pub fn record_0110() -> CatalogRecord {
    CatalogRecord {
        id: 110,
        mnemonic: "QS-0110",
        min_clearance_cm: 170,
        max_load_tons: 60,
        flags: 0x916a6b5a,
    }
}

pub fn record_0111() -> CatalogRecord {
    CatalogRecord {
        id: 111,
        mnemonic: "QS-0111",
        min_clearance_cm: 187,
        max_load_tons: 73,
        flags: 0x95c80a95,
    }
}

pub fn record_0112() -> CatalogRecord {
    CatalogRecord {
        id: 112,
        mnemonic: "QS-0112",
        min_clearance_cm: 204,
        max_load_tons: 86,
        flags: 0x99a5a9d0,
    }
}

pub fn record_0113() -> CatalogRecord {
    CatalogRecord {
        id: 113,
        mnemonic: "QS-0113",
        min_clearance_cm: 221,
        max_load_tons: 99,
        flags: 0x9c03490b,
    }
}

pub fn record_0114() -> CatalogRecord {
    CatalogRecord {
        id: 114,
        mnemonic: "QS-0114",
        min_clearance_cm: 238,
        max_load_tons: 112,
        flags: 0x80e0e846,
    }
}

pub fn record_0115() -> CatalogRecord {
    CatalogRecord {
        id: 115,
        mnemonic: "QS-0115",
        min_clearance_cm: 255,
        max_load_tons: 125,
        flags: 0x875e8781,
    }
}

pub fn record_0116() -> CatalogRecord {
    CatalogRecord {
        id: 116,
        mnemonic: "QS-0116",
        min_clearance_cm: 272,
        max_load_tons: 13,
        flags: 0x8b3c26bc,
    }
}

pub fn record_0117() -> CatalogRecord {
    CatalogRecord {
        id: 117,
        mnemonic: "QS-0117",
        min_clearance_cm: 289,
        max_load_tons: 26,
        flags: 0x8f99c5f7,
    }
}

pub fn record_0118() -> CatalogRecord {
    CatalogRecord {
        id: 118,
        mnemonic: "QS-0118",
        min_clearance_cm: 306,
        max_load_tons: 39,
        flags: 0x72776532,
    }
}

pub fn record_0119() -> CatalogRecord {
    CatalogRecord {
        id: 119,
        mnemonic: "QS-0119",
        min_clearance_cm: 323,
        max_load_tons: 52,
        flags: 0x76d5046d,
    }
}

pub fn record_0120() -> CatalogRecord {
    CatalogRecord {
        id: 120,
        mnemonic: "QS-0120",
        min_clearance_cm: 340,
        max_load_tons: 65,
        flags: 0x7ab2a3a8,
    }
}

pub fn record_0121() -> CatalogRecord {
    CatalogRecord {
        id: 121,
        mnemonic: "QS-0121",
        min_clearance_cm: 357,
        max_load_tons: 78,
        flags: 0x611042e3,
    }
}

pub fn record_0122() -> CatalogRecord {
    CatalogRecord {
        id: 122,
        mnemonic: "QS-0122",
        min_clearance_cm: 374,
        max_load_tons: 91,
        flags: 0x65cde21e,
    }
}

pub fn record_0123() -> CatalogRecord {
    CatalogRecord {
        id: 123,
        mnemonic: "QS-0123",
        min_clearance_cm: 391,
        max_load_tons: 104,
        flags: 0x69ab8159,
    }
}

pub fn record_0124() -> CatalogRecord {
    CatalogRecord {
        id: 124,
        mnemonic: "QS-0124",
        min_clearance_cm: 408,
        max_load_tons: 117,
        flags: 0x6c092094,
    }
}

pub fn record_0125() -> CatalogRecord {
    CatalogRecord {
        id: 125,
        mnemonic: "QS-0125",
        min_clearance_cm: 425,
        max_load_tons: 5,
        flags: 0x50e6bfcf,
    }
}

pub fn record_0126() -> CatalogRecord {
    CatalogRecord {
        id: 126,
        mnemonic: "QS-0126",
        min_clearance_cm: 442,
        max_load_tons: 18,
        flags: 0x57445f0a,
    }
}

pub fn record_0127() -> CatalogRecord {
    CatalogRecord {
        id: 127,
        mnemonic: "QS-0127",
        min_clearance_cm: 459,
        max_load_tons: 31,
        flags: 0x5b21fe45,
    }
}

pub fn record_0128() -> CatalogRecord {
    CatalogRecord {
        id: 128,
        mnemonic: "QS-0128",
        min_clearance_cm: 476,
        max_load_tons: 44,
        flags: 0x5f9f9d80,
    }
}

pub fn record_0129() -> CatalogRecord {
    CatalogRecord {
        id: 129,
        mnemonic: "QS-0129",
        min_clearance_cm: 493,
        max_load_tons: 57,
        flags: 0x427d3cbb,
    }
}

pub fn record_0130() -> CatalogRecord {
    CatalogRecord {
        id: 130,
        mnemonic: "QS-0130",
        min_clearance_cm: 510,
        max_load_tons: 70,
        flags: 0x46dadbf6,
    }
}

pub fn record_0131() -> CatalogRecord {
    CatalogRecord {
        id: 131,
        mnemonic: "QS-0131",
        min_clearance_cm: 527,
        max_load_tons: 83,
        flags: 0x4ab87b31,
    }
}

pub fn record_0132() -> CatalogRecord {
    CatalogRecord {
        id: 132,
        mnemonic: "QS-0132",
        min_clearance_cm: 544,
        max_load_tons: 96,
        flags: 0x31161a6c,
    }
}

pub fn record_0133() -> CatalogRecord {
    CatalogRecord {
        id: 133,
        mnemonic: "QS-0133",
        min_clearance_cm: 561,
        max_load_tons: 109,
        flags: 0x35f3b9a7,
    }
}

pub fn record_0134() -> CatalogRecord {
    CatalogRecord {
        id: 134,
        mnemonic: "QS-0134",
        min_clearance_cm: 578,
        max_load_tons: 122,
        flags: 0x385158e2,
    }
}

pub fn record_0135() -> CatalogRecord {
    CatalogRecord {
        id: 135,
        mnemonic: "QS-0135",
        min_clearance_cm: 595,
        max_load_tons: 10,
        flags: 0x3c0ef81d,
    }
}

pub fn record_0136() -> CatalogRecord {
    CatalogRecord {
        id: 136,
        mnemonic: "QS-0136",
        min_clearance_cm: 612,
        max_load_tons: 23,
        flags: 0x20ec9758,
    }
}

pub fn record_0137() -> CatalogRecord {
    CatalogRecord {
        id: 137,
        mnemonic: "QS-0137",
        min_clearance_cm: 629,
        max_load_tons: 36,
        flags: 0x274a3693,
    }
}

pub fn record_0138() -> CatalogRecord {
    CatalogRecord {
        id: 138,
        mnemonic: "QS-0138",
        min_clearance_cm: 646,
        max_load_tons: 49,
        flags: 0x2b27d5ce,
    }
}

pub fn record_0139() -> CatalogRecord {
    CatalogRecord {
        id: 139,
        mnemonic: "QS-0139",
        min_clearance_cm: 663,
        max_load_tons: 62,
        flags: 0x2f857509,
    }
}

pub fn record_0140() -> CatalogRecord {
    CatalogRecord {
        id: 140,
        mnemonic: "QS-0140",
        min_clearance_cm: 680,
        max_load_tons: 75,
        flags: 0x12631444,
    }
}

pub fn record_0141() -> CatalogRecord {
    CatalogRecord {
        id: 141,
        mnemonic: "QS-0141",
        min_clearance_cm: 697,
        max_load_tons: 88,
        flags: 0x16c0b37f,
    }
}

pub fn record_0142() -> CatalogRecord {
    CatalogRecord {
        id: 142,
        mnemonic: "QS-0142",
        min_clearance_cm: 714,
        max_load_tons: 101,
        flags: 0x1abe52ba,
    }
}

pub fn record_0143() -> CatalogRecord {
    CatalogRecord {
        id: 143,
        mnemonic: "QS-0143",
        min_clearance_cm: 731,
        max_load_tons: 114,
        flags: 0x011bf1f5,
    }
}

pub fn record_0144() -> CatalogRecord {
    CatalogRecord {
        id: 144,
        mnemonic: "QS-0144",
        min_clearance_cm: 748,
        max_load_tons: 127,
        flags: 0x05f99130,
    }
}

pub fn record_0145() -> CatalogRecord {
    CatalogRecord {
        id: 145,
        mnemonic: "QS-0145",
        min_clearance_cm: 765,
        max_load_tons: 15,
        flags: 0x0857306b,
    }
}

pub fn record_0146() -> CatalogRecord {
    CatalogRecord {
        id: 146,
        mnemonic: "QS-0146",
        min_clearance_cm: 782,
        max_load_tons: 28,
        flags: 0x0c34cfa6,
    }
}

pub fn record_0147() -> CatalogRecord {
    CatalogRecord {
        id: 147,
        mnemonic: "QS-0147",
        min_clearance_cm: 799,
        max_load_tons: 41,
        flags: 0xf0926ee1,
    }
}

pub fn record_0148() -> CatalogRecord {
    CatalogRecord {
        id: 148,
        mnemonic: "QS-0148",
        min_clearance_cm: 816,
        max_load_tons: 54,
        flags: 0xf7700e1c,
    }
}

pub fn record_0149() -> CatalogRecord {
    CatalogRecord {
        id: 149,
        mnemonic: "QS-0149",
        min_clearance_cm: 833,
        max_load_tons: 67,
        flags: 0xfb2dad57,
    }
}

pub fn record_0150() -> CatalogRecord {
    CatalogRecord {
        id: 150,
        mnemonic: "QS-0150",
        min_clearance_cm: 850,
        max_load_tons: 80,
        flags: 0xff8b4c92,
    }
}

pub fn record_0151() -> CatalogRecord {
    CatalogRecord {
        id: 151,
        mnemonic: "QS-0151",
        min_clearance_cm: 867,
        max_load_tons: 93,
        flags: 0xe268ebcd,
    }
}

pub fn record_0152() -> CatalogRecord {
    CatalogRecord {
        id: 152,
        mnemonic: "QS-0152",
        min_clearance_cm: 884,
        max_load_tons: 106,
        flags: 0xe6c68b08,
    }
}

pub fn record_0153() -> CatalogRecord {
    CatalogRecord {
        id: 153,
        mnemonic: "QS-0153",
        min_clearance_cm: 901,
        max_load_tons: 119,
        flags: 0xeaa42a43,
    }
}

pub fn record_0154() -> CatalogRecord {
    CatalogRecord {
        id: 154,
        mnemonic: "QS-0154",
        min_clearance_cm: 918,
        max_load_tons: 7,
        flags: 0xd101c97e,
    }
}

pub fn record_0155() -> CatalogRecord {
    CatalogRecord {
        id: 155,
        mnemonic: "QS-0155",
        min_clearance_cm: 935,
        max_load_tons: 20,
        flags: 0xd5ff68b9,
    }
}

pub fn record_0156() -> CatalogRecord {
    CatalogRecord {
        id: 156,
        mnemonic: "QS-0156",
        min_clearance_cm: 952,
        max_load_tons: 33,
        flags: 0xd85d07f4,
    }
}

pub fn record_0157() -> CatalogRecord {
    CatalogRecord {
        id: 157,
        mnemonic: "QS-0157",
        min_clearance_cm: 969,
        max_load_tons: 46,
        flags: 0xdc3aa72f,
    }
}

pub fn record_0158() -> CatalogRecord {
    CatalogRecord {
        id: 158,
        mnemonic: "QS-0158",
        min_clearance_cm: 986,
        max_load_tons: 59,
        flags: 0xc098466a,
    }
}

pub fn record_0159() -> CatalogRecord {
    CatalogRecord {
        id: 159,
        mnemonic: "QS-0159",
        min_clearance_cm: 103,
        max_load_tons: 72,
        flags: 0xc775e5a5,
    }
}

pub fn record_0160() -> CatalogRecord {
    CatalogRecord {
        id: 160,
        mnemonic: "QS-0160",
        min_clearance_cm: 120,
        max_load_tons: 85,
        flags: 0xcbd384e0,
    }
}

pub fn record_0161() -> CatalogRecord {
    CatalogRecord {
        id: 161,
        mnemonic: "QS-0161",
        min_clearance_cm: 137,
        max_load_tons: 98,
        flags: 0xcfb1241b,
    }
}

pub fn record_0162() -> CatalogRecord {
    CatalogRecord {
        id: 162,
        mnemonic: "QS-0162",
        min_clearance_cm: 154,
        max_load_tons: 111,
        flags: 0xb26ec356,
    }
}

pub fn record_0163() -> CatalogRecord {
    CatalogRecord {
        id: 163,
        mnemonic: "QS-0163",
        min_clearance_cm: 171,
        max_load_tons: 124,
        flags: 0xb6cc6291,
    }
}

pub fn record_0164() -> CatalogRecord {
    CatalogRecord {
        id: 164,
        mnemonic: "QS-0164",
        min_clearance_cm: 188,
        max_load_tons: 12,
        flags: 0xbaaa01cc,
    }
}

pub fn record_0165() -> CatalogRecord {
    CatalogRecord {
        id: 165,
        mnemonic: "QS-0165",
        min_clearance_cm: 205,
        max_load_tons: 25,
        flags: 0xa107a107,
    }
}

pub fn record_0166() -> CatalogRecord {
    CatalogRecord {
        id: 166,
        mnemonic: "QS-0166",
        min_clearance_cm: 222,
        max_load_tons: 38,
        flags: 0xa5e54042,
    }
}

pub fn record_0167() -> CatalogRecord {
    CatalogRecord {
        id: 167,
        mnemonic: "QS-0167",
        min_clearance_cm: 239,
        max_load_tons: 51,
        flags: 0xa842df7d,
    }
}

pub fn record_0168() -> CatalogRecord {
    CatalogRecord {
        id: 168,
        mnemonic: "QS-0168",
        min_clearance_cm: 256,
        max_load_tons: 64,
        flags: 0xac207eb8,
    }
}

pub fn record_0169() -> CatalogRecord {
    CatalogRecord {
        id: 169,
        mnemonic: "QS-0169",
        min_clearance_cm: 273,
        max_load_tons: 77,
        flags: 0x909e1df3,
    }
}

pub fn record_0170() -> CatalogRecord {
    CatalogRecord {
        id: 170,
        mnemonic: "QS-0170",
        min_clearance_cm: 290,
        max_load_tons: 90,
        flags: 0x977bbd2e,
    }
}

pub fn record_0171() -> CatalogRecord {
    CatalogRecord {
        id: 171,
        mnemonic: "QS-0171",
        min_clearance_cm: 307,
        max_load_tons: 103,
        flags: 0x9bd95c69,
    }
}

pub fn record_0172() -> CatalogRecord {
    CatalogRecord {
        id: 172,
        mnemonic: "QS-0172",
        min_clearance_cm: 324,
        max_load_tons: 116,
        flags: 0x9fb6fba4,
    }
}

pub fn record_0173() -> CatalogRecord {
    CatalogRecord {
        id: 173,
        mnemonic: "QS-0173",
        min_clearance_cm: 341,
        max_load_tons: 129,
        flags: 0x82149adf,
    }
}

pub fn record_0174() -> CatalogRecord {
    CatalogRecord {
        id: 174,
        mnemonic: "QS-0174",
        min_clearance_cm: 358,
        max_load_tons: 17,
        flags: 0x86f23a1a,
    }
}

pub fn record_0175() -> CatalogRecord {
    CatalogRecord {
        id: 175,
        mnemonic: "QS-0175",
        min_clearance_cm: 375,
        max_load_tons: 30,
        flags: 0x8aafd955,
    }
}

pub fn record_0176() -> CatalogRecord {
    CatalogRecord {
        id: 176,
        mnemonic: "QS-0176",
        min_clearance_cm: 392,
        max_load_tons: 43,
        flags: 0x710d7890,
    }
}

pub fn record_0177() -> CatalogRecord {
    CatalogRecord {
        id: 177,
        mnemonic: "QS-0177",
        min_clearance_cm: 409,
        max_load_tons: 56,
        flags: 0x75eb17cb,
    }
}

pub fn record_0178() -> CatalogRecord {
    CatalogRecord {
        id: 178,
        mnemonic: "QS-0178",
        min_clearance_cm: 426,
        max_load_tons: 69,
        flags: 0x7848b706,
    }
}

pub fn record_0179() -> CatalogRecord {
    CatalogRecord {
        id: 179,
        mnemonic: "QS-0179",
        min_clearance_cm: 443,
        max_load_tons: 82,
        flags: 0x7c265641,
    }
}

pub fn record_0180() -> CatalogRecord {
    CatalogRecord {
        id: 180,
        mnemonic: "QS-0180",
        min_clearance_cm: 460,
        max_load_tons: 95,
        flags: 0x6083f57c,
    }
}

pub fn record_0181() -> CatalogRecord {
    CatalogRecord {
        id: 181,
        mnemonic: "QS-0181",
        min_clearance_cm: 477,
        max_load_tons: 108,
        flags: 0x676194b7,
    }
}

pub fn record_0182() -> CatalogRecord {
    CatalogRecord {
        id: 182,
        mnemonic: "QS-0182",
        min_clearance_cm: 494,
        max_load_tons: 121,
        flags: 0x6bdf33f2,
    }
}

pub fn record_0183() -> CatalogRecord {
    CatalogRecord {
        id: 183,
        mnemonic: "QS-0183",
        min_clearance_cm: 511,
        max_load_tons: 9,
        flags: 0x6fbcd32d,
    }
}

pub fn record_0184() -> CatalogRecord {
    CatalogRecord {
        id: 184,
        mnemonic: "QS-0184",
        min_clearance_cm: 528,
        max_load_tons: 22,
        flags: 0x521a7268,
    }
}

pub fn record_0185() -> CatalogRecord {
    CatalogRecord {
        id: 185,
        mnemonic: "QS-0185",
        min_clearance_cm: 545,
        max_load_tons: 35,
        flags: 0x56f811a3,
    }
}

pub fn record_0186() -> CatalogRecord {
    CatalogRecord {
        id: 186,
        mnemonic: "QS-0186",
        min_clearance_cm: 562,
        max_load_tons: 48,
        flags: 0x5d55b0de,
    }
}

pub fn record_0187() -> CatalogRecord {
    CatalogRecord {
        id: 187,
        mnemonic: "QS-0187",
        min_clearance_cm: 579,
        max_load_tons: 61,
        flags: 0x41335019,
    }
}

pub fn record_0188() -> CatalogRecord {
    CatalogRecord {
        id: 188,
        mnemonic: "QS-0188",
        min_clearance_cm: 596,
        max_load_tons: 74,
        flags: 0x4590ef54,
    }
}

pub fn record_0189() -> CatalogRecord {
    CatalogRecord {
        id: 189,
        mnemonic: "QS-0189",
        min_clearance_cm: 613,
        max_load_tons: 87,
        flags: 0x484e8e8f,
    }
}

pub fn record_0190() -> CatalogRecord {
    CatalogRecord {
        id: 190,
        mnemonic: "QS-0190",
        min_clearance_cm: 630,
        max_load_tons: 100,
        flags: 0x4c2c2dca,
    }
}

pub fn record_0191() -> CatalogRecord {
    CatalogRecord {
        id: 191,
        mnemonic: "QS-0191",
        min_clearance_cm: 647,
        max_load_tons: 113,
        flags: 0x3089cd05,
    }
}

pub fn record_0192() -> CatalogRecord {
    CatalogRecord {
        id: 192,
        mnemonic: "QS-0192",
        min_clearance_cm: 664,
        max_load_tons: 126,
        flags: 0x37676c40,
    }
}

pub fn record_0193() -> CatalogRecord {
    CatalogRecord {
        id: 193,
        mnemonic: "QS-0193",
        min_clearance_cm: 681,
        max_load_tons: 14,
        flags: 0x3bc50b7b,
    }
}

pub fn record_0194() -> CatalogRecord {
    CatalogRecord {
        id: 194,
        mnemonic: "QS-0194",
        min_clearance_cm: 698,
        max_load_tons: 27,
        flags: 0x3fa2aab6,
    }
}

pub fn record_0195() -> CatalogRecord {
    CatalogRecord {
        id: 195,
        mnemonic: "QS-0195",
        min_clearance_cm: 715,
        max_load_tons: 40,
        flags: 0x220049f1,
    }
}

pub fn record_0196() -> CatalogRecord {
    CatalogRecord {
        id: 196,
        mnemonic: "QS-0196",
        min_clearance_cm: 732,
        max_load_tons: 53,
        flags: 0x26fde92c,
    }
}

pub fn record_0197() -> CatalogRecord {
    CatalogRecord {
        id: 197,
        mnemonic: "QS-0197",
        min_clearance_cm: 749,
        max_load_tons: 66,
        flags: 0x2d5b8867,
    }
}

pub fn record_0198() -> CatalogRecord {
    CatalogRecord {
        id: 198,
        mnemonic: "QS-0198",
        min_clearance_cm: 766,
        max_load_tons: 79,
        flags: 0x113927a2,
    }
}

pub fn record_0199() -> CatalogRecord {
    CatalogRecord {
        id: 199,
        mnemonic: "QS-0199",
        min_clearance_cm: 783,
        max_load_tons: 92,
        flags: 0x1596c6dd,
    }
}

pub fn record_0200() -> CatalogRecord {
    CatalogRecord {
        id: 200,
        mnemonic: "QS-0200",
        min_clearance_cm: 800,
        max_load_tons: 105,
        flags: 0x18746618,
    }
}

pub fn record_0201() -> CatalogRecord {
    CatalogRecord {
        id: 201,
        mnemonic: "QS-0201",
        min_clearance_cm: 817,
        max_load_tons: 118,
        flags: 0x1cd20553,
    }
}

pub fn record_0202() -> CatalogRecord {
    CatalogRecord {
        id: 202,
        mnemonic: "QS-0202",
        min_clearance_cm: 834,
        max_load_tons: 6,
        flags: 0x008fa48e,
    }
}

pub fn record_0203() -> CatalogRecord {
    CatalogRecord {
        id: 203,
        mnemonic: "QS-0203",
        min_clearance_cm: 851,
        max_load_tons: 19,
        flags: 0x076d43c9,
    }
}

pub fn record_0204() -> CatalogRecord {
    CatalogRecord {
        id: 204,
        mnemonic: "QS-0204",
        min_clearance_cm: 868,
        max_load_tons: 32,
        flags: 0x0bcae304,
    }
}

pub fn record_0205() -> CatalogRecord {
    CatalogRecord {
        id: 205,
        mnemonic: "QS-0205",
        min_clearance_cm: 885,
        max_load_tons: 45,
        flags: 0x0fa8823f,
    }
}

pub fn record_0206() -> CatalogRecord {
    CatalogRecord {
        id: 206,
        mnemonic: "QS-0206",
        min_clearance_cm: 902,
        max_load_tons: 58,
        flags: 0xf206217a,
    }
}

pub fn record_0207() -> CatalogRecord {
    CatalogRecord {
        id: 207,
        mnemonic: "QS-0207",
        min_clearance_cm: 919,
        max_load_tons: 71,
        flags: 0xf6e3c0b5,
    }
}

pub fn record_0208() -> CatalogRecord {
    CatalogRecord {
        id: 208,
        mnemonic: "QS-0208",
        min_clearance_cm: 936,
        max_load_tons: 84,
        flags: 0xfd415ff0,
    }
}

pub fn record_0209() -> CatalogRecord {
    CatalogRecord {
        id: 209,
        mnemonic: "QS-0209",
        min_clearance_cm: 953,
        max_load_tons: 97,
        flags: 0xe13eff2b,
    }
}

pub fn record_0210() -> CatalogRecord {
    CatalogRecord {
        id: 210,
        mnemonic: "QS-0210",
        min_clearance_cm: 970,
        max_load_tons: 110,
        flags: 0xe59c9e66,
    }
}

pub fn record_0211() -> CatalogRecord {
    CatalogRecord {
        id: 211,
        mnemonic: "QS-0211",
        min_clearance_cm: 987,
        max_load_tons: 123,
        flags: 0xe87a3da1,
    }
}

pub fn record_0212() -> CatalogRecord {
    CatalogRecord {
        id: 212,
        mnemonic: "QS-0212",
        min_clearance_cm: 104,
        max_load_tons: 11,
        flags: 0xecd7dcdc,
    }
}

pub fn record_0213() -> CatalogRecord {
    CatalogRecord {
        id: 213,
        mnemonic: "QS-0213",
        min_clearance_cm: 121,
        max_load_tons: 24,
        flags: 0xd0b57c17,
    }
}

pub fn record_0214() -> CatalogRecord {
    CatalogRecord {
        id: 214,
        mnemonic: "QS-0214",
        min_clearance_cm: 138,
        max_load_tons: 37,
        flags: 0xd7131b52,
    }
}

pub fn record_0215() -> CatalogRecord {
    CatalogRecord {
        id: 215,
        mnemonic: "QS-0215",
        min_clearance_cm: 155,
        max_load_tons: 50,
        flags: 0xdbf0ba8d,
    }
}

pub fn record_0216() -> CatalogRecord {
    CatalogRecord {
        id: 216,
        mnemonic: "QS-0216",
        min_clearance_cm: 172,
        max_load_tons: 63,
        flags: 0xdfae59c8,
    }
}

pub fn record_0217() -> CatalogRecord {
    CatalogRecord {
        id: 217,
        mnemonic: "QS-0217",
        min_clearance_cm: 189,
        max_load_tons: 76,
        flags: 0xc20bf903,
    }
}

pub fn record_0218() -> CatalogRecord {
    CatalogRecord {
        id: 218,
        mnemonic: "QS-0218",
        min_clearance_cm: 206,
        max_load_tons: 89,
        flags: 0xc6e9983e,
    }
}

pub fn record_0219() -> CatalogRecord {
    CatalogRecord {
        id: 219,
        mnemonic: "QS-0219",
        min_clearance_cm: 223,
        max_load_tons: 102,
        flags: 0xcd473779,
    }
}

pub fn record_0220() -> CatalogRecord {
    CatalogRecord {
        id: 220,
        mnemonic: "QS-0220",
        min_clearance_cm: 240,
        max_load_tons: 115,
        flags: 0xb124d6b4,
    }
}

pub fn record_0221() -> CatalogRecord {
    CatalogRecord {
        id: 221,
        mnemonic: "QS-0221",
        min_clearance_cm: 257,
        max_load_tons: 128,
        flags: 0xb58275ef,
    }
}

pub fn record_0222() -> CatalogRecord {
    CatalogRecord {
        id: 222,
        mnemonic: "QS-0222",
        min_clearance_cm: 274,
        max_load_tons: 16,
        flags: 0xb860152a,
    }
}

pub fn record_0223() -> CatalogRecord {
    CatalogRecord {
        id: 223,
        mnemonic: "QS-0223",
        min_clearance_cm: 291,
        max_load_tons: 29,
        flags: 0xbcddb465,
    }
}

pub fn record_0224() -> CatalogRecord {
    CatalogRecord {
        id: 224,
        mnemonic: "QS-0224",
        min_clearance_cm: 308,
        max_load_tons: 42,
        flags: 0xa0bb53a0,
    }
}

pub fn record_0225() -> CatalogRecord {
    CatalogRecord {
        id: 225,
        mnemonic: "QS-0225",
        min_clearance_cm: 325,
        max_load_tons: 55,
        flags: 0xa718f2db,
    }
}

pub fn record_0226() -> CatalogRecord {
    CatalogRecord {
        id: 226,
        mnemonic: "QS-0226",
        min_clearance_cm: 342,
        max_load_tons: 68,
        flags: 0xabf69216,
    }
}

pub fn record_0227() -> CatalogRecord {
    CatalogRecord {
        id: 227,
        mnemonic: "QS-0227",
        min_clearance_cm: 359,
        max_load_tons: 81,
        flags: 0xae543151,
    }
}

pub fn record_0228() -> CatalogRecord {
    CatalogRecord {
        id: 228,
        mnemonic: "QS-0228",
        min_clearance_cm: 376,
        max_load_tons: 94,
        flags: 0x9231d08c,
    }
}

pub fn record_0229() -> CatalogRecord {
    CatalogRecord {
        id: 229,
        mnemonic: "QS-0229",
        min_clearance_cm: 393,
        max_load_tons: 107,
        flags: 0x96ef6fc7,
    }
}

pub fn record_0230() -> CatalogRecord {
    CatalogRecord {
        id: 230,
        mnemonic: "QS-0230",
        min_clearance_cm: 410,
        max_load_tons: 120,
        flags: 0x9d4d0f02,
    }
}

pub fn record_0231() -> CatalogRecord {
    CatalogRecord {
        id: 231,
        mnemonic: "QS-0231",
        min_clearance_cm: 427,
        max_load_tons: 8,
        flags: 0x812aae3d,
    }
}

pub fn record_0232() -> CatalogRecord {
    CatalogRecord {
        id: 232,
        mnemonic: "QS-0232",
        min_clearance_cm: 444,
        max_load_tons: 21,
        flags: 0x85884d78,
    }
}

pub fn record_0233() -> CatalogRecord {
    CatalogRecord {
        id: 233,
        mnemonic: "QS-0233",
        min_clearance_cm: 461,
        max_load_tons: 34,
        flags: 0x8865ecb3,
    }
}

pub fn record_0234() -> CatalogRecord {
    CatalogRecord {
        id: 234,
        mnemonic: "QS-0234",
        min_clearance_cm: 478,
        max_load_tons: 47,
        flags: 0x8cc38bee,
    }
}

pub fn record_0235() -> CatalogRecord {
    CatalogRecord {
        id: 235,
        mnemonic: "QS-0235",
        min_clearance_cm: 495,
        max_load_tons: 60,
        flags: 0x70a12b29,
    }
}

pub fn record_0236() -> CatalogRecord {
    CatalogRecord {
        id: 236,
        mnemonic: "QS-0236",
        min_clearance_cm: 512,
        max_load_tons: 73,
        flags: 0x771eca64,
    }
}

pub fn record_0237() -> CatalogRecord {
    CatalogRecord {
        id: 237,
        mnemonic: "QS-0237",
        min_clearance_cm: 529,
        max_load_tons: 86,
        flags: 0x7bfc699f,
    }
}

pub fn record_0238() -> CatalogRecord {
    CatalogRecord {
        id: 238,
        mnemonic: "QS-0238",
        min_clearance_cm: 546,
        max_load_tons: 99,
        flags: 0x7e5a08da,
    }
}

pub fn record_0239() -> CatalogRecord {
    CatalogRecord {
        id: 239,
        mnemonic: "QS-0239",
        min_clearance_cm: 563,
        max_load_tons: 112,
        flags: 0x6237a815,
    }
}

pub fn record_0240() -> CatalogRecord {
    CatalogRecord {
        id: 240,
        mnemonic: "QS-0240",
        min_clearance_cm: 580,
        max_load_tons: 125,
        flags: 0x66954750,
    }
}

pub fn record_0241() -> CatalogRecord {
    CatalogRecord {
        id: 241,
        mnemonic: "QS-0241",
        min_clearance_cm: 597,
        max_load_tons: 13,
        flags: 0x6d72e68b,
    }
}

pub fn record_0242() -> CatalogRecord {
    CatalogRecord {
        id: 242,
        mnemonic: "QS-0242",
        min_clearance_cm: 614,
        max_load_tons: 26,
        flags: 0x51d085c6,
    }
}

pub fn record_0243() -> CatalogRecord {
    CatalogRecord {
        id: 243,
        mnemonic: "QS-0243",
        min_clearance_cm: 631,
        max_load_tons: 39,
        flags: 0x558e2501,
    }
}

pub fn record_0244() -> CatalogRecord {
    CatalogRecord {
        id: 244,
        mnemonic: "QS-0244",
        min_clearance_cm: 648,
        max_load_tons: 52,
        flags: 0x586bc43c,
    }
}

pub fn record_0245() -> CatalogRecord {
    CatalogRecord {
        id: 245,
        mnemonic: "QS-0245",
        min_clearance_cm: 665,
        max_load_tons: 65,
        flags: 0x5cc96377,
    }
}

pub fn record_0246() -> CatalogRecord {
    CatalogRecord {
        id: 246,
        mnemonic: "QS-0246",
        min_clearance_cm: 682,
        max_load_tons: 78,
        flags: 0x40a702b2,
    }
}

pub fn record_0247() -> CatalogRecord {
    CatalogRecord {
        id: 247,
        mnemonic: "QS-0247",
        min_clearance_cm: 699,
        max_load_tons: 91,
        flags: 0x4704a1ed,
    }
}

pub fn record_0248() -> CatalogRecord {
    CatalogRecord {
        id: 248,
        mnemonic: "QS-0248",
        min_clearance_cm: 716,
        max_load_tons: 104,
        flags: 0x4be24128,
    }
}

pub fn record_0249() -> CatalogRecord {
    CatalogRecord {
        id: 249,
        mnemonic: "QS-0249",
        min_clearance_cm: 733,
        max_load_tons: 117,
        flags: 0x4e5fe063,
    }
}

pub fn record_0250() -> CatalogRecord {
    CatalogRecord {
        id: 250,
        mnemonic: "QS-0250",
        min_clearance_cm: 750,
        max_load_tons: 5,
        flags: 0x323d7f9e,
    }
}

pub fn record_0251() -> CatalogRecord {
    CatalogRecord {
        id: 251,
        mnemonic: "QS-0251",
        min_clearance_cm: 767,
        max_load_tons: 18,
        flags: 0x369b1ed9,
    }
}

pub fn record_0252() -> CatalogRecord {
    CatalogRecord {
        id: 252,
        mnemonic: "QS-0252",
        min_clearance_cm: 784,
        max_load_tons: 31,
        flags: 0x3d78be14,
    }
}

pub fn record_0253() -> CatalogRecord {
    CatalogRecord {
        id: 253,
        mnemonic: "QS-0253",
        min_clearance_cm: 801,
        max_load_tons: 44,
        flags: 0x21d65d4f,
    }
}

pub fn record_0254() -> CatalogRecord {
    CatalogRecord {
        id: 254,
        mnemonic: "QS-0254",
        min_clearance_cm: 818,
        max_load_tons: 57,
        flags: 0x25b3fc8a,
    }
}

pub fn record_0255() -> CatalogRecord {
    CatalogRecord {
        id: 255,
        mnemonic: "QS-0255",
        min_clearance_cm: 835,
        max_load_tons: 70,
        flags: 0x28119bc5,
    }
}

pub fn record_0256() -> CatalogRecord {
    CatalogRecord {
        id: 256,
        mnemonic: "QS-0256",
        min_clearance_cm: 852,
        max_load_tons: 83,
        flags: 0x2ccf3b00,
    }
}

pub fn record_0257() -> CatalogRecord {
    CatalogRecord {
        id: 257,
        mnemonic: "QS-0257",
        min_clearance_cm: 869,
        max_load_tons: 96,
        flags: 0x10acda3b,
    }
}

pub fn record_0258() -> CatalogRecord {
    CatalogRecord {
        id: 258,
        mnemonic: "QS-0258",
        min_clearance_cm: 886,
        max_load_tons: 109,
        flags: 0x170a7976,
    }
}

pub fn record_0259() -> CatalogRecord {
    CatalogRecord {
        id: 259,
        mnemonic: "QS-0259",
        min_clearance_cm: 903,
        max_load_tons: 122,
        flags: 0x1be818b1,
    }
}

pub fn record_0260() -> CatalogRecord {
    CatalogRecord {
        id: 260,
        mnemonic: "QS-0260",
        min_clearance_cm: 920,
        max_load_tons: 10,
        flags: 0x1e45b7ec,
    }
}

pub fn record_0261() -> CatalogRecord {
    CatalogRecord {
        id: 261,
        mnemonic: "QS-0261",
        min_clearance_cm: 937,
        max_load_tons: 23,
        flags: 0x02235727,
    }
}

pub fn record_0262() -> CatalogRecord {
    CatalogRecord {
        id: 262,
        mnemonic: "QS-0262",
        min_clearance_cm: 954,
        max_load_tons: 36,
        flags: 0x0680f662,
    }
}

pub fn record_0263() -> CatalogRecord {
    CatalogRecord {
        id: 263,
        mnemonic: "QS-0263",
        min_clearance_cm: 971,
        max_load_tons: 49,
        flags: 0x0d7e959d,
    }
}

pub fn record_0264() -> CatalogRecord {
    CatalogRecord {
        id: 264,
        mnemonic: "QS-0264",
        min_clearance_cm: 988,
        max_load_tons: 62,
        flags: 0xf1dc34d8,
    }
}

pub fn record_0265() -> CatalogRecord {
    CatalogRecord {
        id: 265,
        mnemonic: "QS-0265",
        min_clearance_cm: 105,
        max_load_tons: 75,
        flags: 0xf5b9d413,
    }
}

pub fn record_0266() -> CatalogRecord {
    CatalogRecord {
        id: 266,
        mnemonic: "QS-0266",
        min_clearance_cm: 122,
        max_load_tons: 88,
        flags: 0xf817734e,
    }
}

pub fn record_0267() -> CatalogRecord {
    CatalogRecord {
        id: 267,
        mnemonic: "QS-0267",
        min_clearance_cm: 139,
        max_load_tons: 101,
        flags: 0xfcf51289,
    }
}

pub fn record_0268() -> CatalogRecord {
    CatalogRecord {
        id: 268,
        mnemonic: "QS-0268",
        min_clearance_cm: 156,
        max_load_tons: 114,
        flags: 0xe352b1c4,
    }
}

pub fn record_0269() -> CatalogRecord {
    CatalogRecord {
        id: 269,
        mnemonic: "QS-0269",
        min_clearance_cm: 173,
        max_load_tons: 127,
        flags: 0xe73050ff,
    }
}

pub fn record_0270() -> CatalogRecord {
    CatalogRecord {
        id: 270,
        mnemonic: "QS-0270",
        min_clearance_cm: 190,
        max_load_tons: 15,
        flags: 0xebedf03a,
    }
}

pub fn record_0271() -> CatalogRecord {
    CatalogRecord {
        id: 271,
        mnemonic: "QS-0271",
        min_clearance_cm: 207,
        max_load_tons: 28,
        flags: 0xee4b8f75,
    }
}

pub fn record_0272() -> CatalogRecord {
    CatalogRecord {
        id: 272,
        mnemonic: "QS-0272",
        min_clearance_cm: 224,
        max_load_tons: 41,
        flags: 0xd2292eb0,
    }
}

pub fn record_0273() -> CatalogRecord {
    CatalogRecord {
        id: 273,
        mnemonic: "QS-0273",
        min_clearance_cm: 241,
        max_load_tons: 54,
        flags: 0xd686cdeb,
    }
}

pub fn record_0274() -> CatalogRecord {
    CatalogRecord {
        id: 274,
        mnemonic: "QS-0274",
        min_clearance_cm: 258,
        max_load_tons: 67,
        flags: 0xdd646d26,
    }
}

pub fn record_0275() -> CatalogRecord {
    CatalogRecord {
        id: 275,
        mnemonic: "QS-0275",
        min_clearance_cm: 275,
        max_load_tons: 80,
        flags: 0xc1c20c61,
    }
}

pub fn record_0276() -> CatalogRecord {
    CatalogRecord {
        id: 276,
        mnemonic: "QS-0276",
        min_clearance_cm: 292,
        max_load_tons: 93,
        flags: 0xc5bfab9c,
    }
}

pub fn record_0277() -> CatalogRecord {
    CatalogRecord {
        id: 277,
        mnemonic: "QS-0277",
        min_clearance_cm: 309,
        max_load_tons: 106,
        flags: 0xc81d4ad7,
    }
}

pub fn record_0278() -> CatalogRecord {
    CatalogRecord {
        id: 278,
        mnemonic: "QS-0278",
        min_clearance_cm: 326,
        max_load_tons: 119,
        flags: 0xccfaea12,
    }
}

pub fn record_0279() -> CatalogRecord {
    CatalogRecord {
        id: 279,
        mnemonic: "QS-0279",
        min_clearance_cm: 343,
        max_load_tons: 7,
        flags: 0xb358894d,
    }
}

pub fn record_0280() -> CatalogRecord {
    CatalogRecord {
        id: 280,
        mnemonic: "QS-0280",
        min_clearance_cm: 360,
        max_load_tons: 20,
        flags: 0xb7362888,
    }
}

pub fn record_0281() -> CatalogRecord {
    CatalogRecord {
        id: 281,
        mnemonic: "QS-0281",
        min_clearance_cm: 377,
        max_load_tons: 33,
        flags: 0xbb93c7c3,
    }
}

pub fn record_0282() -> CatalogRecord {
    CatalogRecord {
        id: 282,
        mnemonic: "QS-0282",
        min_clearance_cm: 394,
        max_load_tons: 46,
        flags: 0xbe7166fe,
    }
}

pub fn record_0283() -> CatalogRecord {
    CatalogRecord {
        id: 283,
        mnemonic: "QS-0283",
        min_clearance_cm: 411,
        max_load_tons: 59,
        flags: 0xa22f0639,
    }
}

pub fn record_0284() -> CatalogRecord {
    CatalogRecord {
        id: 284,
        mnemonic: "QS-0284",
        min_clearance_cm: 428,
        max_load_tons: 72,
        flags: 0xa68ca574,
    }
}

pub fn record_0285() -> CatalogRecord {
    CatalogRecord {
        id: 285,
        mnemonic: "QS-0285",
        min_clearance_cm: 445,
        max_load_tons: 85,
        flags: 0xad6a44af,
    }
}

pub fn record_0286() -> CatalogRecord {
    CatalogRecord {
        id: 286,
        mnemonic: "QS-0286",
        min_clearance_cm: 462,
        max_load_tons: 98,
        flags: 0x91c7e3ea,
    }
}

pub fn record_0287() -> CatalogRecord {
    CatalogRecord {
        id: 287,
        mnemonic: "QS-0287",
        min_clearance_cm: 479,
        max_load_tons: 111,
        flags: 0x95a58325,
    }
}

pub fn record_0288() -> CatalogRecord {
    CatalogRecord {
        id: 288,
        mnemonic: "QS-0288",
        min_clearance_cm: 496,
        max_load_tons: 124,
        flags: 0x98032260,
    }
}

pub fn record_0289() -> CatalogRecord {
    CatalogRecord {
        id: 289,
        mnemonic: "QS-0289",
        min_clearance_cm: 513,
        max_load_tons: 12,
        flags: 0x9ce0c19b,
    }
}

pub fn record_0290() -> CatalogRecord {
    CatalogRecord {
        id: 290,
        mnemonic: "QS-0290",
        min_clearance_cm: 530,
        max_load_tons: 25,
        flags: 0x835e60d6,
    }
}

pub fn record_0291() -> CatalogRecord {
    CatalogRecord {
        id: 291,
        mnemonic: "QS-0291",
        min_clearance_cm: 547,
        max_load_tons: 38,
        flags: 0x873c0011,
    }
}

pub fn record_0292() -> CatalogRecord {
    CatalogRecord {
        id: 292,
        mnemonic: "QS-0292",
        min_clearance_cm: 564,
        max_load_tons: 51,
        flags: 0x8b999f4c,
    }
}

pub fn record_0293() -> CatalogRecord {
    CatalogRecord {
        id: 293,
        mnemonic: "QS-0293",
        min_clearance_cm: 581,
        max_load_tons: 64,
        flags: 0x8e773e87,
    }
}

pub fn record_0294() -> CatalogRecord {
    CatalogRecord {
        id: 294,
        mnemonic: "QS-0294",
        min_clearance_cm: 598,
        max_load_tons: 77,
        flags: 0x72d4ddc2,
    }
}

pub fn record_0295() -> CatalogRecord {
    CatalogRecord {
        id: 295,
        mnemonic: "QS-0295",
        min_clearance_cm: 615,
        max_load_tons: 90,
        flags: 0x76b27cfd,
    }
}

pub fn record_0296() -> CatalogRecord {
    CatalogRecord {
        id: 296,
        mnemonic: "QS-0296",
        min_clearance_cm: 632,
        max_load_tons: 103,
        flags: 0x7d101c38,
    }
}

pub fn record_0297() -> CatalogRecord {
    CatalogRecord {
        id: 297,
        mnemonic: "QS-0297",
        min_clearance_cm: 649,
        max_load_tons: 116,
        flags: 0x61cdbb73,
    }
}

pub fn record_0298() -> CatalogRecord {
    CatalogRecord {
        id: 298,
        mnemonic: "QS-0298",
        min_clearance_cm: 666,
        max_load_tons: 129,
        flags: 0x65ab5aae,
    }
}

pub fn record_0299() -> CatalogRecord {
    CatalogRecord {
        id: 299,
        mnemonic: "QS-0299",
        min_clearance_cm: 683,
        max_load_tons: 17,
        flags: 0x6808f9e9,
    }
}

pub fn record_0300() -> CatalogRecord {
    CatalogRecord {
        id: 300,
        mnemonic: "QS-0300",
        min_clearance_cm: 700,
        max_load_tons: 30,
        flags: 0x6ce69924,
    }
}

pub fn record_0301() -> CatalogRecord {
    CatalogRecord {
        id: 301,
        mnemonic: "QS-0301",
        min_clearance_cm: 717,
        max_load_tons: 43,
        flags: 0x5344385f,
    }
}

pub fn record_0302() -> CatalogRecord {
    CatalogRecord {
        id: 302,
        mnemonic: "QS-0302",
        min_clearance_cm: 734,
        max_load_tons: 56,
        flags: 0x5721d79a,
    }
}

pub fn record_0303() -> CatalogRecord {
    CatalogRecord {
        id: 303,
        mnemonic: "QS-0303",
        min_clearance_cm: 751,
        max_load_tons: 69,
        flags: 0x5b9f76d5,
    }
}

pub fn record_0304() -> CatalogRecord {
    CatalogRecord {
        id: 304,
        mnemonic: "QS-0304",
        min_clearance_cm: 768,
        max_load_tons: 82,
        flags: 0x5e7d1610,
    }
}

pub fn record_0305() -> CatalogRecord {
    CatalogRecord {
        id: 305,
        mnemonic: "QS-0305",
        min_clearance_cm: 785,
        max_load_tons: 95,
        flags: 0x42dab54b,
    }
}

pub fn record_0306() -> CatalogRecord {
    CatalogRecord {
        id: 306,
        mnemonic: "QS-0306",
        min_clearance_cm: 802,
        max_load_tons: 108,
        flags: 0x46b85486,
    }
}

pub fn record_0307() -> CatalogRecord {
    CatalogRecord {
        id: 307,
        mnemonic: "QS-0307",
        min_clearance_cm: 819,
        max_load_tons: 121,
        flags: 0x4d15f3c1,
    }
}

pub fn record_0308() -> CatalogRecord {
    CatalogRecord {
        id: 308,
        mnemonic: "QS-0308",
        min_clearance_cm: 836,
        max_load_tons: 9,
        flags: 0x31f392fc,
    }
}

pub fn record_0309() -> CatalogRecord {
    CatalogRecord {
        id: 309,
        mnemonic: "QS-0309",
        min_clearance_cm: 853,
        max_load_tons: 22,
        flags: 0x34513237,
    }
}

pub fn record_0310() -> CatalogRecord {
    CatalogRecord {
        id: 310,
        mnemonic: "QS-0310",
        min_clearance_cm: 870,
        max_load_tons: 35,
        flags: 0x380ed172,
    }
}

pub fn record_0311() -> CatalogRecord {
    CatalogRecord {
        id: 311,
        mnemonic: "QS-0311",
        min_clearance_cm: 887,
        max_load_tons: 48,
        flags: 0x3cec70ad,
    }
}

pub fn record_0312() -> CatalogRecord {
    CatalogRecord {
        id: 312,
        mnemonic: "QS-0312",
        min_clearance_cm: 904,
        max_load_tons: 61,
        flags: 0x234a0fe8,
    }
}

pub fn record_0313() -> CatalogRecord {
    CatalogRecord {
        id: 313,
        mnemonic: "QS-0313",
        min_clearance_cm: 921,
        max_load_tons: 74,
        flags: 0x2727af23,
    }
}

pub fn record_0314() -> CatalogRecord {
    CatalogRecord {
        id: 314,
        mnemonic: "QS-0314",
        min_clearance_cm: 938,
        max_load_tons: 87,
        flags: 0x2b854e5e,
    }
}

pub fn record_0315() -> CatalogRecord {
    CatalogRecord {
        id: 315,
        mnemonic: "QS-0315",
        min_clearance_cm: 955,
        max_load_tons: 100,
        flags: 0x2e62ed99,
    }
}

pub fn record_0316() -> CatalogRecord {
    CatalogRecord {
        id: 316,
        mnemonic: "QS-0316",
        min_clearance_cm: 972,
        max_load_tons: 113,
        flags: 0x12c08cd4,
    }
}

pub fn record_0317() -> CatalogRecord {
    CatalogRecord {
        id: 317,
        mnemonic: "QS-0317",
        min_clearance_cm: 989,
        max_load_tons: 126,
        flags: 0x16be2c0f,
    }
}

pub fn record_0318() -> CatalogRecord {
    CatalogRecord {
        id: 318,
        mnemonic: "QS-0318",
        min_clearance_cm: 106,
        max_load_tons: 14,
        flags: 0x1d1bcb4a,
    }
}

pub fn record_0319() -> CatalogRecord {
    CatalogRecord {
        id: 319,
        mnemonic: "QS-0319",
        min_clearance_cm: 123,
        max_load_tons: 27,
        flags: 0x01f96a85,
    }
}

pub fn record_0320() -> CatalogRecord {
    CatalogRecord {
        id: 320,
        mnemonic: "QS-0320",
        min_clearance_cm: 140,
        max_load_tons: 40,
        flags: 0x045709c0,
    }
}

pub fn record_0321() -> CatalogRecord {
    CatalogRecord {
        id: 321,
        mnemonic: "QS-0321",
        min_clearance_cm: 157,
        max_load_tons: 53,
        flags: 0x0834a8fb,
    }
}

pub fn record_0322() -> CatalogRecord {
    CatalogRecord {
        id: 322,
        mnemonic: "QS-0322",
        min_clearance_cm: 174,
        max_load_tons: 66,
        flags: 0x0c924836,
    }
}

pub fn record_0323() -> CatalogRecord {
    CatalogRecord {
        id: 323,
        mnemonic: "QS-0323",
        min_clearance_cm: 191,
        max_load_tons: 79,
        flags: 0xf34fe771,
    }
}

pub fn record_0324() -> CatalogRecord {
    CatalogRecord {
        id: 324,
        mnemonic: "QS-0324",
        min_clearance_cm: 208,
        max_load_tons: 92,
        flags: 0xf72d86ac,
    }
}

pub fn record_0325() -> CatalogRecord {
    CatalogRecord {
        id: 325,
        mnemonic: "QS-0325",
        min_clearance_cm: 225,
        max_load_tons: 105,
        flags: 0xfb8b25e7,
    }
}

pub fn record_0326() -> CatalogRecord {
    CatalogRecord {
        id: 326,
        mnemonic: "QS-0326",
        min_clearance_cm: 242,
        max_load_tons: 118,
        flags: 0xfe68c522,
    }
}

pub fn record_0327() -> CatalogRecord {
    CatalogRecord {
        id: 327,
        mnemonic: "QS-0327",
        min_clearance_cm: 259,
        max_load_tons: 6,
        flags: 0xe2c6645d,
    }
}

pub fn record_0328() -> CatalogRecord {
    CatalogRecord {
        id: 328,
        mnemonic: "QS-0328",
        min_clearance_cm: 276,
        max_load_tons: 19,
        flags: 0xe6a40398,
    }
}

pub fn record_0329() -> CatalogRecord {
    CatalogRecord {
        id: 329,
        mnemonic: "QS-0329",
        min_clearance_cm: 293,
        max_load_tons: 32,
        flags: 0xed01a2d3,
    }
}

pub fn record_0330() -> CatalogRecord {
    CatalogRecord {
        id: 330,
        mnemonic: "QS-0330",
        min_clearance_cm: 310,
        max_load_tons: 45,
        flags: 0xd1ff420e,
    }
}

pub fn record_0331() -> CatalogRecord {
    CatalogRecord {
        id: 331,
        mnemonic: "QS-0331",
        min_clearance_cm: 327,
        max_load_tons: 58,
        flags: 0xd45ce149,
    }
}

pub fn record_0332() -> CatalogRecord {
    CatalogRecord {
        id: 332,
        mnemonic: "QS-0332",
        min_clearance_cm: 344,
        max_load_tons: 71,
        flags: 0xd83a8084,
    }
}

pub fn record_0333() -> CatalogRecord {
    CatalogRecord {
        id: 333,
        mnemonic: "QS-0333",
        min_clearance_cm: 361,
        max_load_tons: 84,
        flags: 0xdc981fbf,
    }
}

pub fn record_0334() -> CatalogRecord {
    CatalogRecord {
        id: 334,
        mnemonic: "QS-0334",
        min_clearance_cm: 378,
        max_load_tons: 97,
        flags: 0xc375befa,
    }
}

pub fn record_0335() -> CatalogRecord {
    CatalogRecord {
        id: 335,
        mnemonic: "QS-0335",
        min_clearance_cm: 395,
        max_load_tons: 110,
        flags: 0xc7d35e35,
    }
}

pub fn record_0336() -> CatalogRecord {
    CatalogRecord {
        id: 336,
        mnemonic: "QS-0336",
        min_clearance_cm: 412,
        max_load_tons: 123,
        flags: 0xcbb0fd70,
    }
}

pub fn record_0337() -> CatalogRecord {
    CatalogRecord {
        id: 337,
        mnemonic: "QS-0337",
        min_clearance_cm: 429,
        max_load_tons: 11,
        flags: 0xce6e9cab,
    }
}

pub fn record_0338() -> CatalogRecord {
    CatalogRecord {
        id: 338,
        mnemonic: "QS-0338",
        min_clearance_cm: 446,
        max_load_tons: 24,
        flags: 0xb2cc3be6,
    }
}

pub fn record_0339() -> CatalogRecord {
    CatalogRecord {
        id: 339,
        mnemonic: "QS-0339",
        min_clearance_cm: 463,
        max_load_tons: 37,
        flags: 0xb6a9db21,
    }
}

pub fn record_0340() -> CatalogRecord {
    CatalogRecord {
        id: 340,
        mnemonic: "QS-0340",
        min_clearance_cm: 480,
        max_load_tons: 50,
        flags: 0xbd077a5c,
    }
}

pub fn record_0341() -> CatalogRecord {
    CatalogRecord {
        id: 341,
        mnemonic: "QS-0341",
        min_clearance_cm: 497,
        max_load_tons: 63,
        flags: 0xa1e51997,
    }
}

pub fn record_0342() -> CatalogRecord {
    CatalogRecord {
        id: 342,
        mnemonic: "QS-0342",
        min_clearance_cm: 514,
        max_load_tons: 76,
        flags: 0xa442b8d2,
    }
}

pub fn record_0343() -> CatalogRecord {
    CatalogRecord {
        id: 343,
        mnemonic: "QS-0343",
        min_clearance_cm: 531,
        max_load_tons: 89,
        flags: 0xa820580d,
    }
}

pub fn record_0344() -> CatalogRecord {
    CatalogRecord {
        id: 344,
        mnemonic: "QS-0344",
        min_clearance_cm: 548,
        max_load_tons: 102,
        flags: 0xac9df748,
    }
}

pub fn record_0345() -> CatalogRecord {
    CatalogRecord {
        id: 345,
        mnemonic: "QS-0345",
        min_clearance_cm: 565,
        max_load_tons: 115,
        flags: 0x937b9683,
    }
}

pub fn record_0346() -> CatalogRecord {
    CatalogRecord {
        id: 346,
        mnemonic: "QS-0346",
        min_clearance_cm: 582,
        max_load_tons: 128,
        flags: 0x97d935be,
    }
}

pub fn record_0347() -> CatalogRecord {
    CatalogRecord {
        id: 347,
        mnemonic: "QS-0347",
        min_clearance_cm: 599,
        max_load_tons: 16,
        flags: 0x9bb6d4f9,
    }
}

pub fn record_0348() -> CatalogRecord {
    CatalogRecord {
        id: 348,
        mnemonic: "QS-0348",
        min_clearance_cm: 616,
        max_load_tons: 29,
        flags: 0x9e147434,
    }
}

pub fn record_0349() -> CatalogRecord {
    CatalogRecord {
        id: 349,
        mnemonic: "QS-0349",
        min_clearance_cm: 633,
        max_load_tons: 42,
        flags: 0x82f2136f,
    }
}

pub fn record_0350() -> CatalogRecord {
    CatalogRecord {
        id: 350,
        mnemonic: "QS-0350",
        min_clearance_cm: 650,
        max_load_tons: 55,
        flags: 0x86afb2aa,
    }
}

pub fn record_0351() -> CatalogRecord {
    CatalogRecord {
        id: 351,
        mnemonic: "QS-0351",
        min_clearance_cm: 667,
        max_load_tons: 68,
        flags: 0x8d0d51e5,
    }
}

pub fn record_0352() -> CatalogRecord {
    CatalogRecord {
        id: 352,
        mnemonic: "QS-0352",
        min_clearance_cm: 684,
        max_load_tons: 81,
        flags: 0x71eaf120,
    }
}

pub fn record_0353() -> CatalogRecord {
    CatalogRecord {
        id: 353,
        mnemonic: "QS-0353",
        min_clearance_cm: 701,
        max_load_tons: 94,
        flags: 0x7448905b,
    }
}

pub fn record_0354() -> CatalogRecord {
    CatalogRecord {
        id: 354,
        mnemonic: "QS-0354",
        min_clearance_cm: 718,
        max_load_tons: 107,
        flags: 0x78262f96,
    }
}

pub fn record_0355() -> CatalogRecord {
    CatalogRecord {
        id: 355,
        mnemonic: "QS-0355",
        min_clearance_cm: 735,
        max_load_tons: 120,
        flags: 0x7c83ced1,
    }
}

pub fn record_0356() -> CatalogRecord {
    CatalogRecord {
        id: 356,
        mnemonic: "QS-0356",
        min_clearance_cm: 752,
        max_load_tons: 8,
        flags: 0x63616e0c,
    }
}

pub fn record_0357() -> CatalogRecord {
    CatalogRecord {
        id: 357,
        mnemonic: "QS-0357",
        min_clearance_cm: 769,
        max_load_tons: 21,
        flags: 0x67df0d47,
    }
}

pub fn record_0358() -> CatalogRecord {
    CatalogRecord {
        id: 358,
        mnemonic: "QS-0358",
        min_clearance_cm: 786,
        max_load_tons: 34,
        flags: 0x6bbcac82,
    }
}

pub fn record_0359() -> CatalogRecord {
    CatalogRecord {
        id: 359,
        mnemonic: "QS-0359",
        min_clearance_cm: 803,
        max_load_tons: 47,
        flags: 0x6e1a4bbd,
    }
}

pub fn record_0360() -> CatalogRecord {
    CatalogRecord {
        id: 360,
        mnemonic: "QS-0360",
        min_clearance_cm: 820,
        max_load_tons: 60,
        flags: 0x52f7eaf8,
    }
}

pub fn record_0361() -> CatalogRecord {
    CatalogRecord {
        id: 361,
        mnemonic: "QS-0361",
        min_clearance_cm: 837,
        max_load_tons: 73,
        flags: 0x59558a33,
    }
}

pub fn record_0362() -> CatalogRecord {
    CatalogRecord {
        id: 362,
        mnemonic: "QS-0362",
        min_clearance_cm: 854,
        max_load_tons: 86,
        flags: 0x5d33296e,
    }
}

pub fn record_0363() -> CatalogRecord {
    CatalogRecord {
        id: 363,
        mnemonic: "QS-0363",
        min_clearance_cm: 871,
        max_load_tons: 99,
        flags: 0x4190c8a9,
    }
}

pub fn record_0364() -> CatalogRecord {
    CatalogRecord {
        id: 364,
        mnemonic: "QS-0364",
        min_clearance_cm: 888,
        max_load_tons: 112,
        flags: 0x444e67e4,
    }
}

pub fn record_0365() -> CatalogRecord {
    CatalogRecord {
        id: 365,
        mnemonic: "QS-0365",
        min_clearance_cm: 905,
        max_load_tons: 125,
        flags: 0x482c071f,
    }
}

pub fn record_0366() -> CatalogRecord {
    CatalogRecord {
        id: 366,
        mnemonic: "QS-0366",
        min_clearance_cm: 922,
        max_load_tons: 13,
        flags: 0x4c89a65a,
    }
}

pub fn record_0367() -> CatalogRecord {
    CatalogRecord {
        id: 367,
        mnemonic: "QS-0367",
        min_clearance_cm: 939,
        max_load_tons: 26,
        flags: 0x33674595,
    }
}

pub fn record_0368() -> CatalogRecord {
    CatalogRecord {
        id: 368,
        mnemonic: "QS-0368",
        min_clearance_cm: 956,
        max_load_tons: 39,
        flags: 0x37c4e4d0,
    }
}

pub fn record_0369() -> CatalogRecord {
    CatalogRecord {
        id: 369,
        mnemonic: "QS-0369",
        min_clearance_cm: 973,
        max_load_tons: 52,
        flags: 0x3ba2840b,
    }
}

pub fn record_0370() -> CatalogRecord {
    CatalogRecord {
        id: 370,
        mnemonic: "QS-0370",
        min_clearance_cm: 990,
        max_load_tons: 65,
        flags: 0x3e002346,
    }
}

pub fn record_0371() -> CatalogRecord {
    CatalogRecord {
        id: 371,
        mnemonic: "QS-0371",
        min_clearance_cm: 107,
        max_load_tons: 78,
        flags: 0x22fdc281,
    }
}

pub fn record_0372() -> CatalogRecord {
    CatalogRecord {
        id: 372,
        mnemonic: "QS-0372",
        min_clearance_cm: 124,
        max_load_tons: 91,
        flags: 0x295b61bc,
    }
}

pub fn record_0373() -> CatalogRecord {
    CatalogRecord {
        id: 373,
        mnemonic: "QS-0373",
        min_clearance_cm: 141,
        max_load_tons: 104,
        flags: 0x2d3900f7,
    }
}

pub fn record_0374() -> CatalogRecord {
    CatalogRecord {
        id: 374,
        mnemonic: "QS-0374",
        min_clearance_cm: 158,
        max_load_tons: 117,
        flags: 0x1196a032,
    }
}

pub fn record_0375() -> CatalogRecord {
    CatalogRecord {
        id: 375,
        mnemonic: "QS-0375",
        min_clearance_cm: 175,
        max_load_tons: 5,
        flags: 0x14743f6d,
    }
}

pub fn record_0376() -> CatalogRecord {
    CatalogRecord {
        id: 376,
        mnemonic: "QS-0376",
        min_clearance_cm: 192,
        max_load_tons: 18,
        flags: 0x18d1dea8,
    }
}

pub fn record_0377() -> CatalogRecord {
    CatalogRecord {
        id: 377,
        mnemonic: "QS-0377",
        min_clearance_cm: 209,
        max_load_tons: 31,
        flags: 0x1c8f7de3,
    }
}

pub fn record_0378() -> CatalogRecord {
    CatalogRecord {
        id: 378,
        mnemonic: "QS-0378",
        min_clearance_cm: 226,
        max_load_tons: 44,
        flags: 0x036d1d1e,
    }
}

pub fn record_0379() -> CatalogRecord {
    CatalogRecord {
        id: 379,
        mnemonic: "QS-0379",
        min_clearance_cm: 243,
        max_load_tons: 57,
        flags: 0x07cabc59,
    }
}

pub fn record_0380() -> CatalogRecord {
    CatalogRecord {
        id: 380,
        mnemonic: "QS-0380",
        min_clearance_cm: 260,
        max_load_tons: 70,
        flags: 0x0ba85b94,
    }
}

pub fn record_0381() -> CatalogRecord {
    CatalogRecord {
        id: 381,
        mnemonic: "QS-0381",
        min_clearance_cm: 277,
        max_load_tons: 83,
        flags: 0x0e05facf,
    }
}

pub fn record_0382() -> CatalogRecord {
    CatalogRecord {
        id: 382,
        mnemonic: "QS-0382",
        min_clearance_cm: 294,
        max_load_tons: 96,
        flags: 0xf2e39a0a,
    }
}

pub fn record_0383() -> CatalogRecord {
    CatalogRecord {
        id: 383,
        mnemonic: "QS-0383",
        min_clearance_cm: 311,
        max_load_tons: 109,
        flags: 0xf9413945,
    }
}

pub fn record_0384() -> CatalogRecord {
    CatalogRecord {
        id: 384,
        mnemonic: "QS-0384",
        min_clearance_cm: 328,
        max_load_tons: 122,
        flags: 0xfd3ed880,
    }
}

pub fn record_0385() -> CatalogRecord {
    CatalogRecord {
        id: 385,
        mnemonic: "QS-0385",
        min_clearance_cm: 345,
        max_load_tons: 10,
        flags: 0xe19c77bb,
    }
}

pub fn record_0386() -> CatalogRecord {
    CatalogRecord {
        id: 386,
        mnemonic: "QS-0386",
        min_clearance_cm: 362,
        max_load_tons: 23,
        flags: 0xe47a16f6,
    }
}

pub fn record_0387() -> CatalogRecord {
    CatalogRecord {
        id: 387,
        mnemonic: "QS-0387",
        min_clearance_cm: 379,
        max_load_tons: 36,
        flags: 0xe8d7b631,
    }
}

pub fn record_0388() -> CatalogRecord {
    CatalogRecord {
        id: 388,
        mnemonic: "QS-0388",
        min_clearance_cm: 396,
        max_load_tons: 49,
        flags: 0xecb5556c,
    }
}

pub fn record_0389() -> CatalogRecord {
    CatalogRecord {
        id: 389,
        mnemonic: "QS-0389",
        min_clearance_cm: 413,
        max_load_tons: 62,
        flags: 0xd312f4a7,
    }
}

pub fn record_0390() -> CatalogRecord {
    CatalogRecord {
        id: 390,
        mnemonic: "QS-0390",
        min_clearance_cm: 430,
        max_load_tons: 75,
        flags: 0xd7f093e2,
    }
}

pub fn record_0391() -> CatalogRecord {
    CatalogRecord {
        id: 391,
        mnemonic: "QS-0391",
        min_clearance_cm: 447,
        max_load_tons: 88,
        flags: 0xdbae331d,
    }
}

pub fn record_0392() -> CatalogRecord {
    CatalogRecord {
        id: 392,
        mnemonic: "QS-0392",
        min_clearance_cm: 464,
        max_load_tons: 101,
        flags: 0xde0bd258,
    }
}

pub fn record_0393() -> CatalogRecord {
    CatalogRecord {
        id: 393,
        mnemonic: "QS-0393",
        min_clearance_cm: 481,
        max_load_tons: 114,
        flags: 0xc2e97193,
    }
}

pub fn record_0394() -> CatalogRecord {
    CatalogRecord {
        id: 394,
        mnemonic: "QS-0394",
        min_clearance_cm: 498,
        max_load_tons: 127,
        flags: 0xc94710ce,
    }
}

pub fn record_0395() -> CatalogRecord {
    CatalogRecord {
        id: 395,
        mnemonic: "QS-0395",
        min_clearance_cm: 515,
        max_load_tons: 15,
        flags: 0xcd24b009,
    }
}

pub fn record_0396() -> CatalogRecord {
    CatalogRecord {
        id: 396,
        mnemonic: "QS-0396",
        min_clearance_cm: 532,
        max_load_tons: 28,
        flags: 0xb1824f44,
    }
}

pub fn record_0397() -> CatalogRecord {
    CatalogRecord {
        id: 397,
        mnemonic: "QS-0397",
        min_clearance_cm: 549,
        max_load_tons: 41,
        flags: 0xb47fee7f,
    }
}

pub fn record_0398() -> CatalogRecord {
    CatalogRecord {
        id: 398,
        mnemonic: "QS-0398",
        min_clearance_cm: 566,
        max_load_tons: 54,
        flags: 0xb8dd8dba,
    }
}

pub fn record_0399() -> CatalogRecord {
    CatalogRecord {
        id: 399,
        mnemonic: "QS-0399",
        min_clearance_cm: 583,
        max_load_tons: 67,
        flags: 0xbcbb2cf5,
    }
}

pub fn record_0400() -> CatalogRecord {
    CatalogRecord {
        id: 400,
        mnemonic: "QS-0400",
        min_clearance_cm: 600,
        max_load_tons: 80,
        flags: 0xa318cc30,
    }
}

pub fn record_0401() -> CatalogRecord {
    CatalogRecord {
        id: 401,
        mnemonic: "QS-0401",
        min_clearance_cm: 617,
        max_load_tons: 93,
        flags: 0xa7f66b6b,
    }
}

pub fn record_0402() -> CatalogRecord {
    CatalogRecord {
        id: 402,
        mnemonic: "QS-0402",
        min_clearance_cm: 634,
        max_load_tons: 106,
        flags: 0xaa540aa6,
    }
}

pub fn record_0403() -> CatalogRecord {
    CatalogRecord {
        id: 403,
        mnemonic: "QS-0403",
        min_clearance_cm: 651,
        max_load_tons: 119,
        flags: 0xae31a9e1,
    }
}

pub fn record_0404() -> CatalogRecord {
    CatalogRecord {
        id: 404,
        mnemonic: "QS-0404",
        min_clearance_cm: 668,
        max_load_tons: 7,
        flags: 0x92ef491c,
    }
}

pub fn record_0405() -> CatalogRecord {
    CatalogRecord {
        id: 405,
        mnemonic: "QS-0405",
        min_clearance_cm: 685,
        max_load_tons: 20,
        flags: 0x994ce857,
    }
}

pub fn record_0406() -> CatalogRecord {
    CatalogRecord {
        id: 406,
        mnemonic: "QS-0406",
        min_clearance_cm: 702,
        max_load_tons: 33,
        flags: 0x9d2a8792,
    }
}

pub fn record_0407() -> CatalogRecord {
    CatalogRecord {
        id: 407,
        mnemonic: "QS-0407",
        min_clearance_cm: 719,
        max_load_tons: 46,
        flags: 0x818826cd,
    }
}

pub fn record_0408() -> CatalogRecord {
    CatalogRecord {
        id: 408,
        mnemonic: "QS-0408",
        min_clearance_cm: 736,
        max_load_tons: 59,
        flags: 0x8465c608,
    }
}

pub fn record_0409() -> CatalogRecord {
    CatalogRecord {
        id: 409,
        mnemonic: "QS-0409",
        min_clearance_cm: 753,
        max_load_tons: 72,
        flags: 0x88c36543,
    }
}

pub fn record_0410() -> CatalogRecord {
    CatalogRecord {
        id: 410,
        mnemonic: "QS-0410",
        min_clearance_cm: 770,
        max_load_tons: 85,
        flags: 0x8ca1047e,
    }
}

pub fn record_0411() -> CatalogRecord {
    CatalogRecord {
        id: 411,
        mnemonic: "QS-0411",
        min_clearance_cm: 787,
        max_load_tons: 98,
        flags: 0x731ea3b9,
    }
}

pub fn record_0412() -> CatalogRecord {
    CatalogRecord {
        id: 412,
        mnemonic: "QS-0412",
        min_clearance_cm: 804,
        max_load_tons: 111,
        flags: 0x77fc42f4,
    }
}

pub fn record_0413() -> CatalogRecord {
    CatalogRecord {
        id: 413,
        mnemonic: "QS-0413",
        min_clearance_cm: 821,
        max_load_tons: 124,
        flags: 0x7a59e22f,
    }
}

pub fn record_0414() -> CatalogRecord {
    CatalogRecord {
        id: 414,
        mnemonic: "QS-0414",
        min_clearance_cm: 838,
        max_load_tons: 12,
        flags: 0x7e37816a,
    }
}

pub fn record_0415() -> CatalogRecord {
    CatalogRecord {
        id: 415,
        mnemonic: "QS-0415",
        min_clearance_cm: 855,
        max_load_tons: 25,
        flags: 0x629520a5,
    }
}

pub fn record_0416() -> CatalogRecord {
    CatalogRecord {
        id: 416,
        mnemonic: "QS-0416",
        min_clearance_cm: 872,
        max_load_tons: 38,
        flags: 0x6972bfe0,
    }
}

pub fn record_0417() -> CatalogRecord {
    CatalogRecord {
        id: 417,
        mnemonic: "QS-0417",
        min_clearance_cm: 889,
        max_load_tons: 51,
        flags: 0x6dd05f1b,
    }
}

pub fn record_0418() -> CatalogRecord {
    CatalogRecord {
        id: 418,
        mnemonic: "QS-0418",
        min_clearance_cm: 906,
        max_load_tons: 64,
        flags: 0x518dfe56,
    }
}

pub fn record_0419() -> CatalogRecord {
    CatalogRecord {
        id: 419,
        mnemonic: "QS-0419",
        min_clearance_cm: 923,
        max_load_tons: 77,
        flags: 0x546b9d91,
    }
}

pub fn record_0420() -> CatalogRecord {
    CatalogRecord {
        id: 420,
        mnemonic: "QS-0420",
        min_clearance_cm: 940,
        max_load_tons: 90,
        flags: 0x58c93ccc,
    }
}

pub fn lookup_record(id: u16) -> Option<CatalogRecord> {
    match id {
        1 => Some(record_0001()),
        2 => Some(record_0002()),
        3 => Some(record_0003()),
        4 => Some(record_0004()),
        5 => Some(record_0005()),
        6 => Some(record_0006()),
        7 => Some(record_0007()),
        8 => Some(record_0008()),
        9 => Some(record_0009()),
        10 => Some(record_0010()),
        11 => Some(record_0011()),
        12 => Some(record_0012()),
        13 => Some(record_0013()),
        14 => Some(record_0014()),
        15 => Some(record_0015()),
        16 => Some(record_0016()),
        17 => Some(record_0017()),
        18 => Some(record_0018()),
        19 => Some(record_0019()),
        20 => Some(record_0020()),
        21 => Some(record_0021()),
        22 => Some(record_0022()),
        23 => Some(record_0023()),
        24 => Some(record_0024()),
        25 => Some(record_0025()),
        26 => Some(record_0026()),
        27 => Some(record_0027()),
        28 => Some(record_0028()),
        29 => Some(record_0029()),
        30 => Some(record_0030()),
        31 => Some(record_0031()),
        32 => Some(record_0032()),
        33 => Some(record_0033()),
        34 => Some(record_0034()),
        35 => Some(record_0035()),
        36 => Some(record_0036()),
        37 => Some(record_0037()),
        38 => Some(record_0038()),
        39 => Some(record_0039()),
        40 => Some(record_0040()),
        41 => Some(record_0041()),
        42 => Some(record_0042()),
        43 => Some(record_0043()),
        44 => Some(record_0044()),
        45 => Some(record_0045()),
        46 => Some(record_0046()),
        47 => Some(record_0047()),
        48 => Some(record_0048()),
        49 => Some(record_0049()),
        50 => Some(record_0050()),
        51 => Some(record_0051()),
        52 => Some(record_0052()),
        53 => Some(record_0053()),
        54 => Some(record_0054()),
        55 => Some(record_0055()),
        56 => Some(record_0056()),
        57 => Some(record_0057()),
        58 => Some(record_0058()),
        59 => Some(record_0059()),
        60 => Some(record_0060()),
        61 => Some(record_0061()),
        62 => Some(record_0062()),
        63 => Some(record_0063()),
        64 => Some(record_0064()),
        65 => Some(record_0065()),
        66 => Some(record_0066()),
        67 => Some(record_0067()),
        68 => Some(record_0068()),
        69 => Some(record_0069()),
        70 => Some(record_0070()),
        71 => Some(record_0071()),
        72 => Some(record_0072()),
        73 => Some(record_0073()),
        74 => Some(record_0074()),
        75 => Some(record_0075()),
        76 => Some(record_0076()),
        77 => Some(record_0077()),
        78 => Some(record_0078()),
        79 => Some(record_0079()),
        80 => Some(record_0080()),
        81 => Some(record_0081()),
        82 => Some(record_0082()),
        83 => Some(record_0083()),
        84 => Some(record_0084()),
        85 => Some(record_0085()),
        86 => Some(record_0086()),
        87 => Some(record_0087()),
        88 => Some(record_0088()),
        89 => Some(record_0089()),
        90 => Some(record_0090()),
        91 => Some(record_0091()),
        92 => Some(record_0092()),
        93 => Some(record_0093()),
        94 => Some(record_0094()),
        95 => Some(record_0095()),
        96 => Some(record_0096()),
        97 => Some(record_0097()),
        98 => Some(record_0098()),
        99 => Some(record_0099()),
        100 => Some(record_0100()),
        101 => Some(record_0101()),
        102 => Some(record_0102()),
        103 => Some(record_0103()),
        104 => Some(record_0104()),
        105 => Some(record_0105()),
        106 => Some(record_0106()),
        107 => Some(record_0107()),
        108 => Some(record_0108()),
        109 => Some(record_0109()),
        110 => Some(record_0110()),
        111 => Some(record_0111()),
        112 => Some(record_0112()),
        113 => Some(record_0113()),
        114 => Some(record_0114()),
        115 => Some(record_0115()),
        116 => Some(record_0116()),
        117 => Some(record_0117()),
        118 => Some(record_0118()),
        119 => Some(record_0119()),
        120 => Some(record_0120()),
        121 => Some(record_0121()),
        122 => Some(record_0122()),
        123 => Some(record_0123()),
        124 => Some(record_0124()),
        125 => Some(record_0125()),
        126 => Some(record_0126()),
        127 => Some(record_0127()),
        128 => Some(record_0128()),
        129 => Some(record_0129()),
        130 => Some(record_0130()),
        131 => Some(record_0131()),
        132 => Some(record_0132()),
        133 => Some(record_0133()),
        134 => Some(record_0134()),
        135 => Some(record_0135()),
        136 => Some(record_0136()),
        137 => Some(record_0137()),
        138 => Some(record_0138()),
        139 => Some(record_0139()),
        140 => Some(record_0140()),
        141 => Some(record_0141()),
        142 => Some(record_0142()),
        143 => Some(record_0143()),
        144 => Some(record_0144()),
        145 => Some(record_0145()),
        146 => Some(record_0146()),
        147 => Some(record_0147()),
        148 => Some(record_0148()),
        149 => Some(record_0149()),
        150 => Some(record_0150()),
        151 => Some(record_0151()),
        152 => Some(record_0152()),
        153 => Some(record_0153()),
        154 => Some(record_0154()),
        155 => Some(record_0155()),
        156 => Some(record_0156()),
        157 => Some(record_0157()),
        158 => Some(record_0158()),
        159 => Some(record_0159()),
        160 => Some(record_0160()),
        161 => Some(record_0161()),
        162 => Some(record_0162()),
        163 => Some(record_0163()),
        164 => Some(record_0164()),
        165 => Some(record_0165()),
        166 => Some(record_0166()),
        167 => Some(record_0167()),
        168 => Some(record_0168()),
        169 => Some(record_0169()),
        170 => Some(record_0170()),
        171 => Some(record_0171()),
        172 => Some(record_0172()),
        173 => Some(record_0173()),
        174 => Some(record_0174()),
        175 => Some(record_0175()),
        176 => Some(record_0176()),
        177 => Some(record_0177()),
        178 => Some(record_0178()),
        179 => Some(record_0179()),
        180 => Some(record_0180()),
        181 => Some(record_0181()),
        182 => Some(record_0182()),
        183 => Some(record_0183()),
        184 => Some(record_0184()),
        185 => Some(record_0185()),
        186 => Some(record_0186()),
        187 => Some(record_0187()),
        188 => Some(record_0188()),
        189 => Some(record_0189()),
        190 => Some(record_0190()),
        191 => Some(record_0191()),
        192 => Some(record_0192()),
        193 => Some(record_0193()),
        194 => Some(record_0194()),
        195 => Some(record_0195()),
        196 => Some(record_0196()),
        197 => Some(record_0197()),
        198 => Some(record_0198()),
        199 => Some(record_0199()),
        200 => Some(record_0200()),
        201 => Some(record_0201()),
        202 => Some(record_0202()),
        203 => Some(record_0203()),
        204 => Some(record_0204()),
        205 => Some(record_0205()),
        206 => Some(record_0206()),
        207 => Some(record_0207()),
        208 => Some(record_0208()),
        209 => Some(record_0209()),
        210 => Some(record_0210()),
        211 => Some(record_0211()),
        212 => Some(record_0212()),
        213 => Some(record_0213()),
        214 => Some(record_0214()),
        215 => Some(record_0215()),
        216 => Some(record_0216()),
        217 => Some(record_0217()),
        218 => Some(record_0218()),
        219 => Some(record_0219()),
        220 => Some(record_0220()),
        221 => Some(record_0221()),
        222 => Some(record_0222()),
        223 => Some(record_0223()),
        224 => Some(record_0224()),
        225 => Some(record_0225()),
        226 => Some(record_0226()),
        227 => Some(record_0227()),
        228 => Some(record_0228()),
        229 => Some(record_0229()),
        230 => Some(record_0230()),
        231 => Some(record_0231()),
        232 => Some(record_0232()),
        233 => Some(record_0233()),
        234 => Some(record_0234()),
        235 => Some(record_0235()),
        236 => Some(record_0236()),
        237 => Some(record_0237()),
        238 => Some(record_0238()),
        239 => Some(record_0239()),
        240 => Some(record_0240()),
        241 => Some(record_0241()),
        242 => Some(record_0242()),
        243 => Some(record_0243()),
        244 => Some(record_0244()),
        245 => Some(record_0245()),
        246 => Some(record_0246()),
        247 => Some(record_0247()),
        248 => Some(record_0248()),
        249 => Some(record_0249()),
        250 => Some(record_0250()),
        251 => Some(record_0251()),
        252 => Some(record_0252()),
        253 => Some(record_0253()),
        254 => Some(record_0254()),
        255 => Some(record_0255()),
        256 => Some(record_0256()),
        257 => Some(record_0257()),
        258 => Some(record_0258()),
        259 => Some(record_0259()),
        260 => Some(record_0260()),
        261 => Some(record_0261()),
        262 => Some(record_0262()),
        263 => Some(record_0263()),
        264 => Some(record_0264()),
        265 => Some(record_0265()),
        266 => Some(record_0266()),
        267 => Some(record_0267()),
        268 => Some(record_0268()),
        269 => Some(record_0269()),
        270 => Some(record_0270()),
        271 => Some(record_0271()),
        272 => Some(record_0272()),
        273 => Some(record_0273()),
        274 => Some(record_0274()),
        275 => Some(record_0275()),
        276 => Some(record_0276()),
        277 => Some(record_0277()),
        278 => Some(record_0278()),
        279 => Some(record_0279()),
        280 => Some(record_0280()),
        281 => Some(record_0281()),
        282 => Some(record_0282()),
        283 => Some(record_0283()),
        284 => Some(record_0284()),
        285 => Some(record_0285()),
        286 => Some(record_0286()),
        287 => Some(record_0287()),
        288 => Some(record_0288()),
        289 => Some(record_0289()),
        290 => Some(record_0290()),
        291 => Some(record_0291()),
        292 => Some(record_0292()),
        293 => Some(record_0293()),
        294 => Some(record_0294()),
        295 => Some(record_0295()),
        296 => Some(record_0296()),
        297 => Some(record_0297()),
        298 => Some(record_0298()),
        299 => Some(record_0299()),
        300 => Some(record_0300()),
        301 => Some(record_0301()),
        302 => Some(record_0302()),
        303 => Some(record_0303()),
        304 => Some(record_0304()),
        305 => Some(record_0305()),
        306 => Some(record_0306()),
        307 => Some(record_0307()),
        308 => Some(record_0308()),
        309 => Some(record_0309()),
        310 => Some(record_0310()),
        311 => Some(record_0311()),
        312 => Some(record_0312()),
        313 => Some(record_0313()),
        314 => Some(record_0314()),
        315 => Some(record_0315()),
        316 => Some(record_0316()),
        317 => Some(record_0317()),
        318 => Some(record_0318()),
        319 => Some(record_0319()),
        320 => Some(record_0320()),
        321 => Some(record_0321()),
        322 => Some(record_0322()),
        323 => Some(record_0323()),
        324 => Some(record_0324()),
        325 => Some(record_0325()),
        326 => Some(record_0326()),
        327 => Some(record_0327()),
        328 => Some(record_0328()),
        329 => Some(record_0329()),
        330 => Some(record_0330()),
        331 => Some(record_0331()),
        332 => Some(record_0332()),
        333 => Some(record_0333()),
        334 => Some(record_0334()),
        335 => Some(record_0335()),
        336 => Some(record_0336()),
        337 => Some(record_0337()),
        338 => Some(record_0338()),
        339 => Some(record_0339()),
        340 => Some(record_0340()),
        341 => Some(record_0341()),
        342 => Some(record_0342()),
        343 => Some(record_0343()),
        344 => Some(record_0344()),
        345 => Some(record_0345()),
        346 => Some(record_0346()),
        347 => Some(record_0347()),
        348 => Some(record_0348()),
        349 => Some(record_0349()),
        350 => Some(record_0350()),
        351 => Some(record_0351()),
        352 => Some(record_0352()),
        353 => Some(record_0353()),
        354 => Some(record_0354()),
        355 => Some(record_0355()),
        356 => Some(record_0356()),
        357 => Some(record_0357()),
        358 => Some(record_0358()),
        359 => Some(record_0359()),
        360 => Some(record_0360()),
        361 => Some(record_0361()),
        362 => Some(record_0362()),
        363 => Some(record_0363()),
        364 => Some(record_0364()),
        365 => Some(record_0365()),
        366 => Some(record_0366()),
        367 => Some(record_0367()),
        368 => Some(record_0368()),
        369 => Some(record_0369()),
        370 => Some(record_0370()),
        371 => Some(record_0371()),
        372 => Some(record_0372()),
        373 => Some(record_0373()),
        374 => Some(record_0374()),
        375 => Some(record_0375()),
        376 => Some(record_0376()),
        377 => Some(record_0377()),
        378 => Some(record_0378()),
        379 => Some(record_0379()),
        380 => Some(record_0380()),
        381 => Some(record_0381()),
        382 => Some(record_0382()),
        383 => Some(record_0383()),
        384 => Some(record_0384()),
        385 => Some(record_0385()),
        386 => Some(record_0386()),
        387 => Some(record_0387()),
        388 => Some(record_0388()),
        389 => Some(record_0389()),
        390 => Some(record_0390()),
        391 => Some(record_0391()),
        392 => Some(record_0392()),
        393 => Some(record_0393()),
        394 => Some(record_0394()),
        395 => Some(record_0395()),
        396 => Some(record_0396()),
        397 => Some(record_0397()),
        398 => Some(record_0398()),
        399 => Some(record_0399()),
        400 => Some(record_0400()),
        401 => Some(record_0401()),
        402 => Some(record_0402()),
        403 => Some(record_0403()),
        404 => Some(record_0404()),
        405 => Some(record_0405()),
        406 => Some(record_0406()),
        407 => Some(record_0407()),
        408 => Some(record_0408()),
        409 => Some(record_0409()),
        410 => Some(record_0410()),
        411 => Some(record_0411()),
        412 => Some(record_0412()),
        413 => Some(record_0413()),
        414 => Some(record_0414()),
        415 => Some(record_0415()),
        416 => Some(record_0416()),
        417 => Some(record_0417()),
        418 => Some(record_0418()),
        419 => Some(record_0419()),
        420 => Some(record_0420()),
        _ => None,
    }
}

pub fn score_catalog(salt: u64, bridge_id: u32) -> u64 {
    let mut records = Vec::new();
    for id in 1..=420_u16 {
        if let Some(record) = lookup_record(id) {
            if ((record.flags as u64) ^ salt ^ bridge_id as u64) & 3 != 0 {
                records.push(record);
            }
        }
    }
    let mut score = salt ^ ((bridge_id as u64) << 21);
    for record in &records {
        score ^= checksum::mix_u64(
            record.id as u64 ^ ((record.max_load_tons as u64) << 19) ^ record.flags as u64,
        );
    }
    score
        ^ fastpath::fast_catalog_probe(&mut records, score, |record| {
            record.id as u64
                ^ ((record.min_clearance_cm as u64) << 17)
                ^ ((record.max_load_tons as u64) << 33)
                ^ record.flags as u64
        })
}
