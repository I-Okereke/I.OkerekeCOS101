use std::io::Write;

fn main() {
    let name = vec!["Oluchi Mordi","Adams Aliyu","Shania Bolade","Adekunle Gold","Blanca Edemoh",];
    let matric = vec!["ACC10211111","ECO10110101","CSC10328828","EEE11020202","MEE10202001",];
    let dept = vec!["Accounting","Economics","Computer","Electrical","Mechanical",];
    let level = vec!["300", "100", "200", "200", "100"];


    let mut file =std::fs::File::create("Students info.txt").expect("Create failed");



    println!("Name: {}", name[0]);
    println!("Matric number: {}", matric[0]);
    println!("Department: {}", dept[0]);
    println!("Level: {}\n", level[0]);

    println!("Name: {}", name[1]);
    println!("Matric number: {}", matric[1]);
    println!("Department: {}", dept[1]);
    println!("Level: {}\n", level[1]);

    println!("Name: {}", name[2]);
    println!("Matric number: {}", matric[2]);
    println!("Department: {}", dept[2]);
    println!("Level: {}\n", level[2]);

    println!("Name: {}", name[3]);
    println!("Matric number: {}", matric[3]);
    println!("Department: {}", dept[3]);
    println!("Level: {}\n", level[3]);

    println!("Name: {}", name[4]);
    println!("Matric number: {}", matric[4]);
    println!("Department: {}", dept[4]);
    println!("Level: {}\n", level[4]);



    let info1 = format!(
        "Name: {}\n Matric number: {}\n Department: {}\n Level: {}\n",name[0], matric[0], dept[0], level[0]
    );

    let info2 = format!(
        "Name: {}\n Matric number: {}\n Department: {}\n Level: {}\n",name[1], matric[1], dept[1], level[1]
    );

    let info3 = format!(
        "Name: {}\n Matric number: {}\n Department: {}\n Level: {}\n",name[2], matric[2], dept[2], level[2]
    );

    let info4 = format!(
        "Name: {}\n Matric number: {}\n Department: {}\n Level: {}\n",name[3], matric[3], dept[3], level[3]
    );

    let info5 = format!(
        "Name: {}\n Matric number: {}\n Department: {}\n Level: {}\n",name[4], matric[4], dept[4], level[4]
    );



    file.write_all(info1.as_bytes()).expect("Write failed");
    file.write_all(info2.as_bytes()).expect("Write failed");
    file.write_all(info3.as_bytes()).expect("Write failed");
    file.write_all(info4.as_bytes()).expect("Write failed");
    file.write_all(info5.as_bytes()).expect("Write failed");




}
