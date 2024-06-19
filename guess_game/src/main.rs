use colored::Colorize;
use rand::Rng;
use std::io;

fn main() {
    loop {
        println!("Enter the number");
        println!("enter the input guess");

        let mut guess = String::new();

        let secret_number = rand::thread_rng().gen_range(1..101);

        println!("secret_number {}", secret_number);

        io::stdin().read_line(&mut guess).expect("Fail to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your guess {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{}", "Less than guess".red()),
            std::cmp::Ordering::Equal => {
                println!("{}", "Equal to guess".green());
                break;
            }
            std::cmp::Ordering::Greater => println!("{}", "Greater than guess".red()),
        }
    }
}
