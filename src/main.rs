use vecs::Vec2;
fn main() {
    let v = Vec2::new(1., 1.);

    let v2 = 5. * v;

    let v3 = v2 / v;

    println!("{}", v3);
}