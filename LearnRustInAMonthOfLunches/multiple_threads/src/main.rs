use std::{borrow::Cow, vec};
use std::rc::Rc;

#[derive(Debug)]
struct ErrorInfo {
    error: LocalError,
    message: String,
}

#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,   
}

fn generate_message(
    message: &'static str,
    error_info: Option<ErrorInfo>,
) -> Cow<'static, str> {
    match error_info {
        None => message.into(),
        Some(info) => format!("{message}: {info:?}").into()
    }
}

struct User {
    name: Cow<'static, str>
}

#[derive(Debug)]
struct City {
    name: Rc<String>,
    population: u32,
    city_history: Rc<String>,
}

#[derive(Debug)]
struct CityData {
    names: Vec<Rc<String>>,
    histories: Vec<Rc<String>>,
}

fn main() {
    let msg1 = generate_message("Everything is fine", None);
    let msg2 = generate_message(
        "Got en error", 
        Some(ErrorInfo { 
            error: LocalError::TooBig,
            message: "It was too big".to_string(), 
        })
    );

    for msg in [msg1, msg2] {
        match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, didn't need an allocation:\n {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned, because we needed an allocation:\n {msg}")
            }
        }
    }

    let user_name = "User1";
    let other_user_name = "User10".to_string();

    let user1 = User {
        name: user_name.into()
    };

    let user2 = User {
        name: other_user_name.into()
    };

    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(n) => {
                println!("Borrowed name, didn't need an allocation:\n  {n}")
            }
            Cow::Owned(n) => {
                println!("Owned name because we needed an allocation:\n  {n}")
            }
        }
    }

    let calgary_name = Rc::new("Calgary".to_string());
    let calgary_history = Rc::new("Calgary began as a fort called Fort Calgary that...".to_string());

    let calgary = City {
        name: Rc::clone(&calgary_name),
        population: 1_200_000,
        city_history: Rc::clone(&calgary_history),
    };

    let canada_cities = CityData {
        names: vec![Rc::clone(&calgary_name)],
        histories: vec![Rc::clone(&calgary_history)],
    };

    println!("Calgary's history is {}", calgary.city_history);
    println!("{}", Rc::strong_count(&calgary.city_history));

    let mut join_handles = vec![];

    for num in 0..10 {
        let handle = std::thread::spawn(move || {
            println!("I am printing something: {num}");
        });
        join_handles.push(handle);
    }
    for handle in join_handles {
        handle.join().unwrap();
    }
}
