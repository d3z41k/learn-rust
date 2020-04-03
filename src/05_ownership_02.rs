fn re(v: Vec<32>) -> Vec<i32> {
    println!("{}", v[120] + v[111]);
    v
}

fn borrow1(v: &Vec<i32>) {
    println!("{}", (*v)[10] + (*v)[12]);
}

fn borrow2(v: &Vec<i32>) {
    println!("{}", v[10] + v[12]); // more idiomatic
                                   //    println!("{}", &v[10] + &v[12]);
}

fn count(v: &Vec<i32>, val: i32) -> usize {
    v.into_iter().filter(|&&x| x == val).count()
}

fn main() {
    let mut v = Vec::new();

    for i in 1..1000 {
        v.push(i);
    }

    v = re(v);
    println!("Still own v: {} {}" v[0], v[1]);

    borrow1(&v);
    println!("Still own v: {} {}" v[0], v[1]);

    borrow2(&v);
    println!("Still own v: {} {}" v[0], v[1]);

    //-------------------------------------------

    let v = vec![4, 5, 3, 6, 7, 4, 8, 6, 4, 2, 4, 2, 5, 3, 7, 7];
    for &i in &v {
        let r = count(&v, i);
        println!("{} is repeated {} times", i, r);
    }
}
