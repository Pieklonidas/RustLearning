#![allow(unused)]

use std::ops::Add;

fn say_hello() {
    println!("Hello world!");
}

fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x+y);
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x+y
}

fn get_sum_3(x: i32, y: i32) -> i32 {
    return x+y;
}

fn get_2(x: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += &val;
    }
    sum
}

fn get_sum_gen<T:Add<Output =  T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    say_hello();
    get_sum(5,4);
    println!("{}", get_sum_2(2, 22));
    let (val1, val2) = get_2(1);
    println!("Nums: {} {}", val1, val2);

    let num_list = vec![1,3,4,5,2,6,7];
    println!("Sum of list = {}", sum_list(&num_list));

    println!("5 + 4 = {}", get_sum_gen(5, 4));
    println!("5.2 + 4.5 = {}", get_sum_gen(5.2, 4.5));
}
