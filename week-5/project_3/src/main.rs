//Rust program for Odun's Diner 

use std::io;
use std::process;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();  

    println!("Welcome to Lekki Diner, you would like to order from us?(true or false) ");
    io::stdin().read_line(&mut input1).expect("Not a valid key");
    let response:bool = input1.trim().parse().expect("Not a valid response");
    let y:bool = true;
    let n:bool = false; 

    let menu = "   menu                         price
                 p= Poundo Yam/Edinkaiko Soup   -₦3,200
                 f= Fried Rice and Chicken      -₦3,000
                 a= Amala and Ewedu Soup        -₦2,500
                 e= Eba and Egusi Soup          -₦2,000
                 w= White Rice and Stew         -₦2,500";

    if response == n {
        println!("Sorry, and thank you for coming...");
        process::exit(0);
    }
    else if response == y {
        println!("What would you like to order...(maximum of four orders per session) (Enter the price of your order)
                  {}",menu);
    }
    println!("Order 1: ");
    io::stdin().read_line(&mut input2).expect("invalid keys");
    let order1:f32 = input2.trim().parse().expect("invalid order");

    println!("Order 2: ");
    io::stdin().read_line(&mut input3).expect("invalid keys");
    let order2:f32 = input3.trim().parse().expect("invalid order");

    println!("Order 3: ");
    io::stdin().read_line(&mut input4).expect("invalid keys");
    let order3:f32 = input4.trim().parse().expect("invalid order");

    println!("Order 4: ");
    io::stdin().read_line(&mut input5).expect("invalid keys");
    let order4:f32 = input5.trim().parse().expect("invalid order");

    println!("Enter the letter keys of your order to confirm... (eg. f, e, a): ");
    io::stdin().read_line(&mut input6).expect("Invalid keys");

    let total:f32 = order1 + order2 + order3 + order4;
    println!("Your total order is: {}",total);

    if total > 10000.00 {
        let _discount:f32 = total - ((5.00/100.00) * total);
        println!("A 5% discount has been applied to your order, your new total is: {} . Your order will be delivered to you shortly...",_discount);
    }

}
