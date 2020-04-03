fn main() {
    // If/Else

    let n = 3;

    if n < 10 {
        println!("{} is less than 10", n);
    } else if n == 10 {
        println!("{} is equal to 10", n);
    } else {
        println!("{} is greater than to 10", n);
    }

    let c = false;
    let n = if c { 50 } else { 70 };

    // Match

    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("something else"),
    }

    let n = 15;

    match n {
        1 => println!("one"),
        2 | 3 | 5 | 7 | 11 => println!("this is a prime"),
        13...19 => println!("a teen"),
        _ => println!("ain't special"),
    }

    let pair1 = (0, -2);

    match pair1 {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("no match"),
    }

    let pair2 = (5, -5);

    match pair2 {
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y => println!("equal zero"),
        (x, _) if x % 2 == 0 => println!("x is even"),
        _ => println!("no match"),
    }

    let p = 5;

    match p {
        n @ 1...12 => println!("n: {}", n),
        n @ 13...19 => println!("n: {}", n),
        _ => println!("no match"),
    }

    let n = match p {
        n @ 1...12 => n,
        n @ 13...19 => n,
        _ => 0,
    };

    println!("n: {}", n);
}
