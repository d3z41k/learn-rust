use std::fs::File;

fn division(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}

fn main() {
    let res = division(5.0, 7.0);

    match res {
        Some(x) => println!("{:.7}", x),
        None => println!("can't divide by o"),
    }

    // Open file

    let f = File::open("test.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}
