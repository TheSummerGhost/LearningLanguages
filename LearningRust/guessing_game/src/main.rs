use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    pose_question();
    let secret_number = generate_random_number();
    println!("The random number is {secret_number}");
    loop {
        let guess = get_user_input();
        println!("Your guess was {guess}");

        let answer = compare_numbers(guess, secret_number);

        if answer == 2 {
            break;
        }
    }
    //compare_numbers(guess, secret_number);
}

fn pose_question() {
    println!("Please input your guess");
}

fn get_user_input() -> u32 {
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
    }
}

fn generate_random_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    return secret_number;
}

fn compare_numbers(guess: u32, gen_number: u32) -> u32 {
    let answer: u32;
    match guess.cmp(&gen_number) {
        Ordering::Less => {
            println!("Too low");
            answer = 1;
        }
        Ordering::Equal => {
            println!("You got it!");
            answer = 2;
        }

        Ordering::Greater => {
            println!("Too high");
            answer = 3;
        }
    }
    return answer;
}
