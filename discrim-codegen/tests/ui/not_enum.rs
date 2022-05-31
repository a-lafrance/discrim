use discrim::FromDiscriminant;

#[derive(FromDiscriminant)]
struct NotEnum {
    i: i32,
}

fn main() {
    println!("not enum");
}
