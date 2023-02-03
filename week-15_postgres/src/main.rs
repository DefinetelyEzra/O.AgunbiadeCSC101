use std::io::Read;
use std::io;

fn main(){
    let mut input1 = String::new();

    println!("Welcome to Globacom database, please specify your user identification...");
    io::stdin().read_line(&mut input1).expect("not a valid response");
    let response:&str = input1.trim();

    if response == "administrator" || response == "Administrator" 
    {
        administrator()
    }
    else if response == "project manager" || response == "Project manager" || response == "Project Manager"
    {
        project_manager()
    }
    else if response == "employee" || response == "Employee" 
    {
        employee()
    }
    else if response == "customer" || response == "Customer"
    {
        customer()
    }
    else if response == "vendor" || response == "Vendor"
    {
        vendor()
    }
}

fn administrator(){
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("Welcome Administrator...
              {}", contents);

}

fn project_manager(){
    let mut file2 = std::fs::File::open("project_tb.sql").unwrap();
    let mut contents2 = String::new();
    file2.read_to_string(&mut contents2).unwrap();
    println!("Welcome Project manager...
             {}", contents2);
}

fn employee(){
    let mut file3 = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents3 = String::new();
    file3.read_to_string(&mut contents3).unwrap();
    println!("Welcome Member of Staff...
             {}", contents3);
}

fn customer(){
    let mut file4 = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents4 = String::new();
    file4.read_to_string(&mut contents4).unwrap();
    println!("Welcome Customer...
             {}", contents4);
}

fn vendor(){
    let mut file5 = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents5 = String::new();
    file5.read_to_string(&mut contents5).unwrap();
    println!("Welcome Vendor...
             {}", contents5);
}