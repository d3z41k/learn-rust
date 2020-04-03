fn main() {
    // Loop

    let mut x = 0;

    loop {
        x += 1;

        if x >= 30 {
            println!("x is now greater than or equal to 30 so we break the loop");
            break;
        }

        if x % 3 != 0 {
            continue; // The print statment following this if statement will not be printed
        }

        println!("{} is divisible by 3", x);
    }

    // While

    let mut y = 0;

    while y <= 30 {
        y += 1;

        if y % 3 == 0 {
            println!("{} is divisible by 3", y);
        }
    }

    // For-In

    for z in 1..31 {
        if z % 3 == 0 {
            println!("{} is divisible by 3", z);
        }
    }

    let v = vec![10, 20, 30, 40, 50];
    for i in v {
        println!("i: {}", i);
    }
}
