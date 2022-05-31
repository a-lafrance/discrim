use discrim::FromDiscriminant;

#[derive(FromDiscriminant)]
#[repr(u8)]
enum Maybe<T> {
    Some(T),
    None
}

fn main() {
    println!("generic");
}
