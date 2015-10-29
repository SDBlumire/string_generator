#![cfg(test)]

use rand::Rng;
use super::gen_string;
use std::collections::HashMap;

#[test]
fn it_works() { }

struct FakeRng;
impl Rng for FakeRng {
    fn next_u32(&mut self) -> u32 {
        4
    }
}

#[test]
fn gen_string_fof() {
    let mut catagories: HashMap<String, &[u8]> = HashMap::new();
    let cons = "bcdfg".as_bytes();
    let vowels = "aeiou".as_bytes();
    catagories.insert(String::from("C"), cons);
    catagories.insert(String::from("V"), vowels);
    let pattern = vec![String::from("C"), String::from("V"), String::from("C")];
    assert_eq!( gen_string(&catagories, pattern, &mut FakeRng).ok(), Some(String::from("fof")) );
}
