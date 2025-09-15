mod globals;
mod math;

use globals::*;
use math::*;

fn main() {
    // println!("Hello, world! {} - {}", UP_MIN_INDEX, UP_MAX_INDEX);
    shift(TOTAL_LETTERS, UP_MIN_INDEX, UP_MAX_INDEX, 'C', -50);
}
