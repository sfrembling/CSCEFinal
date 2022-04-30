use std::{io::{BufRead, BufReader}, fs::File};

use rand::prelude::IteratorRandom;

pub struct Quest {
    data: String
}

impl std::fmt::Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Quest {
    pub fn new() -> Quest {
        Quest { data: Quest::generate() }
    }

    pub fn premade() -> Quest {
        let premade_file = File::open("data/premade").unwrap();
        let quest = BufReader::new(premade_file).lines().choose(&mut rand::thread_rng());
        Quest { data: quest.unwrap().unwrap() }
    }

    fn generate() -> String {
        String::new()
    }
    
}