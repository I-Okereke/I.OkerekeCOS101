use std::io;


fn main() {
//input a,b and c

let mut a = String::new();
let mut b = String::new();
let mut c = String::new();

println!("Enter your first value");
io::stdin().read_line(&mut a).expect("Not a valid number");
let a:f64 = a.trim().parse().expect("Not a valid number");

println!("Enter your Second value");
io::stdin().read_line(&mut b).expect("Not a valid number");
let b:f64 = b.trim().parse().expect("Not a valid number");

println!("Enter your third value");
io::stdin().read_line(&mut c).expect("Not a valid number");
let c:f64 = c.trim().parse().expect("Not a valid number");

// calculating d
let d:f64 = b*b - 4.0*a*c;

//eqn for when d >0 is (-b +- root b^2 - 4ac)/(2a) there are two roots

if d > 0.0 {
    let root1 = (-b + d.sqrt()) / (2.0 * a);
    let root2 = (-b - d.sqrt()) / (2.0 * a);
    println!("Roots of equation are {} aand {}",root1, root2 );
}


//eqn for when d=0 is -b/2a, there is only one root

else if d == 0.0 {
    let root = -b / (2.0 * a);
    println!("The root of the equation is {}", root);

}

//for when r<0 theres no root
else{
    println!("No root exists");
}



}
