fn create() -> Box<Fn()> {
    Box::new(move || println!("this is a closure in a box!"))
}

fn main() {
    let b = Box::new(10); // use heap
    println!("{}", b);

    let n = 4;
    let m = &n;
    let z = Box::new(n);

    if *m == *z {
        println!("true");
    }

    let x = create();
    x();
}