use std::f32::consts::PI;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::channel;

fn do_something<F>(f: F)
where 
    F: Fn(),
{
    f();
}

fn takes_fnonce<F: FnOnce()>(f: F) {
    f();
}

fn takes_fn<F: Fn()>(f: F) {
    f();
    f();
}

fn takes_fnmut<F: FnMut()>(mut f: F) {
    f();
    f();
}

fn takes_a_closure_and_does_nothing<F>(f: F)
where
    F: Fn() -> i32,
{
    f();
}

#[derive(Debug)]
struct City {
    name: String,
    years: Vec<u32>,
    populations: Vec<u32>,
}

impl City {
    fn change_city_data<F>(&mut self, mut f: F)
    where 
        F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
    {
        f(&mut self.years, &mut self.populations);
    }
}

fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
    match input {
        "double" => |mut number | {
            number *= 2;
            println!("Doubling number. Now it is {number}");
            number
        },
        "triple" => |mut number | {
            number *= 3;
            println!("Tripling number. Now it is {number}");
            number
        },
        _ => |number| {
            println!("Sorry, it's the same: {number}");
            number
        }
    }
}

fn main() {
    let some_vec = vec![9,8,10];
    do_something(|| {
        some_vec
            .iter()
            .for_each(|x| println!("The number is: {x}"));
    });

    do_something(|| {
        some_vec
            .iter()
            .for_each(|x| println!("The number is: {x}"));
    });

    let mut my_string = String::from("Hello there");

    let prints_string = || {
        println!("{my_string}");
    };
    takes_fn(prints_string);

    let adds_exclamation_and_prints = || {
        my_string.push('!');
        println!("{my_string}");
    };
    takes_fnmut(adds_exclamation_and_prints);


    let prints_then_drops = || {
        println!("Now dropping {my_string}");
        drop(my_string);
    };
    takes_fnonce(prints_then_drops);

    let my_closure = || return 9;
    takes_a_closure_and_does_nothing(my_closure);

    let mut tallinn = City {
        name: "Tallinn".to_string(),
        years: vec![1372, 1834, 1897, 1925, 1959, 1989, 2000, 2010, 2020],
        populations: vec![3_250, 15_300, 58_800,
            119_800, 283_071, 478_974,
            400_378, 406_703, 437_619],
    };

    tallinn.change_city_data(|x, y | {
        x.push(2030);
        y.push(500_000);
    });

    tallinn.change_city_data(|years, populations| {
        let new_vec = years
            .iter_mut()
            .zip(populations.iter_mut())
            .take(3)
            .collect::<Vec<(_,_)>>();
        println!("{new_vec:?}");
    });

    tallinn.change_city_data(|x, y|{
        let position_option = x.iter().position(|x| *x == 1834);
        if let Some(position) = position_option {
            println!(
                "Going to delete {} at position {:?} now.",
                x[position], position
            );
            x.remove(position);
            y.remove(position);
        }
    });

    println!(
        "Years left are {:?}\nPopulations left are {:?}",
        tallinn.years, tallinn.populations
    );

    let my_number = 10;

    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut does_nothing = returns_a_closure("HI!");

    let doubled = doubles(my_number);
    let tripled = triples(my_number);
    let same = does_nothing(my_number);
    
    println!("{}, {}, {}", doubled, tripled, same);
    
    let my_number = Arc::new(Mutex::new(0));

    let cloned_1 = Arc::clone(&my_number);
    let cloned_2 = Arc::clone(&my_number);

    let thread1 = std::thread::spawn(move || {
        for _ in 0..10 {
            println!("The thread 1 is working!");
            *cloned_1.lock().unwrap() += 1;
        }
    });

    let thread2 = std::thread::spawn(move || {
        for _ in 0..10 {
            println!("The thread 2 is working!");
            *cloned_2.lock().unwrap() += 1;
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Value is: {my_number:?}");
    println!("Exiting the program");

    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = vec![];

    for _ in 0..10 {
        let my_number_clone = Arc::clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..10 {
                *my_number_clone.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle);
    }

    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{my_number:?}");

    let my_number = Mutex::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
        s.spawn(|| {
            for _ in 0..10 {
                *my_number.lock().unwrap() += 1;
            }
        });
    });

    let (sender, receiver) = channel();
    
    sender.send(5).unwrap();
    println!("{}", receiver.recv().unwrap());

    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    std::thread::spawn(move || {
        sender.send("Send a &str this time").unwrap();
        sender.send("Send a &str this time").unwrap();
    });

    std::thread::spawn(move || {
        sender_clone.send("And here is another &str").unwrap();
        sender_clone.send("And here is another &str").unwrap();
    });

    while let Ok(res) = receiver.recv() {
        println!("{res}");
    }

    let (sender, receiver) = channel();
    
    drop(receiver);
    if let Err(e) = sender.send(5) {
        println!("Got an error!: {e}");
    }
}
