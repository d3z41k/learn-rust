use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// Relations
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width,
            height,
            // width: width,
            // height: height,
        }
    }
}

// Methods
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self) {
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn area(obj: &Object) -> u32 {
    obj.width * obj.height
}

fn main() {
    let o = Object {
        width: 35,
        height: 55,
    };

    println!("{}x{} with area: {}", o.width, o.height, area(&o));

    let obj = Object::new(57, 83);

    obj.show();

    println!("{:#?}", obj);
    println!("{}", obj);
}
