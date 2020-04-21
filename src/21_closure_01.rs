fn run<F>(f: F)
    where F: Fn() {
    f();
}

fn add3<F>(f: F) -> i32
    where F: Fn(i32) -> i32 {
    f(3)
}

struct A<F: Fn(i32) -> i32> {
    f: F
}

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<self::Item>;
}

fn main() {
    let p = || println!("hello from run function!");
    run(p);

    let x = |i| i * 10;

    println!("3 * 10 = {}", add3(x));

    let a = A {
        f: x,
    };

    println!("3 * 10 = {}", add3(a.f));

//    let f = |i: i32| -> i32 i + 1;
    let f = |i| i + 1;
    let x = 10;
    println!("{}", f(x));

    let p = || println!("this is a closure");
    p();

    let mut c = 0;

    let mut inc = || {
        c += 1;
        println!("incremented by 1: {}", c);
    };

    inc();
    inc();
    inc();

    let v = [1, 2, 3];

    println!("v {}", v.iter().any(|&x| x != 2));

    for i in v.iter() {
        println!("{}", i)
    }
}