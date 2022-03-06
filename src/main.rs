use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn get_num(x: u32) -> i32 {
    let ret_num = if x > 5 { 2 + 5 } else {1};
    ret_num
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);
    for number in (1..=4).rev() {
        println!("{}!", number);
    }

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("the value {}", get_num(guess));

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("ok");
                break;
            },
            Ordering::Greater => println!("big"),
        }
    }
}
