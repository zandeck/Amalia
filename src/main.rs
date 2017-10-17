#[macro_use]
extern crate nom;
extern crate cgmath;

mod md5;
use md5::md5mesh_parser::parse_md5mesh;
use nom::FileProducer;
use std::fs::File;
use std::io::Read;

fn main() {

    let mut path = "./Resources/bob_lamp_update/bob_lamp_update.md5mesh";
    let mut f = File::open(path).unwrap();
    let mut buff = vec![];

    f.read_to_end(&mut buff).unwrap();
    let res = parse_md5mesh(&buff);
    println!("{:?}", &res);
}