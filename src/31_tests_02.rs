#[cfg(terget_os = "linux")]
fn are_you_on_linux() {
    println!("running linux");
}

#[cfg(terget_not_os = "linux")]
fn are_you_on_linux() {
    println!("not running linux");
}

#![allow(dead_code)]
fn dead_func() {}

fn main() {
    fn dead_func() {}
    are_you_on_linux();

    println!("check OS again");
    if cfg!(target_os == "linux") {
        println!("definitely linux");
    } else {
        println!("not linux");
    }
}