use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use rand::prelude::IteratorRandom;

pub struct Quest {
    data: String,
}

impl std::fmt::Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl Quest {
    pub fn new() -> Quest {
        Quest {
            data: Quest::generate(),
        }
    }

    pub fn premade() -> Quest {
        let premade_file = File::open("data/premade").unwrap();
        let quest_data = BufReader::new(premade_file)
            .lines()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .unwrap();
        Quest {
            data: quest_data,
        }
    }

    fn generate() -> String {
        String::new()
    }
}
