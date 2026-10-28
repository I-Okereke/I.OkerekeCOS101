// define the struct
struct Laptop {
    brand:String,
    quantity:u32,
    price:u32
}

//logic to calculate price of each laptop
impl Laptop {
    fn price(&self) -> u32 {
        self.price * 3
    }
}

fn main() {
    //initialize a structure
    let lap1 = Laptop {
        brand: String::from("HP"),
        quantity:10,
        price: 650000
    };
    let lap2 = Laptop{
        brand:String::from("IBM"),
        quantity:6,
        price:755000
    };
    let lap3 = Laptop{
        brand:String::from("Toshiba"),
        quantity:10,
        price:550000
    };
    let lap4= Laptop{
        brand:String::from("Dell"),
        quantity:4,
        price:850000
    };


    println!("stock of {} laptops is {}",lap1.brand,lap1.quantity );
    println!("stock of {} laptops is {}",lap2.brand,lap2.quantity );
    println!("stock of {} laptops is {}",lap3.brand,lap3.quantity );
    println!("stock of {} laptops is {}\n\n\n",lap4.brand,lap4.quantity );




    let hp_cost = lap1.price();
    let ibm_cost = lap2.price();
    let toshiba_cost = lap3.price();
    let dell_cost = lap4.price();

    let total = hp_cost + ibm_cost + toshiba_cost + dell_cost;

    println!("Total price for getting 3 of each laptop is N{}",total );

}