mod hasher;

use hasher::hash_from;
pub fn hash_finder(args: (usize, usize, bool, String)) -> Vec<(i32, String)> {
    let mut result: Vec<(i32, String)> = Vec::new();
    let (num_zeros, iter_num, switch, message): (usize, usize, bool, String) = args;
    match switch {
        false => {
            result.push((0, hash_from(message.as_str())));
        }
        true => {
            let mut val: i32 = 0;
            let zeros: String = "0".repeat(num_zeros);
            while result.len() != iter_num {
                let tmp: String = hash_from(val.to_string().as_str());
                if tmp.ends_with(zeros.as_str()) {
                    result.push((val, tmp));
                }
                val += 1;
            }
        }
    }
    return result;
}
