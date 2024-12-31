use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut num = String::new();
        println!("Welcome to the Guessing Game!");
        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        println!("You guessed: {num}");
        let int_num: u32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match int_num.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "Number is equal. You win!".green());
                break;
            }
            Ordering::Greater => println!("{}", "Number is greater".red()),
            Ordering::Less => println!("{}", "Number is less".red()),
        }
    }
}
