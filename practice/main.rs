use std::io;
fn main(){
	println!("Enter your age :");
	let mut age=String::new();
	let mut input=io::stdin();
	input.read_line(&mut age);
	
	println!("Your age is {}",age);
}