use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {

		println!("Input your guess: ");
		let mut guess = String::new();


		stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse(){
			Ok(num) => num,
			Err(_) => {println!("That is not a number"); continue;},
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Equal => {println!("Correct!"); break;},
			Ordering::Greater => println!("Too big!"),
		}
	}

}
