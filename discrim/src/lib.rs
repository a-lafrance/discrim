pub use discrim_codegen::*;

// TODO: better error handling, specifically maybe a more robust type than just storing the discriminant
// Should never be manually implemented
pub trait FromDiscriminant<T>: Sized {
    fn from_discriminant(tag: T) -> Result<Self, T>;
}
