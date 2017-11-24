extern crate eb_core;


use std::env;
use eb_core::track::Track;


fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    println!("reading: {}", fp);
    let track = Track::from_file(fp);
}
