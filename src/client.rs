extern crate rand;

use seeded_random::{Random, Seed};

pub(crate) struct Client{
    attempt: u64,
    rng: Random,
}

impl Client{
    pub fn new(seed: u64) -> Client {
        Client {
            attempt: 0, 
            rng: Random::from_seed(Seed::unsafe_new(seed))
        }
    }

    pub fn get_transmission(&mut self) -> u128 {
        self.attempt += 1;
        return ((self.attempt as u128) << 32) + (self.rng.u32() as u128);
    }
}

