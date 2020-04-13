use std::fmt;

struct Square<T> {
    x: T,
}

fn p<T: fmt::Debug>(x: T) {
    println!("{:?}", x);
}

struct A<T> {
    x: T,
}

impl <T> A<T> {
    fn item(&self) -> &T {
        &self.x
    }
}

struct C<U, V> {
    x: U,
    y: V,
}

struct D<V> {
    x: V,
    y: V,
}
fn main() {
    let s = Square{x: 10};
    let s = Square{x: 1.0};
    let s = Square{x: "Hello"};
    let s = Square{x: 'c'};

    p(10);
    p(String::from("String!"));

    let a = A{x: "Hello"};

    p(a.item());
}
