fn pr1<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() == y.len() {
        x
    } else {
        y
    }
}

fn pr2<'a>() -> &'a str {
    let s = "Hello";
    s

}

struct D<'a, 'b> {
    x: &'a str,
    y: &'b str,
}

fn main() {
    let a = "a string";
    let b = "b string";

    let c = pr1(a, b);

    println!("{}", c);
    println!("{}", pr2());

    let d = D{x: "Hello", y: "There"};

    let s: &'static str = "THe String"; // very slow

}