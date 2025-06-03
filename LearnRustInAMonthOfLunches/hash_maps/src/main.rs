

use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::num::ParseIntError;

fn parse_and_log_str(input: &str) -> Result<i32, ParseIntError> {
    let parsed_number = input.parse::<i32>()?;
    println!("Number parsed successfully into {parsed_number}");
    Ok(parsed_number)
}

fn main() {
    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "Pan Tadeusz");
    let key = 1;
    match book_hashmap.get(&key) {
        Some(value) => println!("Key {key} has a value already: {value}"),
        None => {
            book_hashmap.insert(key, "Ferdydurke");
        }
    }

    println!("{:?}", book_hashmap);

    let mut old_hashmap_values = Vec::new();
    let hashmap_entries = [
        (1, "Krzyzacy"),
        (1, "Le Petit Prince"),
        (1, "Polonez"),
        (1, "Wiedzmin")
    ];

    for (key, value) in hashmap_entries {
        if let Some(old_value) = book_hashmap.insert(key, value) {
            println!("Overwriting {old_value} with {value}!");
            old_hashmap_values.push(old_value);
        }
    }
    println!("All old values: {old_hashmap_values:?}");

    let book_collection = vec!["Sezon burz", "Krew elfow", "Czas pogardy", "Sezon burz"];

    let mut book_hashmap = HashMap::new();
     
    for book in book_collection {
        book_hashmap.entry(book).or_insert(true);
    }
    for (book, true_or_false) in book_hashmap {
        println!("Do we have {book}? {}", true_or_false)
    }

    let book_collection = vec!["Sezon burz", "Krew elfow", "Czas pogardy", "Sezon burz"];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0);
        *return_value += 1;
    }

    for (book, number) in book_hashmap {
        println!("{book}, {number}");
    }

    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];

    println!("How many numbers in the Vec? {}", many_numbers.len());
    
    let mut number_hashset = HashSet::new();
    
    for number in many_numbers {
        number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!(
        "There are {hashset_length} unique numbers, so we are missing {}.",
        50 - hashset_length
    );
    println!("It does not contain: ");
    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }

    let many_numbers = vec![37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48,
        28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21, 20, 38, 16, 48, 39, 31, 41,
        32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9, 46, 33];
    
    let mut current_number = i32::MIN;
    let mut number_set = BTreeSet::new();
    for number in many_numbers {
        number_set.insert(number);
    }
    for number in number_set {
        if number < current_number {
            println!("This will never happen")
        }
        current_number = number;
    }

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }

    let mut jobs = BinaryHeap::new();

    jobs.push((100, "Reply to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    while let Some((_, job)) = jobs.pop() {
        println!("You need to: {job}");
    }

    let mut my_vec = VecDeque::from(vec![0; 600_000]);
    for _ in 0..600_000 {
        my_vec.pop_front();
    }

    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060", "89 elo"];
    for item in str_vec {
        let parsed = parse_and_log_str(item);
        println!("Result: {parsed:?}");
    }
}
