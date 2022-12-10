use std::io::Write;

fn main() {
    
    let names = "  Student name 
    Oluchi Mordi 
    Adams Aliyu
    Shania Bolade
    Adekunle Gold
    Blanca Edemoh\n";

    let number = "  Matric. Number
    ACC10211111
    ECO10110101
    CSC10328828
    EEE11020202
    MEE10202001\n";

    let dept = "  Department
    Accounting
    Economics 
    Computer
    Electrical
    Mechanical\n";

    let level = "  Level
    300
    100
    200
    200
    100\n";

let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Create failed");
file.write_all("Welcome to PAU-SMIS\n".as_bytes()).expect("Write failed");
file.write_all(names.as_bytes()).expect("Write failed");
file.write_all(number.as_bytes()).expect("Write failed");
file.write_all(dept.as_bytes()).expect("Write failed");
file.write_all(level.as_bytes()).expect("Write Failed");

println!("\nData written to a file");
}
