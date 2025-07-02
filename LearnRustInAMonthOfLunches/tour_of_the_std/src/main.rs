use rand::random;
use core::num;
use std::fmt::{self, format};
use std::i64;
use std::ops::Add;

#[derive(Debug)]
enum Hours {
    Working(u32),
    NotWorking(u32),
    Error(u32),
}

impl From<u32> for Hours {
    fn from(value: u32) -> Self {
        match value {
            hour if (8..17).contains(&hour) => Hours::Working(value),
            hour if (0..=24).contains(&hour) => Hours::NotWorking(value),
            wrong_hour => Hours::Error(wrong_hour),
        }
    }
}

fn add_numbers(one: u8, two: u8) {
    match one.checked_add(two) {
        Some(num) => println!("Added {one} to {two}: {num}"),
        None => println!("Error: couldn't add {one} to {two}"),
    }
}

#[derive(Clone)]
struct Country {
    name: String,
    population: u32,
    gdp: u32,
}

impl Country {
    fn new(name: &str, population: u32, gdp: u32) -> Self {
        Self { name: name.to_string(), population, gdp }
    }
}

impl Add for Country {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            name: format!("{} and {}", self.name, rhs.name),
            population: self.population + rhs.population,
            gdp: self.gdp + rhs.gdp,
        }
    }
}

impl fmt::Display for Country {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In {} are {} people and a GDP of ${}",
            self.name, self.population, self.gdp
        )
    }
}

trait ChangeForm {
    type SomethingElse;
    fn change_form(self) -> Self::SomethingElse;
}

impl ChangeForm for String {
    type SomethingElse = char;
    fn change_form(self) -> Self::SomethingElse {
        self.chars().next().unwrap_or(' ')
    }
}

impl ChangeForm for i32 {
    type SomethingElse = i64;
    fn change_form(self) -> Self::SomethingElse {
        println!("i32 just got really big!");
        i64::MAX
    }
}

fn main() {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];
    let [city1, _city2, _city3] = my_cities;
    println!("{city1}");

    let my_cities = ["Beirut", "Tel Aviv", "Calgary", "Nicosia", "Seoul"];
    let [first, .., last] = my_cities;
    println!("{first}, {last}");

    let int_array = [1, 5, 9, 13, 17, 21, 25, 29];
    let string_array = int_array.map(|i| i.to_string());
    println!("{int_array:?}");
    println!("{string_array:?}");

    let int_array = [1, 5, 9, 13, 17, 21, 25, 29];
    let hours_array = int_array.map(Hours::from);
    println!("{hours_array:?}");

    let array = std::array::from_fn(|i| i);
    assert_eq!(array, [0, 1, 2, 3, 4]);

    println!("This will always work: {}", char::from(100));
    println!("So will this: {}", char::from(random::<u8>()));

    for _ in 0..100_000 {
        if let Ok(successful_character) = char::try_from(random::<u32>()) {
            print!("{successful_character}");
        }
    }
    println!("");

    for _ in 0..3 {
        let some_number = random::<u8>();
        let other_number  = random::<u8>();
        add_numbers(some_number, other_number);
    }

    let nauru = Country::new("Nauru", 12_511, 133_200_000);
    let vanuatu = Country::new("Vanuatu", 219_137, 956_300_000);
    let micronesia = Country::new("Micronesia", 113_131, 404_000_000);

    println!("{}", nauru);
    let nauru_and_vanuatu = nauru + vanuatu;
    println!("{nauru_and_vanuatu}");
    println!("{}", nauru_and_vanuatu + micronesia);

    let nums = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, -7.77, -10.0];
    let max = nums
        .iter()
        .fold(f64::MIN, |num, next_num| num.max(*next_num));

    let min: f64 = nums
        .iter()
        .fold(f64::MAX, |num, next_num| num.min(*next_num));
    println!("{max}, {min}");

    let string1 = "Hello there!".to_string();
    println!("{}", string1.change_form());

    let string2 = "I'm back!".to_string();
    println!("{}", String::change_form(string2));

    let small_num = 1;
    println!("{}", small_num.change_form());

    let mut my_string =  String::from(".daer ot drah tib elttil a si gnirts sihT");

    while let Some(c) = my_string.pop() {
        print!("{c}");
    }
}
