use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number game");
    let sec_num = rand::thread_rng().gen_range(1..=100);
    println!("the sec_number : {sec_num}");
    loop {
        println!("input a number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read");
        println!("u gess:{guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("too semall"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("u win");
                break;
            }
        }
    }
}
