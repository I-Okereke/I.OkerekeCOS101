
use std::io;

fn main() {

    //get number of candidates
    let mut number = String::new();
        println!("Enter number of candidates");
            io::stdin().read_line(&mut number).expect("Fail");
                let number: i32 = number.trim().parse().expect("Fail");

    if number <= 0 {
        println!("Fail");
        return;
    }

    //store candidates and their experience
    let mut candidates: Vec<(String, i32)> = Vec::new();

    for count in 0..number {

        //get candidate name
        let mut name = String::new();
            println!("Enter candidate {}", count+1);
                io::stdin().read_line(&mut name).expect("Fail");
                    let name: String = name.trim().parse().expect("Fail");

        //get years of experience
        let mut exp = String::new();
            println!("Enter years of experience");
                io::stdin().read_line(&mut exp).expect("Fail");
                    let exp: i32 = exp.trim().parse().expect("Fail");

        candidates.push((name, exp));

    }

    //find the candidate with the highest experience
    let mut highest = candidates[0].1;
    let mut index = 0;

    //loop to iterate elements in vector
    for i in 0..candidates.len()
    {
        //iterating through i on the vector
        if candidates[i].1 > highest {
            highest = candidates[i].1;
            index = i;
        }
    }

    //display the most experienced candidate
    println!("Most experienced candidate: {}", candidates[index].0);
    println!("Years of experience: {}", highest);

}
