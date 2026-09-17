use std::io;

fn main() {
    // Ask if the employee is experienced
    println!("Is the employee experienced? (type yes or no):");
    let mut exp_input = String::new();
    io::stdin()
        .read_line(&mut exp_input)
        .expect("Failed to read line");

    // Check experience
    if exp_input.trim() == "no" {
        println!("The annual incentive is: N100,000");
    } else if exp_input.trim() == "yes" {
        
        // Ask for age
        println!("Enter employee age:");
        let mut age = String::new();
        io::stdin().read_line(&mut age).expect("enter a valid age");
        let age: i8 = age.trim().parse().expect("enter a valid age");

        // final incentive calculation
        if age >= 40 {
            println!("The annual incentive is: N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("The annual incentive is: N1,480,000");
        } else if age < 28 {
            println!("The annual incentive is: N1,300,000");
        } else {
            println!("No incentive found for this age.");
        }
    } else {
        println!("Please enter 'yes' or 'no'.");
    }
}