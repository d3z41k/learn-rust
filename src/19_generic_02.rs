use std::ops::Mul;

trait Shape <T> {
    fn area(&self) -> T;
}

struct  Rectangle<T: Mul> {
    x: T,
    y: T,
}

struct  Circle<T: Mul> {
    radius: T,
}

impl <T:Mul<Output = T> + Copy> Shape<T> for Circle<T> {
    fn area (&self) -> T {
        // 3.141 * (self.radius * self.radius) // need macros
    }
}

// impl <T: Copy> Shape<T> for Rectangle<T>
//     where T: Mul<Output = T>, {
//     fn area (&self) -> T {
//         self.x * self.y
//     }
// }

impl <T:Mul<Output = T> + Copy> Shape<T> for Rectangle<T> {
    fn area (&self) -> T {
        self.x * self.y
    }
}



fn main() {
    let r = Rectangle {x: 10, y: 20};
    let r2 = Rectangle {x: 10.12, y: 20.72};

    println!("{} {}", r.area(), r2.area())

}