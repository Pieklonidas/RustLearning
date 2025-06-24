#![allow(unused)]
#![allow(static_mut_refs)]

use std::sync::Mutex;
use std::mem::transmute;
use rand::random;

#[derive(Debug)]
struct Buffers<T, const N: usize> {
    array_one: [T; N],
    array_two: [T; N],
}

const NUMBER: u8 = give_eigth();

const fn give_eigth() -> u8 {
    8
}

#[derive(Debug)]
struct Log {
    date: &'static str,
    message: String,
}

static GLOBAL_LOGGER: Mutex<Vec<Log>> = Mutex::new(Vec::new());

fn add_message(date: &'static str) {
    GLOBAL_LOGGER.lock().unwrap().push(Log {
        date,
        message: "Everything is fine".to_string(),
    });
}

static mut NUMBER2: u32 = 0;

fn main() {
    let buffer_1 = Buffers {
        array_one: [0u8; 3],
        array_two: [0; 3],
    };

    let buffer_2 = Buffers {
        array_one: [0i32; 4],
        array_two: [10; 4],
    };

    println!("{buffer_1:#?}, {buffer_2:#?}");

    let mut my_vec = Vec::new();
    my_vec.push(give_eigth());

    add_message("2022-12-12");
    add_message("2023-05-05");
    println!("{GLOBAL_LOGGER:#?}");

    let mut join_handle_vec =vec![];
    for _ in 0..1000 {
        join_handle_vec.push(std::thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    NUMBER2 += 1;
                }
            }
        }));
    }

    for handle in join_handle_vec {
        handle.join().unwrap();
    }

    unsafe {
        println!("{NUMBER2}");
    }

    let x = -19;
    let y: u32 = unsafe {
        transmute::<i32, u32>(x)
    };

    println!("{y}");

    for _ in 0..5 {
        let random_u16 = random::<u16>();
        print!("{random_u16} ");
    }
    print!("\n");
}
