use std::mem;

fn main() {
    // Bool

    let t = true;
    let f = false;

    // Char
    // A 4 byte character.

    let c = 'c';
    let c: char = 'c';

    // Integer
    // i8, u8, i16, u16, i32, u32, i64, u64, isize, usize

    let n = 5;
    let m = 2147483647;

    // Float
    // f32, f64

    let pi = 3.14;
    let e = 2.718;

    // Array
    // let name: [type; size] = [elem1, elem2, elem3, elem4];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a1 = a[0];
    println!(
        "Element [0]: {},  Len: {} Mem: {}",
        a1,
        a.len(),
        mem::size_of_val(&a)
    );

    // Slice

    let a = [1, 2, 3, 4, 5];
    let s = &a[2..4];
    println!("{:?}{:?}", a, s);

    // String

    let str1 = "Hello, World!"; // &str
    let str2 = "Hello, World!".to_string(); // String
    let str3 = String::from("Hello, World!"); // String

    let slice_str3 = &str3[0..5];

    println!("Print slice string: {}", slice_str3);

    let h = String::from("Hello, ");
    let w = String::from("World!");
    let concat_hw = h + &w;

    println!("Concatenate string: {}", concat_hw);

    // Tuple

    let t = (42, 3.27, 'x');
    let t: (i32, f64, char) = (42, 3.27, 'x');

    let (_, _, z) = t;
    let f = (2, t);

    println!("Print character: {}", t.2);
    println!("{:?}", f.1);
    println!("{:#?}", f.1);
}
