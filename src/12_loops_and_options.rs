fn main() {
    let mut s = Some(0);

    loop {
        match s {
            Some(i) => {
                if i > 19 {
                    println!("Quit");
                    s = None;
                } else {
                    println!("{}", i);
                    s = Some(i + 2);
                }
            }
            _ => {
                break;
            }
        }
    }

    while let Some(i) = s {
        if i > 19 {
            println!("Quit");
            s = None;
        } else {
            println!("{}", i);
            s = Some(i + 2);
        }
    }
}
