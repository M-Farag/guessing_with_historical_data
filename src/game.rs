use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Guess {
    pub machine_code:usize,
    pub attempts_counter:usize,
    pub historical_inputs:Vec<usize>
}

impl Guess {
   pub fn start() -> Self 
    {
        Self { 
            machine_code: rand::thread_rng().gen_range(1..=100), 
            attempts_counter: 0, 
            historical_inputs: vec![] 
        }
    }

    pub fn check(&self,user_input:usize) -> (bool, &str)
    {
        let results = match user_input.cmp(&self.machine_code) {
          Ordering::Equal => {
            (true,"You win")
          },
          Ordering::Less => {
            (false,"Your guess is too low, please try again")
          },
          Ordering::Greater => {
            (false,"Your guess is too high, please try again")
          }  
        };

        results
    }


}
