#![allow(unused)]

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String{
    println!("A string {}", x);
    return x
}

fn change_string(name: &mut String){
    name.push_str(" is happy");
    println!("Message: {}", name);
}

fn main() {
    let str1 = String::from("World");
    let str2 = str1.clone();
    println!("Hello {}", str1);
    println!("Hello {}", str2);

    print_str(str1);
    let str3 = print_return_str(str2);
    println!("str3 = {}", str3);
    let mut str4 = str3.clone();
    change_string(&mut str4);
}
