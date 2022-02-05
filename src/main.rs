use vecs::Vec2;
fn main() {
    let v = Vec2::new(5, 10);

    let v2 = Vec2::new(10, 30);

    let v3 = v.dot(v2);

    println!("{:?}", v3);
}