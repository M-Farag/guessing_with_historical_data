pub mod game;

use game::Guess;
use std::io;

fn main() {
    'app_main_loop: loop {
        let mut g_1 = Guess::start();

        println!("Please guess the number");
        'app_game_runtime_loop: loop {
            println!("ur guess _>");
            let mut user_input:String = String::new();
            io::stdin().read_line(&mut user_input).expect("Err reading your input");
            let user_input:usize = match user_input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue
            };

            let results = g_1.check(user_input);
            
            if results.0 {
                println!("{}",results.1);

                println!("You tried it for {:#?} time(s) & Your historical inputs are {:#?}",g_1.attempts_counter,g_1.historical_inputs);
                break 'app_game_runtime_loop;
            }
            println!("{}",results.1);
        }
        println!("Game restarted, Enjoy & try to get it under {} trie(s) ;) ",g_1.attempts_counter);
        g_1.reset();
    }
}
