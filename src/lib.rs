#![no_std]

// We need to provide the modules and types that ed25519-dalek expects.

extern crate generic_array;
extern crate typenum;

// Provide a stub module for constants.
pub mod constants {
    // (You could add real constants here if needed.)
}

// Provide a stub module for digest.
pub mod digest {
    // A dummy Digest trait.
    pub trait Digest {}

    // Re-export a dummy U64 from generic_array::typenum.
    pub mod generic_array {
        pub mod typenum {
            /// A stub type to represent the 64-length type.
            pub struct U64;
        }
    }
}

// Provide a stub module for Edwards curve types.
pub mod edwards {
    /// A dummy type for a compressed Edwards point.
    pub struct CompressedEdwardsY;

    /// A dummy type for an Edwards point.
    pub struct EdwardsPoint;
}

// Provide a stub module for scalars.
pub mod scalar {
    /// A dummy scalar type.
    pub struct Scalar;
}

// You can leave your test function (or remove it if not needed).
pub fn curve_test_function() -> u32 {
    42
}
