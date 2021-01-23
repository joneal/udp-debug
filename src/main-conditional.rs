// fn main() {
//     // Bindings
//     let c = true;

//     let n = if c { 50 } else { 76 };

//     println!("n: {}", n);

//     let mut i: u32 = 0;
//     loop {
//         println!("finite");
//         i += 1;
//         if i > 10 {
//             break;
//         }
//     }

//     'a: loop {
//         println!("loop a");
//         'b: loop {
//             println!("loop b");
//             'c: loop {
//                 println!("loop c");

//                 //break 'b;

//                 if true {
//                     continue;
//                 } else {
//                     break;
//                 }
//             }
//         }
//     }

//     // match
//     let z = 5;
//     match z {
//         1 => println!("one"),
//         2 => println!("two"),
//         3 => println!("three"),
//         4 => println!("four"),
//         5 => println!("five"),
//         6 => println!("six"),
//         _ => println!("something else"),
//     }

//     let t = 15;
//     match t {
//         1 => println!("One"),
//         2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
//         13..=19 => println!("A teen"),
//         _ => println!("Nothing special"),
//     }

//     let pair = (0, -2);
//     match pair {
//         (0, y) => println!("y : {}", y),
//         (x, 0) => println!("x : {}", x),
//         _ => println!("No match"),
//     }

//     let pair2 = (5, -5);
//     match pair2 {
//         (x,y) if x == y => println!("Equal"),
//         (x,y) if x + y == 0 => println!("Equal zero"),
//         (x,_) if x & 2 == 0 => println!("X is even"),
//         _ => println!("No match")
//     }

//     // Bind p to n
//     let p = 5;
//     match p {
//         n @ 1..=12 => println!("n:{}", n),
//         n @ 13..=19 => println!("n:{}", n),
//         _ => println!("no match")
//     }
// }
