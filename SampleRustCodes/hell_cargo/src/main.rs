mod front_of_house;

pub use crate::front_of_house::hosting;
use std::collections::HashMap;


fn borrow_this(mut data: String) {
    println!("The word is {}", data);
    data.push_str(", world");
    println!("The word is {}", data);
}

//Stuct user
#[derive(Debug)]
struct User {
    active: bool,
    name: String
}

impl User {
    fn print_name(&self) {
        println!("Name: {}", self.name)
    }

    fn print_active(&self) {
        println!("Name: {}", self.active)
    }

    fn print_complete(user: &User) {
        println!("Data is : {:#?}", user)
    }
}

fn build_user(active: bool, name: String) -> User {
    User {
        active,
        name
    }
}

// Dice Roll

fn check_dice(data: Option<i32>) {
    match data {
        Some(i) => roll_dice(i),
        _       => println!("Wtf you doin"),
    }
}

fn roll_dice(data: i32) {
    println!("The roll is {}", data)
}

fn vector_ops() {

    let mut v = vec!["str", "str", "s"];

    for i in &mut v {
        *i = "strrr";
    }
    
    v.pop();
    for i in &v {
        println!("wht {}", i);
    }

}

fn map_ops() {
    let mut sc: HashMap<&str, i32> = HashMap::new();
    sc.insert("Blue", 10);
    sc.insert("Green", 20);

    for (key, value) in &sc {
        println!("{}: {}", key, value);
    }
    let x = "Data";
    *sc.entry("Blue").or_insert(0) +=1010;
    for (key, value) in &sc {
        println!("{}: {}", key, value);
    }

    println!("{}", x);
}

fn main() {
   map_ops();
}





