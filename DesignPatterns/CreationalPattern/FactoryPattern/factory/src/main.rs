/*

Factory Design Pattern
Construct a Car.
Get a car.

 */

pub trait CAR {
    fn get_product(&self);
    fn get_chasis(&self);
}

#[derive(Debug)]
pub struct BMW {
    pub product: String,
    pub chasis: String
}

impl CAR for BMW {
    fn get_product(&self) {
        println!("This is your car {:?}", self);
    }

    fn get_chasis(&self) {
        println!("This is your chasis {:?}", self);
    }
}


#[derive(Debug)]
pub struct LAMBO {
    pub product: String,
    pub chasis: String
}

impl CAR for LAMBO {
    fn get_product(&self) {
        println!("This is your car {:?}", self);
    }

    fn get_chasis(&self) {
        println!("This is your chasis {:?}", self);
    }
}

enum carType {
    BMW,
    LAMBO
}

struct carFactory;

impl carFactory {

    fn new(s: carType) -> Box<dyn CAR> {
        match s {
            carType::BMW => return Box::new(BMW{
                product: String::from("BMW X1"),
                chasis: String::from("BMW base chasis")}),
            carType::LAMBO => return Box::new(BMW{
                product: String::from("Lambo roadster"),
                chasis: String::from("Lambo sport chasis")})
        }
    }

}

fn main() {
    println!("Hello, world!");

    let car = carFactory::new(carType::BMW);
    let car2 = carFactory::new(carType::LAMBO);
    car.get_chasis();
    car.get_product();
    car2.get_chasis();
    car2.get_product();

}
