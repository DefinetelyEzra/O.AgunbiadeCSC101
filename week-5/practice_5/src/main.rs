//Rust program to read the height of a person
//and then print if the person is tall, dwarf, or average height,

use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your Height (in centimeters): ");
    io::stdin().read_line(&mut input).expect("Not a valid String");
    let height:f32 = input.trim().parse().expect("Not a valid Height");

    if height >= 150.0 && height <= 170.0
    { 
        println!("You are an average height person");
    }
    else if height > 170.0 && height <= 195.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are Short");
    }
    else
    {
        println!("Abnormal height");
    }
}
