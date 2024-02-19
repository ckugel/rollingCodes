extern crate rand;

use seeded_random::{Random, Seed};

pub(crate) struct Server {
    rng: Random,   
    attempt: u64,
    past: Vec<u32>,
    offset: usize,
}

impl Server {
    pub fn new(seed: u64) -> Server {
        Server { 
            rng: Random::from_seed(Seed::unsafe_new(seed)), 
            attempt: 0, 
            past: Vec::new(), 
            offset: 0 
        }

    }

    /**
     * @return true if the transmission was successful. Will auto increment attempts
     * @return false, if not
     */
    pub fn check_transmission(&mut self, value: u128) -> bool {
        let attempts: u64 = (value >> 32) as u64;
        let actual_message: u32 = (value << 32 >> 32) as u32;

        println!("attempts: {}", attempts);
        println!("actual message: {}", actual_message);
        
        while self.attempt <= attempts {
                self.past.push(self.rng.u32());
                self.attempt+=1;
        }
        println!("{:?}", self.past);
        if self.past[attempts as usize - 1] == actual_message {
            // if this is the case we know for sure that all of the values in "past" before this attemp "attempts" are useless
            self.clear_past_below_attempt(attempts as usize);
            return true;
        }
        else {
            return false;
        }
    }

    fn clear_past_below_attempt(&mut self, attempt: usize) -> () {
        let start: usize = 0;
        let end: usize = attempt - self.offset; 
        let additional_offset: usize = end - start;

        self.past.drain(start..end + 1);

        self.offset = additional_offset;
    }

    
}