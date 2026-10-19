extern crate creusot_std;
use creusot_std::prelude::*;

// Bitwise-mode integer modules have no `of_int`, so a float-to-integer cast
// has no target to build there: it must be rejected, not lowered.
#[bitwise_proof]
pub fn float_to_byte(x: f64) -> u8 {
    x as u8
}
