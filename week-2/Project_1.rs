fn main() {
    let p: f64 = 520_000_000.00;
    let r: f64 = 10.0;
    let n: f64 = 5.0;

    let a = p * (1.0 + r / 100.0).powf(n);

    println!("Amount is {}", a);

    let ci = a - p;

    println!("The compound interest of N520,000,000 for 5 years at 10% per annum is {}", ci);
}   