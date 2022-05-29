# discrim
Construct enum variants from their discriminant

## Overview

This is an initial prototype of a library capable of generating code to construct enum variants from their discriminant. It only works for fieldless `#[repr(...)]` enums (those are the only cases that make sense), and works by deriving an implementation of the custom `FromDiscriminant` trait.

This library is still pretty experimental, and is very lightly tested & sparsely documented, so at the moment use it at your own risk. I've put it on `crates.io` for the daredevils out there and also just to practice publishing a crate. Future versions to come in the near future will be more fit for actual use.

Also, much of the inspiration behind this implementation goes to David Tolnay's `serde_repr` crate, which does something very similar. I'm no codegen expert, so I borrowed pretty heavily from his implementation in terms of high-level concepts, and implemented them myself as best I could.

## Example

A simple example, based on the use case that inspired me to make this crate in the first place:
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
}
```
