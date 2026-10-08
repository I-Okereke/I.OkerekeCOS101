use std::io;

fn checker(){

    let mut input = String::new();
    println!("Enter a character");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let ch:char = input.trim().parse().expect("Invalid input");

    if ch >= '0' && ch <= '9'
    {
        println!("character '{}' is a digit",ch);
    }
    else {
        println!("Charactier '{}' is not a digir",ch);
    }
}

fn main() {
    //calling function
    println!("Welcome! This program checks whether a charactier variable 
        contains a digit of not");
    checker()
}