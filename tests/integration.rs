use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate lazy_static;

extern crate hson_gen;
use hson_gen::HsonGen;

lazy_static! {
    static ref SHORT_DATA: String = {
        let mut data = String::new();
        let mut file = File::open("tests/samples/small.hson").unwrap();
        file.read_to_string(&mut data).unwrap();

        data
    };
}

#[test]
fn test_small () {
    let mut generator = HsonGen::new();
    let html = generator.generate(&SHORT_DATA).unwrap();
    assert_eq!(html, "<!DOCTYPE html><html><head><link href=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.7/css/bootstrap.min.css\" rel=\"stylesheet\"><title>Awesome page</title></head><body><div class=\"main visible\"><article class=\"active\"><p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p><p>Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.</p></article><aside><ul class=\"right-bar active\"><li><a href=\"https://google.com\">Follow <b>the</b> Money</a></li><li><a href=\"https://mozilla.org\">Learn more</a></li></ul></aside></div></body></html>");
}