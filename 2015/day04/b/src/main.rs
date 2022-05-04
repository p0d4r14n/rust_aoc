use md5::{Digest, Md5};
use std::{fs::File, io::Read};

fn find_number(data: String) -> usize {
    let mut i = 0;
    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", data, i));
        let hash = format!("{:x}", hasher.finalize());

        if hash.get(0..6).unwrap() == "000000" {
            return i;
        } else {
            i += 1;
            continue;
        }
    }
}

fn main() {
    let mut data = String::new();
    let mut f = File::open("2015/day04/input.txt").expect("Error on File::open");
    f.read_to_string(&mut data)
        .expect("Error on read_to_string");

    println!("Number: {:?}", find_number(data));
}
