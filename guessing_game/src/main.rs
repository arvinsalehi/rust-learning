use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn main() {
//     let secret_number = rand::thread_rng().gen_range(1..=100);
//     println!("Guess the number");

//     loop {
//         println!("input the number bitch.");

//         let mut guess = String::new();
//         io::stdin()
//             .read_line(&mut guess)
//             .expect("Failed to read line");
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("Enter a number dumb bitch");
//                 continue;
//             }
//         };
//         println!("You guessed: {guess}");

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("You are short bitch"),
//             Ordering::Greater => println!("Only You're mothers ass is that big bitch"),
//             Ordering::Equal => {
//                 println!("Lucky bitch");
//                 break;
//             }
//         }
//     }
// }

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Guess a number");
    loop {
        let mut guess = String::from("");
    io::stdin()
        .read_line(&mut guess)
        .expect("Falied to read the line");

    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    println!("You Guesssed {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("to short"),
        Ordering::Equal => {
            println!("");
            break;
        },
        Ordering::Greater => println!("greater")
    }
    }
}   

