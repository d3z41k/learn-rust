fn is_even(n: u32) -> bool {
    n % 2 == 0
}

fn main() {
    let top = 10000;
    let mut c = 0;

    for n in 0.. {
        let x = n * n;

        if x >= top {
            break;
        } else if is_even(x) {
            c += x;
        }
    }
    println!("{}", c);

    let s: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < 10000)
        .filter(|&n| is_even(n))
        .fold(0, |s, i| s + i);

    println!("{}", s);
}