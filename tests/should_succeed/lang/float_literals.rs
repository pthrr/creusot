extern crate creusot_std;
use creusot_std::prelude::*;

// Each literal below used to reach Why3 wrong. The proofs go through only if
// Why3 is handed exactly the value Rust computes.

const ONE: f64 = 1.0;
const ONE_AND_A_SIXTEENTH: f64 = 1.0625;
const TEN: f64 = 10.0;
const BIG: f64 = 1.0e30;

// The fraction of 1.03125 starts with a zero nibble. A 12-digit hex field
// dropped it, and Why3 read 1.5, which this bound excludes.
#[ensures(ONE < result && result < ONE_AND_A_SIXTEENTH)]
pub fn leading_zero_nibble() -> f64 {
    1.03125
}

// An integral value above 2^53. Its shortest decimal is not its exact value,
// and Why3 rejected the whole file.
#[ensures(TEN < result)]
pub fn beyond_two_to_the_53() -> f64 {
    BIG
}

// Negative zero was printed as `--0.0`, which does not parse.
pub fn negative_zero() -> f64 {
    -0.0
}
