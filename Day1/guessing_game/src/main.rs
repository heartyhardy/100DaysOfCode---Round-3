use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

    println!();
    println!(" 🎮 Guessing a number game!");
    println!("___________________________");

    let secret_num:u32 = rand::thread_rng().gen_range(1..101);

    loop{

        println!();
        println!("🔷 Please input your guess: (1-100)");
        println!("___________________________________");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("⛔ Failed to read your input!");
        
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("⛔ Please input a number!");
                continue;
            },
        };

        println!();
        println!("🔷 You guessed {}", guess);
        println!();

        match guess.cmp(&secret_num){
            Ordering::Less => println!(" 🔴 Too Small!"),
            Ordering::Greater => println!(" 🔴 Too Big!"),
            Ordering::Equal => {
                println!("🏆 You guessed it! Congrats! 🏆");
                println!("_______________________________");
                println!();
                break;
            }
        }
    }
}
