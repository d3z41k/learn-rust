use std::ops;

#[derive(Debug, Clone, Copy)]
struct A(i32);

#[derive(Eq, PartialEq, PartialOrd, Ord)]
// struct B(f32);

struct X;
struct Y;
#[derive(Debug)]
struct XY;
#[derive(Debug)]
struct YX;

impl ops::Add<Y> for X{
    type Output = XY;

    fn add(self, _rhs: Y) -> XY {
        XY
    }
}

impl ops::Add<X> for Y{
    type Output = YX;

    fn add(self, _rhs: X) -> YX {
        YX
    }
}

struct S{
    s: String,
}

impl Drop for S {
    fn drop(&mut self) {
        println!("dropped {}", self.s)
    }
}

fn main() {
    let a = A(32);
    // let b = B(12.13);
    // let c = a.clone();
    let c = a; // derive Copy

    println!("{:?}", a);

    println!("{:?}", X + Y);
    println!("{:?}", Y + X);

    let d = S{s: String::from("D")};
    {
        let e = S{s: String::from("E")};

        {
            let f = S{s: String::from("F")};
            println!("leaving inner scope 2");
        }
        println!("leaving inner scope 1");
    }

    drop(d);
    println!("program ending");
}
