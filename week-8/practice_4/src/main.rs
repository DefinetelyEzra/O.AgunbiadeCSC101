fn main() {
    
    // Name vector
    let name = vec!["Mary", "Susan", "Sally", "Greg", "Ade", "Mark", "June", "Ife"];

    // Age vector
    let age = vec![16,17,19,22,20,21,18,21];

    println!("\nAge allocation:\n");

    // Loop to iterate elements in vector
    for i in 0..age.len()
    {
        //iterating through i in the vector
        println!("{} is {} years old\n",name[i],age[i]);
    }
}

