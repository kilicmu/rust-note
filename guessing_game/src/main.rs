use std::io;
use std::cmp::Ordering;
use rand:: Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
        .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("win!");
                break;
            }
        }

        println!("you guessed: {}", guess);
    }   
}