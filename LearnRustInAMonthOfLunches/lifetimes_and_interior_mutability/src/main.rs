use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

fn works() -> &'static str {
    "I live forever!"
}

#[derive(Debug)]
struct City<'a> {
    name: &'a str,
    date_founded: u32,
}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points", self.name, self.hit_points)
    }
}

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

impl PhoneModel {
    fn make_not_on_sale(&self) {
        self.on_sale.set(false);
    }
}

#[derive(Debug)]
struct User {
    id: u32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

fn main() {
    println!("{}", works());

    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
    let my_city = City {
        name: &city_names[0],
        date_founded: 1921,
    };
    println!("{} was founded in {}", my_city.name, my_city.date_founded);

    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };

    println!("{}", billy);
    billy.take_damage();

    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    super_phone_3000.make_not_on_sale();
    println!("{super_phone_3000:#?}");

    let user_1 = User {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{:?}", user_1.active);

    let mut borrow = user_1.active.borrow_mut();
    *borrow = false;
    // *user_1.active.borrow_mut() = false;

    let my_mutex = Mutex::new(5);
    let mut mutex_changer = my_mutex.lock().unwrap();

    println!("{my_mutex:?}");

    println!("{mutex_changer:?}");
    *mutex_changer = 6;

    println!("{mutex_changer:?}");

    drop(mutex_changer);
    println!("{my_mutex:?}");

    let my_mutex = Mutex::new(5);
    {
        let mut mutex_changer = my_mutex.lock().unwrap();
        *mutex_changer = 6;
        let mut other_mutex_changer = my_mutex.try_lock();

        if let Ok(value) = other_mutex_changer {
            println!("The MutexGuard has: {value}");
        }
        else {
            println!("Didn't get the lock");
        }
    }
 
    println!("{my_mutex:?}");
    *my_mutex.lock().unwrap() = 7;
    println!("{my_mutex:?}");

    let my_rwlock = RwLock::new(5);
    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    if let Ok(mut number) = my_rwlock.try_write() {
        *number += 10;
        println!("Now the number is {}", number);
    } else {
        println!("Couldn't get write access, sorry!")
    };

    println!("{read1:?}, {read2:?}");
    drop(read1);
    drop(read2);

    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;
    drop(write1);
    println!("{:?}", my_rwlock);

}
