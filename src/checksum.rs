pub fn fnv1a32(data: &[u8]) -> u32 {
    let mut hash = 0x811c_9dc5_u32;
    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    hash
}

pub fn rolling64(data: &[u8]) -> u64 {
    let mut a = 0x9e37_79b9_7f4a_7c15_u64;
    let mut b = 0xc2b2_ae3d_27d4_eb4f_u64;
    for (idx, byte) in data.iter().enumerate() {
        a ^= (*byte as u64).wrapping_add((idx as u64) << (idx & 7));
        a = a.rotate_left(9).wrapping_mul(0x1000_0000_01b3);
        b ^= a.rotate_right((idx & 31) as u32);
        b = b.wrapping_add(0x1656_67b1_9e37_79f9);
    }
    a ^ b.rotate_left(17)
}

pub fn mix_u64(mut value: u64) -> u64 {
    value ^= value >> 33;
    value = value.wrapping_mul(0xff51_afd7_ed55_8ccd);
    value ^= value >> 33;
    value = value.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    value ^ (value >> 33)
}
