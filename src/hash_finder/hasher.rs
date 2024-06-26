mod comp;
mod init;

pub fn hash_from(message: &str) -> String {
    let mut blocks: Vec<[u32; 64]> = init::generate_blocks(message);

    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    for block in blocks.iter_mut() {
        comp::compress(block, &mut h);
    }

    let mut res: String = String::new();
    for h in h {
        res += format!("{h:x}").as_str();
    }
    return res;
}
