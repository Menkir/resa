use std::ops::Mul;
use std::ops::Add;

struct Vec2<T>
{
    x: T,
    y: T,
}

impl<T: Mul + Add> Vec2<T> {
    fn magnitude_squared(&self) -> T {
        self.x * self.x + self.y * self.y // error here
    }
}

fn main() {
    let x = Vec2 { x: 1f32, y: 1f32 };
    println!("{}", x.magnitude_squared());
}
