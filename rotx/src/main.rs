use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].parse::<i32>() {
        Result::Ok(val) => println!("You specified {}", val),
        Result::Err(e) => println!("Oops. {}", e),
    }
}
