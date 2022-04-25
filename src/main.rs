
use std::env::{self, Args};
use uuid::Uuid;

fn main() {
    let mut args = env::args();
    let cnt = args.nth(1).unwrap().parse().expect("invalid count given");

    for i in 0..cnt {
        let id = Uuid::new_v4();
        println!("{}", id.to_string());
        if i % 1000000 == 0 {
            eprintln!("{}", i);
        }
    }
}

