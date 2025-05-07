#![allow(unused)]
use std::vec;

fn main() {
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];
    vec2.push(5);
    println!("1st: {}", vec2[0]);
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No 2nd value")
    }

    for i in &mut vec2 {
        *i *= 2;
    }
    for i in &vec2 {
        println!("{}", i);
    }
    println!("Vec length: {}", vec2.len());
    println!("Pop: {:?}", vec2.pop());
    for i in &vec2 {
        println!("{}", i);
    }
}
