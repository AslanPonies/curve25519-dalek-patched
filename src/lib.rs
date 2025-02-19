#![no_std]

// A simple test function (your original stub)
pub fn curve_test_function() -> u32 {
    42
}

/// Stub implementation of the `digest` module as expected by ed25519-dalek.
pub mod digest {
    /// A dummy Digest trait.
    pub trait Digest {}

    /// Stub for generic_array and its typenum.
    pub mod generic_array {
        pub mod typenum {
            /// A dummy type representing U64.
            #[derive(Debug, Clone, Copy)]
            pub struct U64;
        }
    }
}

/// Stub module for constants.
pub mod constants {
    // If ed25519-dalek expects any constants here, you can add them.
    // For now we leave it empty.
}

/// Stub module for Edwards curve types.
pub mod edwards {
    /// A dummy compressed Edwards curve point.
    #[derive(Debug, Clone, Copy)]
    pub struct CompressedEdwardsY;

    /// A dummy Edwards curve point.
    #[derive(Debug, Clone, Copy)]
    pub struct EdwardsPoint;
}

/// Stub module for scalar arithmetic.
pub mod scalar {
    /// A dummy scalar type.
    #[derive(Debug, Clone, Copy)]
    pub struct Scalar;
}

// Re-export items so that ed25519-dalek finds them in the expected locations.
pub use digest::Digest;
pub use digest::generic_array::typenum::U64;
pub use constants::*;
pub use edwards::{CompressedEdwardsY, EdwardsPoint};
pub use scalar::Scalar;
