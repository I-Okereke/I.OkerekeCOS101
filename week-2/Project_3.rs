fn main() {
	let tv: f64 = 210_000.00;
	let time: f64 = 3.0;
	let rate: f64 = 5.0;

	let value = tv * (1.0 - (rate / 100.0)).powf(time);


	println!("The value of the book after 3 years is {}",value);
}