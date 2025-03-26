mod position;
use position::*;
mod int_position;
use int_position::IntPosition;
mod float_position;
use float_position::FloatPosition;
mod string_position;
use string_position::{Point, StringPosition};

fn main() {
    // let mut f_pos = FloatPosition::new();
    // for i in 1..=10 {
    //     f_pos.add(&i.to_string());
    // }
    // println!("indices {:?}", f_pos);
    // println!("order {:?}", f_pos.order());
    // println!("indices {:?}", f_pos);
    // let mut f_pos = FloatPosition::from_keys(vec!["a", "z"]);
    // for i in 0..64 {
    //     f_pos.insert(&i.to_string(), 1);
    // }
    // println!("order {:?}", f_pos.order());
    // println!("indices {:?}", f_pos);
}


fn sample() {
    // let mut expected: Vec<String> = ('a'..='z').map(|c| c.to_string()).collect();
    // let mut keys: Vec<String> = ('b'..='z').map(|c| c.to_string()).collect();
    // keys.sort_by(|a, b| b.cmp(a));

    // let mut f_pos = FloatPosition::from_keys(vec!["a", "z"]);
    // for k in keys.iter().map(|k| k.as_str()) {
    //     f_pos.insert(k, 1);
    // }
    // println!("indices: {:?}", f_pos);
    // println!("keys: {:?}", f_pos.keys());
    // println!("order: {:?}", f_pos.order());
    let mut f_pos = FloatPosition::from_keys(vec!["a", "z"]);
    for i in 0..1060 {
        f_pos.insert(&i.to_string(), 1);
    }
    println!("order {:?}", f_pos.order());
    println!("indices {:?}", f_pos);
}
