use rand::Rng;

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

    

}
