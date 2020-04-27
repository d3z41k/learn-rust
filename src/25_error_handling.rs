use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

fn exit1(x: i32) {
    if x == 0 {
        panic!("We got a 0");
    }
    println!("Things are fine!");
}

fn exit2(x: Option<i32>) {
    match x {
        Some(0) => panic!("We got a 0"),
        Some(x) => println!("We got a {} things are fine!", x),
        None => println!("We got nothing"),
    }
}

fn read_file1() -> Result<String, io::Error> {
    let f = File::open("text.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file2() -> Result<String, io::Error> {
    let mut f = File::open("text.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("text.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    exit1(1);
//    exit1(0);

    exit2(Some(1));
    exit2(Some(10));
    exit2(None);
//    exit2(Some(0));

    let f = File::open("text.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("text.txt") {
                Ok(fc) => fc,
                Err(e) => { panic!("could't create file: {:?}", e) },
            }
        },
        Err(error) => {
            panic!("could't open the file: {:?}", error);
        },
    };

}