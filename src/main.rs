use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
	let separator: String = "-".repeat(17);

	println!("{}\n[ Guessing Game ]\n{0}\n", separator);

	let target = rand::thread_rng().gen_range(1..=100);
	let mut tries = 1;

	loop {
		print!("[ {} ] Input your guess: ", tries);
		io::stdout().flush().expect("Failed to write line");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => match num {
				1..=100 => num,
				_ => {
					println!("Please enter a number between 1 and 100!\n{}", separator);
					continue;
				}
			},
			Err(_) => {
				println!("Please enter a positive integer!\n{}", separator);
				continue;
			}
		};

		match guess.cmp(&target) {
			Ordering::Less => println!("Your guess is too low!"),
			Ordering::Greater => println!("Your guess is too high!"),
			Ordering::Equal => {
				break println!(
					"\nYou win! The number was {}\nAttempts: {}\n",
					target, tries
				);
			}
		}

		println!("{}", separator);
		tries += 1;
	}
}
