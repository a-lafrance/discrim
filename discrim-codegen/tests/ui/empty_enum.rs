use discrim::FromDiscriminant;

#[derive(FromDiscriminant)]
enum Empty { }

fn main() {
    println!("empty");
}
