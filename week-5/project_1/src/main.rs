use std::io;

fn main() {
    println!("IKENNA'S BUKKA MENU");
    println!("Poundo Yam / Edinkaiko Soup : N3,200");
    println!("Fried Rice & Chicken        : N3,000");
    println!("Amala & Ewedu Soup          : N2,500");
    println!("Eba & Egusi Soup            : N2,000");
    println!("White Rice & Stew           : N2,500");

    // Read food selection
    println!("What food do you want today ");
    let mut food_input = String::new();
    io::stdin().read_line(&mut food_input).expect("Failed to read");
    let food_choice = food_input.trim();

    // Determine price
    let price: f64 = if food_choice == "Poundo Yam / Edinkaiko Soup" {
        3200.0
    } else if food_choice == "Fried Rice & Chicken" {
        3000.0
    } else if food_choice == "Amala & Ewedu Soup" {
        2500.0
    } else if food_choice == "Eba & Egusi Soup" {
        2000.0
    } else if food_choice == "White Rice & Stew" {
        2500.0
    } else {
        println!("Invalid choice.");
        return;
    };

    // Determine quantity
    println!("Enter quantity: ");
    let mut qty_input = String::new();
    io::stdin().read_line(&mut qty_input).expect("Failed to read line");
    let quantity: f64 = qty_input.trim().parse().expect("Please enter a valid number");

    // Calculate total 
    let total_charge = price * quantity;
    println!("Subtotal: N{}", total_charge);

    // Apply 5% discount if total > 10000
    let final_total = if total_charge > 10000.0 {
        let discount = total_charge * 0.05;
        println!("Discount (5%): -N{}", discount);
        total_charge - discount
    } else {
        println!("Discount: N0.00");
        total_charge
    };

    println!("Total Payable: N{}", final_total);
}