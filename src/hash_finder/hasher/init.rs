use std::slice::{Iter, IterMut};

pub fn generate_blocks(message: &str) -> Vec<[u32; 64]> {
    let mut message_len: u64 = message.len() as u64;
    let mut field_len: u8 = 8u8;
    let bits_len: u64 = message_len << 3;

    let message: &[u8] = message.as_bytes();

    let payload: u64 = message_len + field_len as u64;
    let mut padding: u64 = 64u64 - payload % 64;
    let block_count: u64 = (payload >> 6) + 1u64;
    let mut blocks: Vec<[u8; 64]> = Vec::new();

    blocks.resize(block_count as usize, [0x00; 64]);

    let mut cur_message_pos: usize = 0;

    for block in blocks.iter_mut() {
        let mut iter: IterMut<u8> = block.iter_mut();
        while let Some(byte) = iter.next() {
            match message_len {
                0 => {
                    if padding != 0 {
                        padding -= 1;
                    } else {
                        field_len -= 1;
                        *byte = (bits_len >> (field_len * 8)) as u8;
                    }
                }
                _ => {
                    *byte = message[cur_message_pos];
                    cur_message_pos += 1;
                    message_len -= 1;

                    if message_len == 0 {
                        let byte: &mut u8 = iter.next().unwrap();
                        *byte = 128;
                        padding -= 1;
                    }
                }
            }
        }
    }
    return refactor_to64(blocks);
}

fn refactor_to64(blocks: Vec<[u8; 64]>) -> Vec<[u32; 64]> {
    let mut blocks32: Vec<[u32; 64]> = Vec::new();
    let mut iter_block8 = blocks.iter();

    blocks32.resize(blocks.len(), [0x00; 64]);

    for block32 in blocks32.iter_mut() {
        let block8 = iter_block8.next().unwrap();
        let mut iter_byte8: Iter<u8> = block8.iter();
        for i in 0..16 {
            for _ in 0..4 {
                let byte8 = iter_byte8.next().unwrap();
                block32[i] = block32[i] << 8;
                block32[i] += *byte8 as u32;
            }
        }
        for i in 16..64 {
            block32[i] = block32[i - 16]
                .wrapping_add(s0(block32[i - 15]))
                .wrapping_add(block32[i - 7])
                .wrapping_add(s1(block32[i - 2]));
        }
    }

    return blocks32;
}

fn right_rotate_32(n: u32, d: u32) -> u32 {
    return (n >> d) | (n << (32 - d));
}

fn s0(byte: u32) -> u32 {
    return right_rotate_32(byte, 7u32) ^ right_rotate_32(byte, 18u32) ^ (byte >> 3);
}

fn s1(byte: u32) -> u32 {
    return right_rotate_32(byte, 17u32) ^ right_rotate_32(byte, 19u32) ^ (byte >> 10);
}
