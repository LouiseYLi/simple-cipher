mod cipher;
mod globals;
mod math;

use cipher::*;
use globals::*;

fn main() {
    let msg: &str = "If he had anything confidential to say, he wrote it in cipher, that is, by so changing the order of the letters of the alphabet, that not a word could be made out.";
    println!("{} - {}", UP_MIN_INDEX, UP_MAX_INDEX);
    encrypt(msg, -529);
}
