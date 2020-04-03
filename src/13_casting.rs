fn main() {
    let f = 24.4321_f32;
    let i = f as u8;
    let c = i as char;

    println!("{} {} {}", f, i, c);

    println!("{} {}", 12 as char, 14 as char);
}
