use std::io;

fn main(){
    println!("Enter  your Age! ");
    let mut numInput=String::new();
    let mut input=io::stdin();
    input.read_line(&mut numInput);
    let mut age:u32=numInput.trim().parse().expect("Invalid Input!");
    if(age<18){
      println!("Cannot Vote");
    }
    else{
      println!("Can Vote");
    }
}


/*


Extra string type object creted so that we can run parse() trim() methods onto it and extreact int type value from it.

Because in Rust all standard input is taken as string by default

This is why when our Rust program involves taking an Integer input we create a temporary numInput variable in the beginning of the program itself

Above Program Explained in Steps :

1. Created a temporary numInput first (read above why)
2. Created and stored io object into input variable
3. so that we can use object's overloaded read_line() method 
   which in takes our numInput variable
4. create our age variable and assign numInput value with chained methods trim and parse, 
   trim to exclude other characters and parse to extract int value

*/

use std::io;

fn main(){
	println!("Enter a number to Test it's Parity..");
	let mut numinput=String::new();
	let mut input=io::stdin();
	input.read_line(&mut numinput).expect("Could not fetch input");
	let mut num:u32= numinput.trim().parse().expect("Invalid Number");

	if num%2==0 {	println!("Number is even");	}
	else {	println!("Number is odd");	}


}

