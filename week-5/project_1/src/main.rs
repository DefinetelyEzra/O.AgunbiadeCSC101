//finding the roots of a quadratic equation

use std::io;

fn main() {
    
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Input value for a: ");
    io::stdin().read_line(&mut input1).expect("Invalid value");
    let valuea:f32 = input1.trim().parse().expect("Invalid number");

    println!("Input value for b: ");
    io::stdin().read_line(&mut input2).expect("Invalid value");
    let valueb:f32 = input2.trim().parse().expect("Invalid number");

    println!("Input value for c: ");
    io::stdin().read_line(&mut input3).expect("Invalid value");
    let valuec:f32 = input3.trim().parse().expect("Invalid value");

    let result:f32 = (valueb * valueb) - (4.00 * valuea * valuec);

    let determinant:f32 = result.sqrt();

    println!("The determinant is = {}",determinant);

    if determinant > 0.00 {
        println!("Therefore are two distinct roots",);
    }
    else if determinant == 0.00 {
        println!("Therefore is one real root",);
    }
    else if determinant < 0.00 {
        println!("Therefore are no real roots");
    }
}
