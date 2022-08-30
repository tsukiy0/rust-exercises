use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let answer = rand::thread_rng().gen_range(1..=100);

    println!("Answer is {answer}");

    println!("Guess the number!");

    loop {
        println!("Please input your guess:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("error: failed to read line");

        let guess: i32 = input.trim().parse().expect("error: input was not a number");

        println!("You guessed: {guess}");

        match guess.cmp(&answer) {
            Ordering::Equal => {
                println!("You win!");
                break
            },
            Ordering::Greater => println!("Too large!"),
            Ordering::Less => println!("Too small!"),
        }
    }
}
