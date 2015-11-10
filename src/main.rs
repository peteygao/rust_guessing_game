extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number from 1 to 100!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	loop {
		println!("Input a number: ");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.ok()
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				match guess.trim() {
					"quit" => {
						println!("You've lost! The secret number was {}", secret_number);
						break;
					},
					input => {
						println!("{} is not a valid guess, try again.", input);
						continue;
					}
				}
			},
		};

		match guess.cmp(&secret_number) {
			Ordering::Less 		=> println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal		=> {
				println!("You win!");
				break;
			},
		}
	}
}
