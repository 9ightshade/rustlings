// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// fn main() {
//     println!("The system welcomes player");
//     println!("Select a game to play");
//     guess_number();
// }

// fn guess_number() {
//     let mut tries: i32 = 7;
//     println!(
//         "Guess number between 1 and 50, you have {tries} tries Goodluck you going to need it..."
//     );

//     let secret_number: u32 = rand::thread_rng().gen_range(1..=50); //generate secret number
//     loop {
//         if tries != 0 {
//             let mut guess: String = String::new();

//             io::stdin()
//                 .read_line(&mut guess)
//                 .expect("fail to read line, user input");

//             let guess: u32 = match guess.trim().parse() {
//                 Ok(num) => num,
//                 Err(_) => {
//                     println!("Please enter a number");
//                     continue;
//                 }
//             };

//             match guess.cmp(&secret_number) {
//                 Ordering::Less => println!("Too low!"),
//                 Ordering::Equal => {
//                     println!("You win!");
//                     println!("Secret number is {secret_number}");
//                     break;
//                 }
//                 Ordering::Greater => println!("Too high!"),
//             }
//         } else {
//             println!("no more tries, better luck next time..")
//         }
//         tries -= 1;
//         println!("tries left {tries}");
//     }
// }
