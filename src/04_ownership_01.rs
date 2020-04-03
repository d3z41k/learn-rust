fn take(v: Vec<i32>) {
    println!("we took v: {}", v[10] + v[100]);
}

fn cop(a: i32, b: i32) {
    println!("{}", a + b);
}

fn main() {
    let x = 1; // store in stack
    let y = x;

    {
        let z = 5; // "z" exist only in this scope
    }

    // x + z;

    let s = String::from("String!"); // store in heap
    let s1 = s;

    // println!("{}", s); // value borrowed here after move

    let s = String::from("String!");
    let s2 = &s;

    println!("{}", s);

    // Complex ownership (heap)

    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    take(v); // move it to function
             // println!("{}", v[0]);
    println!("Finished!");

    // Simple ownership (stack)

    let a = 24;
    let b = 15;

    cop(a, b);

    println!("We have a: {} and b: {}", a, b);
}
