use std::io;

// function for trapezium
fn trapezium() {
    let mut height = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f64 = height.trim().parse().expect("Error");

    let mut base1 = String::new();
    println!("Enter base 1:");
    io::stdin().read_line(&mut base1).expect("Failed to read input");
    let base1: f64 = base1.trim().parse().expect("Error");

        let mut base2 = String::new();
        println!("Enter base 2:");
        io::stdin().read_line(&mut base2).expect("Failed to read input");
        let base2: f64 = base2.trim().parse().expect("Error");

    let area:f64 = (height / 2.0) * (base1 + base2);
    println!("Area of Trampezium is: {}", area);
}

// function for rhombus
fn rhombus() {
    let mut diagonal1 = String::new();
    println!("Enter diagonal 1:");
    io::stdin().read_line(&mut diagonal1).expect("Failed to read input");
    let diagonal1: f64 = diagonal1.trim().parse().expect("Error");

    let mut diagonal2 = String::new();
    println!("Enter diagonal 2:");
    io::stdin().read_line(&mut diagonal2).expect("Failed to read input");
    let diagonal2: f64 = diagonal2.trim().parse().expect("Error");

  let area:f64 = 0.5 * diagonal1 * diagonal2;
  println!("Area of Rhombus is: {}", area);
}

// function for parallelogram
fn parallelogram() {
    let mut base = String::new();
    println!("Enter base:");
    io::stdin().read_line(&mut base).expect("Failed to read input");
    let base: f64 = base.trim().parse().expect("Error");

    let mut altitude = String::new();
    println!("Enter altitude:");
    io::stdin().read_line(&mut altitude).expect("Failed to read input");
    let altitude: f64 = altitude.trim().parse().expect("Error");

    let area:f64 = base * altitude;
    println!("Area of Parallelogram is: {}", area);
}

// function for cube
fn cube() {
    let mut side = String::new();
    println!("Enter side:");
    io::stdin().read_line(&mut side).expect("Failed to read input");
    let side: f64 = side.trim().parse().expect("Error");

    let area:f64 = 6.0 * side * side;
    println!("Surface Area of Cube is: {}", area);
}

// function for cylinder
fn cylinder() {
    let mut radius = String::new();
    println!("Enter radius:");
    io::stdin().read_line(&mut radius).expect("Failed to read input");
    let radius: f64 = radius.trim().parse().expect("Error");

    let mut height = String::new();
    println!("Enter height:");
    io::stdin().read_line(&mut height).expect("Failed to read input");
    let height: f64 = height.trim().parse().expect("Error");

    let pi = 22.0 / 7.0;
    let volume = pi * radius * radius * height;
    println!("Volume of Cylinder is: {}", volume);
}

fn main() {
    println!("Choose which shape you want to calculate");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Surface Area");
    println!("5. Cylinder Volume");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice = choice.trim();

  if choice == "trapezium"  {
      trapezium();
  } else if choice == "rhombus" {
    rhombus();
  } 
      else if choice == "parallelogram" {
          parallelogram();
      } 
  else if choice == "cube"   {
      cube();
  } 
  else if choice == "cylinder"  {
    cylinder();
  } 
  else {
      println!("Eroor");
  }
}