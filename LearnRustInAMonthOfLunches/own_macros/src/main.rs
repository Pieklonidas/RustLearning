#![allow(unused)]
macro_rules! give_six {
    () => {
        6
    };
}

macro_rules! six_or_print {
    (6) => {
       6 
    };
    () => {
        println!("You didn't give me 6");
    };
}

macro_rules! check {
    ($input1:ident, $input2:expr) => {
        println!(
            "Is {:?} equal to {:?}? {:?}",
            $input1,
            $input2,
            $input1 == $input2
        );
    };
}

macro_rules! might_print {
    (Dziwne ale dziala) => {
        println!("You guessed the secret message!");
    };
    () => {
        println!("You didn't guess it");
    };
}

macro_rules! might_print2 {
    ($input:expr) => {
        println!("You gave me: {:?}", $input);
    };
}

macro_rules! wants_statement {
    ($input:stmt) => {
        println!("You matched the macro input!");
    };
}

macro_rules! print_anything {
    ($input:tt) => {
        let output = stringify!($input);
        println!("{}", output);
    };
}

macro_rules! print_anything2 {
    ($($input1:tt),*) => {
        let output = stringify!($($input1:tt),*);
        println!("{}", output);
    };
}

macro_rules! make_a_function {
    ($name:ident, $($input:tt),+) => {
        fn $name()
        {
            let output = stringify!($($input), +);
            println!("{}", output);
        }
    };
}

macro_rules! my_macro {
    () => {
        println!("Let's print this.");
    };
    ($input:expr) => {
        my_macro!();
    };
    ($($input:expr),*) => {
        my_macro!();
    };
}

macro_rules! comma_check {
    () => {
        println!("Got nothing!");
    };
    ($input:expr) => {
        println!("One expression!");
    };
    ($input:expr $(,)?) => {
        println!("One expression with a comma at the end!");
    };
    ($input:expr $(,)? $(,)?) => {
        println!("One expression with two commas at the end!");
    };
    ($input:expr $(;)? $(,)?) => {
        println!("One expression with a semicolon and a comma!");
    };
}

macro_rules! derive_try_from {
    ($type:ident, $length:expr) => {
        impl TryFrom<&str> for $type {
            type Error = String;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                let length = $length;

                if value.chars().count() > length {
                    Err(format!("Must be no longer than {length}"))
                } else {
                    Ok(Self(value.to_string()))
                }
            }
        }
    };
}

#[derive(Debug)]
struct SmallStringHolder(String);
#[derive(Debug)]
struct MediumStringHolder(String);
#[derive(Debug)]
struct LargeStringHolder(String);

derive_try_from!(SmallStringHolder, 5);
derive_try_from!(MediumStringHolder, 8);
derive_try_from!(LargeStringHolder, 12);

fn main() {
    let six = give_six!();
    println!("{}", six);

    let my_number = six_or_print!(6);
    println!("{}", my_number);
    six_or_print!();
    might_print!();
    might_print!(Dziwne ale dziala);
    might_print2!(());
    might_print2!(6);
    might_print2!(vec![8,9,10,11]);
    wants_statement!(let x = 0);

    let x = 6;
    let my_vec = vec![7, 8, 9];
    check!(x, 6);
    check!(my_vec, vec![7, 8, 9]);
    check!(x, 10);
    print_anything!(cokolwiek);
    print_anything2!(adofinaio, awepifnao);
    print_anything2!();
    print_anything2!(oafnoea, aweoifnoae, aodifnaowenf, oiaewnfo);
    make_a_function!(print_it, 5, 6, 7, 8, I);
    print_it();
    make_a_function!(say_its_nice, this, is, really, nice);
    say_its_nice();
    my_macro!(vec![8, 9, 0]);
    my_macro!(toheteh);
    my_macro!(8, 7, 0, 10);
    my_macro!();

    comma_check!();
    comma_check!(8);
    comma_check!(8,);
    comma_check!(8,,);
    comma_check!(8;,);

    println!("{}", matches!(9, 9));
    println!("{}", matches!(9, 0..=10));
    println!("{}", matches!(9, 100..=1000));

    println!("{}", matches!(9, 9 if false));

    println!("{:?}", SmallStringHolder::try_from("Hello there"));
    println!("{:?}", MediumStringHolder::try_from("Hello there"));
    println!("{:?}", LargeStringHolder::try_from("Hello there"));
}
