use std::io;

fn main(){
	let mut ageinput=String::new();
	println!("Enter your age:");
	let mut input=io::stdin();
	input.read_line(&mut ageinput).expect("Failed to provide the input");
	let mut age:u32 =ageinput.trim().parse().expect("Invalid Number!");

	if age>=18 {
		println!("Congrats! you are eligible to vote");
	}
	else {
		println!("Sorry Kid, You'll have to wait..");
	}
	
}