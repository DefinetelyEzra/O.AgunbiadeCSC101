//Rust program that takes as input the experience and age of an employee

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter your full name: ");
    io::stdin().read_line(&mut input1).expect("Invalid String");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Invalid String Value");
    let age:i32 = input2.trim().parse().expect("Invalid number");

    println!("Enter your years of experience: ");
    io::stdin().read_line(&mut input3).expect("Invalid String Value");
    let experience:i32 = input3.trim().parse().expect("Invalid number");

    if age >= 40 && experience >= 2
    {
        println!("You are entitled to ₦1,560,00 month Mr/Miss {}",input1);
    } 
    else if age > 30 && age < 40 && experience >= 2
    {
        println!("You are entitled to ₦1,480,000 per month Mr/Miss {}",input1);
    }
    else if age < 28 && experience >= 2
    {
        println!("You are entitled to ₦1,300,000 per month Mr/Miss {}",input1);
    }
    else if experience < 2
    {
        println!("You are entitled to ₦100,000 per month Mr/Miss {}",input1);
    }

}
