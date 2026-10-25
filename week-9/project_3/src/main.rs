
use std::fs::OpenOptions;
use std::io::Write;



fn main() {
    
let commisionner = vec!["Aigbogun Alama Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let com =format!("{:?}",commisionner );
        let mut file = std::fs::File::create("commisionners.txt").expect("Create failed");
            file.write_all(com.as_bytes()).expect("Write failed");



let ministry = vec!["Internal Affairs","Justice","Defense","Power and Steel","Petroleum"];
    let min=format!("{:?}",ministry);
        let mut file2 = std::fs::File::create("Ministry.txt").expect("Create failed");
        file2.write_all(min.as_bytes()).expect("Write failed");



let zone = vec!["South West","North East","South South","South West","South East"];
    let zones = format!("{:?}",zone);
        let mut file3 = std::fs::File::create("Zone.txt").expect("Failed to cratee");
            file3.write_all(zones.as_bytes()).expect("Failed to write");



let _file4 = std::fs::File::create("Merge.txt").expect("Failed to create");


let mut file5 = OpenOptions::new().append(true).open("Merge.txt").expect("Cannot open file");
file5.write_all(min.as_bytes()).expect("Failed to write");
file5.write_all(zones.as_bytes()).expect("Failed to write");
file5.write_all(com.as_bytes()).expect("Failed to write");


}
