# discrim
Construct enum variants from their discriminant

## Overview

In order to initialize certain enum values from their discriminant, you basically have to manually write a giant match block that maps from each discriminant to the corresponding variant. This is both an annoyingly manual process and a potentially unsafe one; for example, what if you forget to update a discriminant value in the match after changing it in the enum definition?

This crate addresses that by providing a way to automatically generate that initialization code. More specifically, for fieldless, non-generic enums with a `#[repr(...)]` specified, it provides the `FromDiscriminant` trait and corresponding derive macro that automatically generates the initialization code as an implementation of the trait.

This project is licensed under the MIT License.

## Example

A simple example, based on the use case that inspired this crate to begin with:
```rust
use discrim::FromDiscriminant;

#[derive(Debug, FromDiscriminant)]
#[repr(u8)]
enum Opcode {
    Add, Sub, Mul, Div,
}

fn main() {
    // prints "Ok(Mul)"
    println!("{:?}", Opcode::from_discriminant(2));

    // prints "Err(5)"
    println!("{:?}", Opcode::from_discriminant(5));
}
```
