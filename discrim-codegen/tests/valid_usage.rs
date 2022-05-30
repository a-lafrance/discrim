use discrim::FromDiscriminant;

const FLAG: u8 = 5;

#[derive(Debug, FromDiscriminant, PartialEq)]
#[repr(u8)]
enum TestVariants {
    ImplicitDiscriminant,
    NormalDiscriminant = 3,
    WeirdDiscriminant = if FLAG == 3 { FLAG } else { FLAG - 1 },
}

#[test]
fn implicit_discriminant() {
    assert_eq!(
        TestVariants::from_discriminant(0),
        Ok(TestVariants::ImplicitDiscriminant),
    );
}

#[test]
fn explicit_normal_discriminant() {
    assert_eq!(
        TestVariants::from_discriminant(3),
        Ok(TestVariants::NormalDiscriminant),
    );
}

#[test]
fn explicit_weird_discriminant() {
    assert_eq!(
        TestVariants::from_discriminant(FLAG - 1),
        Ok(TestVariants::WeirdDiscriminant),
    );
}

#[test]
fn nonexistent_discriminant() {
    assert_eq!(
        TestVariants::from_discriminant(1),
        Err(1),
    );
}
