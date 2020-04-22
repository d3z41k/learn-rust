use std::collections::HashMap;

macro_rules! a_macro {
    () => {
        println!("This is a macro");
    }
}

macro_rules! x_and_y {
    (x => $e:expr) => { println!("X: {}", $e) };
    (y => $e:expr) => { println!("Y: {}", $e) };
}

macro_rules! build_fn {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    };
}

macro_rules! print_ex {
    ($e:expr) => {
        println!("{:?} = {:?}",  stringify!($e), $e);
    };
}

macro_rules! exeme {
    ($l:expr; and $r:expr) => {
        println!("{:?} and {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l && $r
        )
    };

    ($l:expr; or $r:expr) => {
        println!("{:?} or {:?} is {:?}",
            stringify!($l),
            stringify!($r),
            $l || $r
        )
    };
}

macro_rules! compr {
    ($id1: ident | $id2: ident <- [$start: expr ; $end: expr], $cond: expr) => {
        {
            let mut vec = Vec::new();

            for num in $start..$end + 1 {
                if $cond(num) {
                    vec.push(num);
                }
            }

            vec
        }
    };
}

fn even(x: i32) -> bool {
    x % 2 == 0
}

fn odd(x: i32) -> bool {
    x % 2 != 0
}

macro_rules! new_map {
    ($($key: expr => $val: expr),+) => {
        {
            let mut map = HashMap::new();

            $(
                map.insert($key, $val);
            )*

            map
        }
    };
}

macro_rules! calc {
    (eval $e: expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};

    (eval $e: expr, $(eval $es: expr),+) => {
        {
            calc! {eval $e}
            calc! { $(eval $es),+}
        }
    }
}

fn main() {
    a_macro!();

    x_and_y!(x => 10);
    x_and_y!(y => 20 + 30);

    build_fn!(hi_there);
    hi_there();

    print_ex!({
        let y = 20;
        let z = 30;
        z + y + 10 * 3 * 100
    });

    exeme!(1 == 1; and 2 == 3 - 1);
    exeme!(true; or false);

    let evens = compr![x | x <- [1;10], even];
    println!("{:?}", evens);

    let odds = compr![y | y <- [1;10], odd];
    println!("{:?}", odds);

    let m = new_map!{
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", m);

    calc! {
        eval 4 * 5,
        eval 7 + 3,
        eval (10 * 5) - 20,
        eval 5 / 4
    }
}