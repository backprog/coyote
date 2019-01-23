use std::fs::File;
use std::io::prelude::*;

extern crate coyote;
use coyote::Coyote;


fn main() {
    let mut data = String::new();
    let mut file = File::open("tests/samples/small.hson").unwrap();
    file.read_to_string(&mut data).unwrap();

    let mut coyote = Coyote::new().unwrap();
    let html = coyote.gen(&data).unwrap();

    println!("{}", &html);
}
