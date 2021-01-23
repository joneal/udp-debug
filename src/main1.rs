// use std::mem;
// use std::env;

// fn main() {

//     let dir = env::current_dir().unwrap();

//     let mut x: u32 = 5;
//     x = 6;

//     // character
//     let c: char = 'z';

//     // tuple
//     let t:(i32, f64, char) = (42, 6.12, 'j');
//     let (z, _, x) = t;
//     let m = t.0;

//     // arrays
//     let a = [1,2,3,4,5,6,7,8];
//     let a1 = a[0];

//     // tuple in a tuple
//     let t = (1,'a',false);
//     let t2 = (2, t);
//     println!("{} {} {} ", t.0, t.1, t.2);
//     println!("{:?}", t2);  // ? is a debug flag
//     println!("{:#?}", t2);  // ? is a debug flag, with pretty print

//     // arrays
//     let xs: [i32; 5] = [4,5,6,7,8];
//     println!("{} {} {} ", xs[0], xs.len(), mem::size_of_val(&xs));

//     let ys = &xs[2..4];   // slice
//     println!("{} {}", ys[0], ys[1]);

//     let s = "String";   // slice of a string.  strings are not literal types, but compound types
//     let ss = String::from("String");
//     let sss = "String".to_string();
//     let slice = &ss[0..4];
    
//     let t = ();  // empty tuple, unit value.  any function that does not return anything returns a tuple

// }
