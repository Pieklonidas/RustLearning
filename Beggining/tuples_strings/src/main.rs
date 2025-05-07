#![allow(unused)]
fn main() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.50);

    println!("Name: {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
    println!("Name: {}", v2);
    println!("XD: {}", v3);

    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace() {
        println!("{}", word);
    }
    
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut vec1: Vec<char> = st3.chars().collect();
    vec1.sort();
    vec1.dedup();
    for char in vec1 {
        println!("{}", char);
    }
    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();
    let st7 = String::from("Just some");
    let st8 = String::from(" words");
    let st9 = st7 + &st8;
    for char in st9.bytes() {
        println!("{}", char);
    }
}
