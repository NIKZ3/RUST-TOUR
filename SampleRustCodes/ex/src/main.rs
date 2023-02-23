use std::fs::File;
use std::io;

fn main() {
    println!("Hello, world!");
    
    let result = iseven(11);

    match result {
        Ok(data) => { 
            println!("Wow {}", data);
        }
        Err(msg) => {
            println!("Not wow {}", msg);
        }
    }

    let data = propagate_error();
    
    match data {
        Ok(()) => {
            println!("aaa");
        }

        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("AAA");
            }
            other_error => {
                println!("aaa");
            }
        }
    }

    println!("Donw");
}


fn iseven (no: i32) -> Result<String, String> {

    if no%2 == 0 {
        return Ok("Number is even".to_string());
    }

    else {
        return Err("What is this".to_string()); 
    }
}

fn propagate_error() -> Result<(), io::Error> {
    let mut f = File::open("hello.txt")?;
    Ok(())
}
