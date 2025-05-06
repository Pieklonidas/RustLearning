#![allow(unused)]

use std::{cmp::Ordering, i32};

fn main() {
    let age = 8;
    if(age >= 1) && (age <= 18)
    {
        println!("Important Birthday");
    }
    else if (age == 21) || (age == 50) 
    {
        println!("Another important Birthday");
    } 
    else if age >= 65
    {
        println!("afdan important Birthday");
    }
    else 
    {
        println!("Not an important Birthday");
    }

    let mut my_age = 47;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    let age2 = 8;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an important Birthday")
    };

    my_age = 18;
    let voting_age = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You gained the right to vote")
    };
}
