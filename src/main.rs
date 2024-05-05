mod hash_finder;
mod input;

use crate::hash_finder::hash_finder;
use crate::input::get_input;

fn main() {
    let input: (usize, usize, bool, String) = get_input();
    let output: Vec<(i32, String)> = hash_finder(input);

    for (num, hash) in output {
        println!("{}, \"{}\"", num, hash);
    }

    return;
}
