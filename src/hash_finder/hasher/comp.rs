pub fn compress(block: &mut [u32; 64], val: &mut [u32; 8]) {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    let mut a: u32 = val[0];
    let mut b: u32 = val[1];
    let mut c: u32 = val[2];
    let mut d: u32 = val[3];
    let mut e: u32 = val[4];
    let mut f: u32 = val[5];
    let mut g: u32 = val[6];
    let mut h: u32 = val[7];

    for i in 0..64 {
        let tmp1: u32 = h
            .wrapping_add(sum1(e))
            .wrapping_add(choice(e, f, g))
            .wrapping_add(K[i])
            .wrapping_add(block[i]);
        let tmp2: u32 = sum0(a).wrapping_add(majority(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(tmp1);
        d = c;
        c = b;
        b = a;
        a = tmp1.wrapping_add(tmp2);
    }

    val[0] = val[0].wrapping_add(a);
    val[1] = val[1].wrapping_add(b);
    val[2] = val[2].wrapping_add(c);
    val[3] = val[3].wrapping_add(d);
    val[4] = val[4].wrapping_add(e);
    val[5] = val[5].wrapping_add(f);
    val[6] = val[6].wrapping_add(g);
    val[7] = val[7].wrapping_add(h);
}

fn sum0(a: u32) -> u32 {
    return right_rotate_32(a, 2) ^ right_rotate_32(a, 13) ^ right_rotate_32(a, 22);
}

fn sum1(e: u32) -> u32 {
    return right_rotate_32(e, 6) ^ right_rotate_32(e, 11) ^ right_rotate_32(e, 25);
}

fn choice(e: u32, f: u32, g: u32) -> u32 {
    return (e & f) ^ ((!e) & g);
}

fn majority(a: u32, b: u32, c: u32) -> u32 {
    return (a & b) ^ (a & c) ^ (b & c);
}

fn right_rotate_32(n: u32, d: u32) -> u32 {
    return (n >> d) | (n << (32 - d));
}
