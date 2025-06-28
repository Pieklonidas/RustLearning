use std::io;
use std::env::args;
use std::fs::{self, read_to_string};
use std::io::Write;
use std::fs::File;
use std::io::Read;

/// This is an enum
enum Letters {
    Capitalize,
    Lowercase,
    Nothing,
}
fn main() -> std::io::Result<()> {
    println!("Please type something, or x to escape");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {input_string}");
    }
    println!("See you later!");

    println!("{:?}", std::env::args());

    let input = args();
    for entry in input {
        println!("You entered: {}", entry);
    }

    let input = args();
    input.skip(1).for_each(|item| {
        println!(
            "You wrote {item}, which in capital letters is {}",
            item.to_uppercase()
        );
    });

    let mut changes = Letters::Nothing;
    let input = args().collect::<Vec<_>>();

    if let Some(arg) = input.get(1) {
        match arg.as_str() {
            "capital" => changes = Letters::Capitalize,
            "lowercase" => changes = Letters::Lowercase,
            _ => {}
        }
    }

    for word in input.iter().skip(2) {
        match changes {
            Letters::Capitalize => println!("{}", word.to_uppercase()),
            Letters::Lowercase => println!("{}", word.to_lowercase()),
            _ => println!("{}", word),
        }
    }
    
    fs::File::create("myfilename.txt")?
        .write_all(b"Let's put this in the file")?;
    fs::write("calvin_with_dad.txt",
"Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
Dad: Sure they did. In fact, those photographs are in color. It's just the world was black and white then.")?;

    let mut calvin_file = File::open("calvin_with_dad.txt")?;
    let mut calvin_string = String::new();
    calvin_file.read_to_string(&mut calvin_string)?;
    calvin_string.split_whitespace().for_each(|word| print!("{} ", word.to_uppercase()));

    let mut calvin_file = File::options()
        .append(true)
        .read(true)
        .open("calvin_with_dad.txt")?;
    calvin_file.write_all(b"\nCalvin: Really?\n")?;
    write!(&mut calvin_file, "Dad: Yep. The world didn't turn color until sometime in the 1930s...\n")?;
    println!("");
    println!("{}", read_to_string("calvin_with_dad.txt")?);
    
    let main = include_str!("main.rs");
    println!("Here's what main.rs looks like: \n\n{main}");
    Ok(())
}
