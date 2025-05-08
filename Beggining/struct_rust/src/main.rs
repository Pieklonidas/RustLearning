#![allow(unused)]
fn main() {
    struct Customer{
        name: String,
        address: String,
        balance: f32
    }
    let mut bob = Customer{
        name: String::from("Bob Smith"),
        address: String::from("555 Main street"),
        balance: 234.54
    };

    bob.address = String::from("505 main street");

    struct Rectangle<T, U>{
        length: T,
        height: U
    }
    
    let rec = Rectangle{
        length: 4,
        height: 10.5
    };

    trait Shape{
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    struct Circle {length: f32, width: f32};

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle{
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    let circ: Circle = Shape::new(10.0, 10.0);
    println!("Circle Area: {}", circ.area());
}
