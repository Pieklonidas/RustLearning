#![allow(unused)]

use std::default;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain
}

impl Default for Character {
    fn default() -> Self {
        Self { 
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}

impl Character {
    fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    fn weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    fn build(mut self) -> Result<Character, String> {
        if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
            Ok(self)
        } 
        else {
            Err("Could not create character. Characters must have:
1) Height below 200
2) Weight below 300
3) A name that is not Smurf (that is a bad word)".to_string())
        }
    }
}

#[derive(Debug, Default)]
struct Size {
    height: f64,
    length: f64,
    width: f64,
}

#[derive(Debug)]
pub struct Character2 {
    name: String,
    age: u8,
}

impl Default for Character2 {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
        }
    }
}

#[derive(Debug)]
pub struct CharacterBuilder {
    pub name: String,
    pub age: u8,
}

impl CharacterBuilder {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn try_build(self) -> Result<Character2, &'static str> {
        if !self.name.to_lowercase().contains("smurf") {
            Ok(Character2 { name: self.name, age: self.age })
        }
        else {
            Err("Can't make a character with the word 'smurf' inside it!")
        }
    }
}

fn do_something_with_character(character: &Character2) {}

#[derive(Debug)]
struct HoldsANumber(u8);

impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let character_1 = Character::default();

    println!(
        "The character {:?} is {:?} years old",
        character_1.name, character_1.age
    );

    let only_height = Size {
    height: 1.0,
    ..Default::default()
    };

    println!("{only_height:?}");

    let character_1 = Character::default().height(180).weight(60).name("Bobby").build();
    println!("{character_1:?}");

    let character_with_smurf = Character::default().name("Smurf").build();

    let character_too_tall = Character::default().height(300).build();
    
    let character_too_heavy = Character::default().weight(440).build();

    let okay_character = Character::default().height(180).weight(60).name("Billy").build();

    let character_vec = vec![
        character_with_smurf, character_too_tall, character_too_heavy, okay_character
    ];

    for character in character_vec {
        match character {
            Ok(character) => println!("{character:?}\n"),
            Err(err) => println!("{err}\n"),
        }
    }

    let default_character = Character2::default();
    do_something_with_character(&default_character);
    let second_character = CharacterBuilder::new("Bobby".to_string(), 27)
        .try_build()
        .unwrap();
    do_something_with_character(&second_character);
    let bad_character = CharacterBuilder::new("SmurfySmurf".to_string(), 27).try_build();
    println!("{bad_character:?}");

    let my_number = HoldsANumber(20);
    println!("{:?}", *my_number + 20);
    
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();

    let mut my_number = HoldsANumber(20);
    *my_number = 30;
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();

}
