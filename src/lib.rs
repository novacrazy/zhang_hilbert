//! This crate provides a function that produces an arbitrary-sized
//! pseudo-Hilbert scan based on “A Pseudo-Hilbert Scan for Arbitrarily-Sized
//! Arrays” by Zhang, et al.
//!
//! # Differences from the original algorithm
//!
//! ## The `division` function
//!
//! TODO
//!
//! ## The last `E_B(E, O)` block
//!
//! This implementation uses a different curve-type selection rule for the
//! last `E_B(E, O)` block in a `E_R(E, O)` rectangle. This makes the leaving
//! point fixed at a known point in more cases, making the output suitable for
//! tiling.
//!
//! ```text
//! cargo run --example hilbertgen -- 6 7
//!   ,---, ,---,        ,---, ,---,
//!   '-, '-' ,-'        '-, '-' ,-'
//!   ,-' ,-, '-,        ,-' ,-, '-,
//!   '-, | '---'        '-, | '---'
//!   ,-' '-, ,--        ,-' '-----,
//!   '-, ,-' '-,        '-, ,-----'
//!   --' '-----'        --' '------
//!    Original      This implementation
//!
//! cargo run --example hilbertgen -- 4 3
//!     ,------           ,-----,
//!     '-----,           '-, ,-'
//!     ------'           --' '--
//!    Original      This implementation
//! ```
//!
//! ## Aspect-ratio bounded tiling
//!
//! The algorithm accepts any rectangle size, but the output quality
//! deteriorates as the proportions of the rectangle gets distant from square.
//! `ArbHilbertScanCore` improves it by dividing the rectangle into multiple
//! rectangles whose proportions are closer to square than the original
//! rectangle is (thus *aspect-ratio bounded*).
//!
//! ```
//! $ cargo run --example hilbertgen -- 40 7
//! ,---, ,---, ,---, ,---, ,---, ,-, ,---, ,---, ,---, ,---, ,-, ,---, ,---, ,---,
//! '-, '-' ,-' '-, '-' ,-' '-, '-' '-' ,-' '-, '-' ,-' '-, '-' '-' ,-' '-, '-' ,-'
//! ,-' ,-, '-, ,-' ,-, '-, ,-' ,-, ,-, '-, ,-' ,-, '-, ,-' ,-, ,-, '-, ,-' ,-, '-,
//! '-, | '---' '-, | '---' '---' | | '---' '-, | '---' '---' | | '---' '-, | '---'
//! ,-' '-----, ,-' '-----, ,-----' '-----, ,-' '-----, ,-----' '-----, ,-' '-----,
//! '-, ,-----' '-, ,-----' '-----, ,-----' '-, ,-----' '-----, ,-----' '-, ,-----'
//! --' '---------' '-------------' '---------' '-------------' '---------' '------
//!
//! $ cargo run --example hilbertgen -- -a zhang 40 7
//! ,-----------------------, ,-, ,-, ,-, ,-, ,-, ,-, ,-, ,-, ,-, ,---------------,
//! '---------------------, '-' '-' '-' '-' '-' '-' '-' '-' '-' '-' ,-------------'
//! ,---------------------' ,-, ,-, ,-, ,-, ,-, ,-, ,-, ,-, ,-, ,-, '-------------,
//! '-----------------------' '-' '-' '-' '-' '-' | | '-' '-' '-' '---------------'
//! ,---------------------------------------------' '-----------------------------,
//! '---------------------------------------------, ,-----------------------------'
//! ----------------------------------------------' '------------------------------
//! ```
//!
mod core;
mod arb;

pub use self::{core::*, arb::*};

pub type HilbertScan32 = HilbertScanCore<u32, [LevelState<u32>; 32]>;

pub type ArbHilbertScan32 = ArbHilbertScanCore<u32, [LevelState<u32>; 32]>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
