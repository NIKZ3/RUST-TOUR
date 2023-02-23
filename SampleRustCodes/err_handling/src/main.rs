use std::fs::File;
use std::error::Error;
use std::io;

fn main() {
    println!("Hello, world!");
    let mut path = format!("/data/d1");
    let mut chunks:Vec<&str> = path.split("/").collect();
    println!("{:#?}",chunks);
    match propagate_error() {
        Ok(String) => {println!("sdsd");}
        Err(e) => {
            if let Some(err) = e.downcast_ref::<io::Error>() {
                println!("asdas");
            }
        }
    }

    if let Err(e) = propagate_error() {
        if let Some(err) = e.downcast_ref::<io::Error>() {
            println!("asdas");
        }
    }
    println!("Donw");
}

fn propagate_error() -> Result<&'static str, Box<dyn Error>> {
    let mut f = File::open("hello.txt")?;
    //Err("STSTstst")?
    Ok("string")
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
