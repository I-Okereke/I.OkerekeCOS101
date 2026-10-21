use std::io::Write;


fn main() {
    println!("The categories of drinks Nigerian Breweries Plc make are in the file created:");


    let lager = "Lager:33 Export,Desperados,Goldberg,Gulder,Heineken,Star\n";
    let stout ="Stout:Legend,Turbo king,Williams\n";
    let alc = "Non-alcoholic drinks:Maltina,Amstel Malta,Maltina Gold,Fayrouz\n";

    let mut file = std::fs::File::create("Nigerian Breweries data.txt").expect("Create failed");

    //create the file
    file.write_all("The categories of drinks Nigerian Breweries Plc make are\n".as_bytes()).expect("write failed");

    //write the information to the file
    file.write_all(lager.as_bytes()).expect("Write failed");
    file.write_all(stout.as_bytes()).expect("Write failed");
    file.write_all(alc.as_bytes()).expect("Write failed");

    println!("File created successfully");





}
