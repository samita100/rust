use rand::Rng;
use std::io;
fn main() {
    println!("Hello world :)");  
    let secret_number = rand::thread_rng().gen_range(1..101);
    match secret_number {
        1 => println!("lolo"),
        2 => println!("Golo"),
        _ => println!("nthing"),
    }
}

fn maina() {
    let rohan :i32 = rand::thread_rng().gen_range(1..100);
}