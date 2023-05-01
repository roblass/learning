use std::io;

fn main() {
    let x = 5;
    println!("The value of x in the outer scope is {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    let x = "whoa, dude";
    println!("The new value of x in the outer is {x}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
	
	// this is not valid
	//println!("Tup is {tup.1}");
	let foo = tup.1;
	println!("Tup's middle element is {foo}");

	//lol, this does not do what i thought it would, this is not f-strings
	let (x, y, z) = tup;
	let tup_str = "({x}, {y}, {z})";

	println!("Tup is {tup_str}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    func2();
}

fn func2() {
    println!("Hello from func2!");
}
