#[derive(Debug)]
enum Example {
    Float(f64),
    Int(i32),
    Text(String),
}

fn main() {
    let r = vec![
        Example::Int(142),
        Example::Float(12.34),
        Example::Text(String::from("string")),
    ];

    println!("{:?}", &r);
}
