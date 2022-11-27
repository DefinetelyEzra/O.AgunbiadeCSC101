use std::io; 


fn main() {

    let mut input1 = String::new();
    println!("Enter your full name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");

    let mut input2 = String::new();
    println!("How many siblings do you have? ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let sibling_num:i32 = input2.trim().parse().expect("Not a valid number");

    if sibling_num == 1 
    {
             let mut input3 = String::new();
             println!("Enter their first name: ");
             io::stdin().read_line(&mut input3).expect("Not an expected input");
             let onlychildname:&str = input3.trim();

             let mut input4 = String::new();
             println!("Enter their age: ");
             io::stdin().read_line(&mut input4).expect("Not an expected input");
             let onlychildage:i32 = input4.trim().parse().expect("Not a vialid number");

             if onlychildage >= 18
             {
                println!("Are they single or married? ");
             }

             let mut input5 = String::new();
             io::stdin().read_line(&mut input5).expect("Not a valid response");
             let maritalstatus1:&str = input5.trim();

             if maritalstatus1 == "single"
             {
                println!("Are they a student or a worker? ");
             }
             else if maritalstatus1 == "married"
             {
                println!("Do they have any children, and what state do they live in? ");
             }

             let mut input6 = String::new();
             io::stdin().read_line(&mut input6).expect("Not a vaild response");
             let status1:&str = input6.trim();

             if status1 == "student"
             {
                println!("What is their university and course of study? ");
             }
             else if onlychildage < 18
             {
                println!("WAEC Status? ");
             }

             let mut input7 = String::new();
             io::stdin().read_line(&mut input7).expect("Not a vaild response");
             let state1:&str = input7.trim();

             let mut input8 = String::new();
             io::stdin().read_line(&mut input8).expect("Not a vaild response");
             let uni1:&str = input8.trim();

             let mut input9 = String::new();
             io::stdin().read_line(&mut input9).expect("Not a vaild response");
             let waec1:&str = input9.trim();

              if waec1 == "completed"
             {
                println!("What was the secondary school they attended? ");
             }
             else if waec1 == "pending"
             {
                println!("What is their current class level? ");
             }

             let mut input10 = String::new();
             io::stdin().read_line(&mut input10).expect("Not a vaild response");
             let secondary1:&str = input10.trim();

             let mut input11 = String::new();
             io::stdin().read_line(&mut input11).expect("Not a vaild response");
             let  class1:&str = input11.trim();

             let mut auxinput = String::new(); 
             println!("Enter their age once more in a sentence: ");
             io::stdin().read_line(&mut auxinput).expect("Not an expected input");
             let ageinfo1:&str = auxinput.trim();

             if maritalstatus1 == "married"
             {
               let info_married:[&str; 4] = [onlychildname, ageinfo1, maritalstatus1, state1];
               println!("Your sibling's info {:?}",info_married );
             }
             else if maritalstatus1 == "single" && status1 == "worker"
             {
               let info_single:[&str; 4] = [onlychildname, ageinfo1, maritalstatus1, status1];
               println!("Your sibling's info {:?}",info_single);
             }
             else if maritalstatus1 == "single" && status1 == "student" && waec1 == "completed"
             {
               let info_student:[&str; 5] = [onlychildname, ageinfo1, maritalstatus1, status1, uni1];
               println!("Your sibling's info {:?}",info_student);
             }
             else if maritalstatus1 == "single" && status1 == "student" && waec1 == "pending"
             {
               let info_student2:[&str; 6] = [onlychildname, ageinfo1, maritalstatus1, status1, secondary1, class1];
               println!("Your sibling's info {:?}",info_student2);
             }
    }
    else if sibling_num > 1 
    {
        let mut inputnum = String::new();
        io::stdin().read_line(&mut inputnum).expect("Not a valid input");
        println!("How many sibling do you have? ");
        let multiplesibling:i32 = inputnum.trim().parse().expect("Invalid input");

        for n in 0..multiplesibling {

             let mut input3 = String::new();
             println!("Enter their first name: ");
             io::stdin().read_line(&mut input3).expect("Not an expected input");
             let onlychildname:&str = input3.trim();

             let mut input4 = String::new();
             println!("Enter their age: ");
             io::stdin().read_line(&mut input4).expect("Not an expected input");
             let onlychildage:i32 = input4.trim().parse().expect("Not a vialid number");

             if onlychildage >= 18
             {
                println!("Are they single or married? ");
             }

             let mut input5 = String::new();
             io::stdin().read_line(&mut input5).expect("Not a valid response");
             let maritalstatus1:&str = input5.trim();

             if maritalstatus1 == "single"
             {
                println!("Are they a student or a worker? ");
             }
             else if maritalstatus1 == "married"
             {
                println!("Do they have any children, and what state do they live in? ");
             }

             let mut input6 = String::new();
             io::stdin().read_line(&mut input6).expect("Not a vaild response");
             let status1:&str = input6.trim();

             if status1 == "student"
             {
                println!("What is their university and course of study? ");
             }
             else if onlychildage < 18
             {
                println!("WAEC Status? ");
             }

             let mut input7 = String::new();
             io::stdin().read_line(&mut input7).expect("Not a vaild response");
             let state1:&str = input7.trim();

             let mut input8 = String::new();
             io::stdin().read_line(&mut input8).expect("Not a vaild response");
             let uni1:&str = input8.trim();

             let mut input9 = String::new();
             io::stdin().read_line(&mut input9).expect("Not a vaild response");
             let waec1:&str = input9.trim();

              if waec1 == "completed"
             {
                println!("What was the secondary school they attended? ");
             }
             else if waec1 == "pending"
             {
                println!("What is their current class level? ");
             }

             let mut input10 = String::new();
             io::stdin().read_line(&mut input10).expect("Not a vaild response");
             let secondary1:&str = input10.trim();

             let mut input11 = String::new();
             io::stdin().read_line(&mut input11).expect("Not a vaild response");
             let  class1:&str = input11.trim();

             let mut auxinput = String::new(); 
             println!("Enter their age once more in a sentence: ");
             io::stdin().read_line(&mut auxinput).expect("Not an expected input");
             let ageinfo1:&str = auxinput.trim();

             if maritalstatus1 == "married"
             {
               let info_married:[&str; 4] = [onlychildname, ageinfo1, maritalstatus1, state1];
               println!("Your sibling's info {:?}",info_married );
             }
             else if maritalstatus1 == "single" && status1 == "worker"
             {
               let info_single:[&str; 4] = [onlychildname, ageinfo1, maritalstatus1, status1];
               println!("Your sibling's info {:?}",info_single);
             }
             else if maritalstatus1 == "single" && status1 == "student" && waec1 == "completed"
             {
               let info_student:[&str; 5] = [onlychildname, ageinfo1, maritalstatus1, status1, uni1];
               println!("Your sibling's info {:?}",info_student);
             }
             else if maritalstatus1 == "single" && status1 == "student" && waec1 == "pending"
             {
               let info_student2:[&str; 6] = [onlychildname, ageinfo1, maritalstatus1, status1, secondary1, class1];
               println!("Your sibling's info {:?}",info_student2);
             }
         }
      }
}