use std::io;

fn main() {

//find what job the person does
    let mut job = String::new();
        println!("Enter your job");
            io::stdin().read_line(&mut job).expect("Fail");
                let job:String = job.trim().parse().expect("Fail");

//get their experience level
    let mut exp = String::new();
        println!("Enter years of experience");
            io::stdin().read_line(&mut exp).expect("Fail");
                let exp:i8 = exp.trim().parse().expect("Fail");


    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    let office = vec!["Intern", "Administrator", "Senior Administrator","Office Manager", "Director", "CEO"];

    let academic = vec!["-", "Research Assistant", "PhD Candidate","Post-Doc Researcher", "Senior Lecturer", "Dean"];

    let lawyer = vec!["Paralegal", "Junior Associate", "Associate","Senior Associate 1-2", "Senior Associate 3-4", "Partner"];

    let teacher = vec!["Placement", "Classroom Teacher", "Senior Teacher","Leading Teacher", "Deputy Principal", "Principal"];


//calculate the proper index to use using experience level
    if exp >= 1 && exp <= 2 
    {
        let index = 0;
        if job == "office admin" {
            println!("Your level is: {}", office[index]);
        } else if job == "academic" {
            println!("Your level is: {}", academic[index]);
        } else if job == "lawyer" {
            println!("Your level is: {}", lawyer[index]);
        } else if job == "teacher" {
            println!("Your level is: {}", teacher[index]);
        } else {
            println!("No job like this");
        }
        println!("Level: {}", levels[index]);

    } 

    else if exp >= 3 && exp <= 5 
    {
        let index = 1;
        if job == "office admin" {
            println!("Your level is: {}", office[index]);
        } else if job == "academic" {
            println!("Your level is: {}", academic[index]);
        } else if job == "lawyer" {
            println!("Your level is: {}", lawyer[index]);
        } else if job == "teacher" {
            println!("Your level is: {}", teacher[index]);
        } else {
            println!("No job like this");
        }
        println!("Level: {}", levels[index]);

    } 

    else if exp >= 6 && exp <= 8 
    {
        let index = 2;
        if job == "office admin" {
            println!("Your level is: {}", office[index]);
        } else if job == "academic" {
            println!("Your level is: {}", academic[index]);
        } else if job == "lawyer" {
            println!("Your level is: {}", lawyer[index]);
        } else if job == "teacher" {
            println!("Your level is: {}", teacher[index]);
        } else {
            println!("No job like this");
        }
        println!("Level: {}", levels[index]);

    } 

    else if exp >= 9 && exp <= 10 
    {
        let index = 3;
        if job == "office admin" {
            println!("Your level is: {}", office[index]);
        } else if job == "academic" {
            println!("Your level is: {}", academic[index]);
        } else if job == "lawyer" {
            println!("Your level is: {}", lawyer[index]);
        } else if job == "teacher" {
            println!("Your level is: {}", teacher[index]);
        } else {
            println!("No job like this");
        }
        println!("Level: {}", levels[index]);

    } 

    else if exp >= 11 && exp <= 13 
    {
        let index = 4;
        if job == "office admin" {
            println!("Your level is: {}", office[index]);
        } else if job == "academic" {
            println!("Your level is: {}", academic[index]);
        } else if job == "lawyer" {
            println!("Your level is: {}", lawyer[index]);
        } else if job == "teacher" {
            println!("Your level is: {}", teacher[index]);
        } else {
            println!("No job like this");
        }
        println!("Level: {}", levels[index]);

    } 

    else if exp > 13 
    {
        let index = 5;
        if job == "office admin" {
            println!("Your level is: {}", office[index]);
        } else if job == "academic" {
            println!("Your level is: {}", academic[index]);
        } else if job == "lawyer" {
            println!("Your level is: {}", lawyer[index]);
        } else if job == "teacher" {
            println!("Your level is: {}", teacher[index]);
        } else {
            println!("No job like this");
        }
        println!("Level: {}", levels[index]);

    } 

    else {
        println!("Fail");

        //god abeg
    }
}