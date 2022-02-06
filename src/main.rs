use vecs::*;
fn main() {
    let mut v = Vec2::new(5, -10);

    let v2 = Vec2::new(10, 30);

    v += v2;

    println!("{}", v);
}