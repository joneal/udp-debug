use std::fmt;

fn take(v: Vec<i32>) {
    println!("We took v: {}", v[10] + v[100]);
}

// Methods
impl Object {
    fn new(width: u32, height: u32) -> Object {
        Object {
            width: width,
            height:height
        }
    }
}

// Related functions
impl Object {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn show(&self){
        println!("{}x{} with area: {}", self.width, self.height, self.area());
    }
}

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32
}

fn main() {

    let o = Object {
        width: 35,
        height : 55
    };

    let obj = Object::new(11, 22);

    println!("{}x{} with area: {}", o.width, o.height, o.area());
    obj.show();

    println!("{:?}", o);
    println!("{:#?}", obj);


    let mut v  = Vec::new();

    for i in 1..1000{
        v.push(i);
    }

    take(v);

    //println!("{}", v[0]);
    println!("Finished");
}