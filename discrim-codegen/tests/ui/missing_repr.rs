use discrim::FromDiscriminant;

#[derive(FromDiscriminant)]
enum MissingRepr {
    Bad,
}

fn main() {
    println!("missing repr");
}
