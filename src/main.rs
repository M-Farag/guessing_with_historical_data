pub mod game;

use game::Guess;
fn main() {
    let mut g_1 = Guess::start();

    println!("Game 1: {:#?}",g_1);
}
