
use std::io;

fn area_of_trapezium(h: f32, c: f32, b1: f32, b2: f32){

    let  area_of_trapezium = h / c * (b1 + b2);
    println!("The area of the trapezium is: {}",area_of_trapezium);
}

fn area_of_rhombus(half: f32, d1: f32, d2: f32){

    let  area_of_rhombus = half * d1 * d2;
    println!("The area of the rhombus is: {}",area_of_rhombus);
}

fn area_of_parallelogram(base: f32, altitude: f32){

    let area_of_parallelogram = base * altitude;
    println!("The area of the parallogram is: {}", area_of_parallelogram);
}

fn area_of_cube(sides: f32, length: f32)
{
    let area_of_cube = sides * (length * length);
    println!("The area of the cube is: {}",area_of_cube);
}

fn volume_of_cylinder(pi: f32, radius: f32, height: f32)
{
    let volume_of_cylinder = pi * (radius * radius) * height;
    println!("The volume of the cylinder is: {}",volume_of_cylinder);
}

fn main() {

    let mut pick_formula = String::new();
    println!("\nWhat formula would you like to use: ");
    io::stdin().read_line(&mut pick_formula).expect("Failed to read input");
    let formula:&str = pick_formula.trim();

    if formula == "area_of_trapezium"
    {

        let mut input1 = String::new();
        println!("\nEnter parameter for height: ");
        io::stdin().read_line(&mut input1).expect("failed to read input");
        let h:f32 = input1.trim().parse().expect("Invalid parameter");

        let mut input2 = String::new();
        println!("\nEnter parameter for base1: ");
        io::stdin().read_line(&mut input2).expect("failed to read input");
        let b1:f32 = input2.trim().parse().expect("Invalid parameter");

        let mut input3 = String::new();
        println!("\nEnter parameter for base2: ");
        io::stdin().read_line(&mut input3).expect("failed to read input");
        let b2:f32 = input3.trim().parse().expect("Invalid parameter"); 

        let c:f32 = 2.0;

        area_of_trapezium(h, c, b1, b2)
    }
    else if formula == "area_of_rhombus"
    {
        let mut input4 = String::new();
        println!("\nEnter parameter for d1: ");
        io::stdin().read_line(&mut input4).expect("failed to read input");
        let d1:f32 = input4.trim().parse().expect("invalid parameter");

        let mut input5 = String::new();
        println!("\nEnter parameter for d2: ");
        io::stdin().read_line(&mut input5).expect("failed to read input");
        let d2:f32 = input5.trim().parse().expect("invalid parameter");

        let half:f32 = 0.5;

        area_of_rhombus(half, d1, d2)
    }
    else if formula == "area_of_parallelogram"
    {
        let mut input6 = String::new();
        println!("\nEnter parameter for base: ");
        io::stdin().read_line(&mut input6).expect("failed to read input");
        let base:f32 = input6.trim().parse().expect("invalid parameter");

        let mut input7 = String::new(); 
        println!("\nEnter parameter for altitude: ");
        io::stdin().read_line(&mut input7).expect("failed to read input");
        let altitude:f32 = input7.trim().parse().expect("invalid parameter");

        area_of_parallelogram(base, altitude)
    }    
    else if formula == "area_of_cube"
    {
        let mut input8 = String::new();
        println!("\nEnter the value of length: ");
        io::stdin().read_line(&mut input8).expect("failed to read input");
        let length:f32 = input8.trim().parse().expect("invalid value");

        let sides:f32 = 6.0;

        area_of_cube(sides, length)
    }
    else if formula == "volume_of_cylinder"
    {
        let mut input9 = String::new();
        println!("\nEnter the value of the radius: ");
        io::stdin().read_line(&mut input9).expect("failed to read value");
        let radius:f32 = input9.trim().parse().expect("invalid value");

        let mut input10 = String::new();
        println!("\nEnter the value of the height: ");
        io::stdin().read_line(&mut input10).expect("failed to read value");
        let height:f32 = input10.trim().parse().expect("invalid value");

        let pi:f32 = 22.0 / 7.0;
        
        volume_of_cylinder(pi, radius, height)
    }
}
