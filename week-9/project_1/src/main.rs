use std::io::Write;

fn main() {
    
    let lager = "  Lager 
    33 Exports 
    Desperados
    Goldberg
    Gulder
    Heniken
    Star\n";

    let stout = "  Stout
    Legend
    Turbo King
    Williams\n";

    let non_alcoholics = " Non-Alcoholic
    Maltina
    Amstel Malta
    Malta Gold
    Fayrouz\n";

let mut file = std::fs::File::create("Nigerian-Brewery-LTD.txt").expect("Create failed");
file.write_all("Welcome to Nigerian Brewery Limited\n".as_bytes()).expect("Write failed");
file.write_all(lager.as_bytes()).expect("Write failed");
file.write_all(stout.as_bytes()).expect("Write failed");
file.write_all(non_alcoholics.as_bytes()).expect("Write failed");

println!("\nData written to a file");
}
