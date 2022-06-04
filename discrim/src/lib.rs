//! Automatically initialize enum variants by discriminant.
//!
//! This crate provides the [`FromDiscriminant`] trait and associated [derive macro](discrim_codegen::FromDiscriminant)
//! for automatically generating code to initialize enum values from their discriminant.
//!
//! Basically, for fieldless, non-generic enums with a `#[repr(...)]` specified, the derive macro generates
//! the giant match block required to initialize the enum from a discriminant value as an implementation of [`FromDiscriminant`].
//!
//! # Usage
//! Given an enum that fits these constraints:
//! ```rust
//! use discrim::FromDiscriminant;
//!
//! #[derive(Debug, FromDiscriminant, PartialEq)]
//! #[repr(u8)]
//! enum Opcode {
//!     Add, Sub, Mul, Div
//! }
//!
//! assert_eq!(Opcode::from_discriminant(2), Ok(Opcode::Mul));
//! assert_eq!(Opcode::from_discriminant(5), Err(5));
//! ```

pub use discrim_codegen::FromDiscriminant;

// TODO: better error handling, specifically maybe a more robust type than just storing the discriminant
// Should never be manually implemented

/// Initialize enum values from their discriminant.
///
/// You should never implement this trait yourself; use the [derive macro](discrim_codegen::FromDiscriminant) instead.
/// See the docs for either the crate root or the derive macro for usage examples of this trait.
pub trait FromDiscriminant<T>: Sized {
    fn from_discriminant(discrim: T) -> Result<Self, T>;
}
