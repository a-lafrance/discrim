use discrim::FromDiscriminant;

#[derive(Debug, FromDiscriminant, PartialEq)]
#[repr(u8)]
enum Opcode {
    Add, Sub, Mul, Div,
}

#[test]
fn sanity_check() {
    assert_eq!(Opcode::from_discriminant(2), Ok(Opcode::Mul));
}
