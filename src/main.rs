use vecs::*;
fn main() {
    let v = Vec2::new(10., 1.);
    let v1 = Vec2::new(10., 1.);

    let v2 = v.dot(v1);

    println!("{}", v2);
}