[![Crate](https://img.shields.io/badge/crates.io-v0.2.2-orange.svg)](https://crates.io/crates/vecs)
[![API](https://img.shields.io/badge/docs-passing-green.svg)](https://docs.rs/vecs/latest/vecs/)

# a simple vector math library

## examples

```
use vecs::Vec2;

fn main() {
    // creates 2 Vec2 objects
    let v1 = Vec2::new(12., 6.);
    let v2 = Vec2::new(17., 9.);

    // adds the vectors together
    let v3 = v1 + v2;

    // prints (29, 15) to the console
    println!("{}", v3);
}
```

```
use vecs::Vec3;

fn main() {
    // creates 2 Vec3 objects
    let v1 = Vec3::new(2., 6., 7.);
    let v2 = Vec3::new(5., 3., 8.);

    // gets the 2 vectors cross product
    let v3 = v1.cross(v2);

    // prints (27, 19, -24) to the console
    println!("{}", v3);
}
```
