pub mod game;

use game::Guess;
use std::io;

fn main() {
    let mut g_1 = Guess::start();
    

    println!("Please guess the number");
    'app_main_loop: loop {
        println!("ur guess _>");
        let mut user_input:String = String::new();
        io::stdin().read_line(&mut user_input).expect("Err reading your input");
        let user_input:usize = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let results = g_1.check(user_input);
        println!("Results are: {}, {}",results.0,results.1);
    }
}
