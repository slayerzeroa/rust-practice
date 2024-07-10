use rand::Rng; // rand 크레이트를 import
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

    println!("Guess the Number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    // println!("Secret Number: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}




// fn main() {
//     println!("Guess the Number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     std::io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let guess: u32 = guess.trim().parse().expect("Please type a number!");

//     println!("You guessed: {}", guess);

//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {}", secret_number);

//     // 차이를 계산하여 출력
//     let difference = (secret_number as i32 - guess as i32).abs();
//     println!("The difference is: {}", difference);
// }
