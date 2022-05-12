// Copy as template
use std::{fs::File, io::Read};

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day06/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Solution: {:?}", 123);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_1() {
//         // my test
//         assert_eq!(5, 5);
//     }
// }
