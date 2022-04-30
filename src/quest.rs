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
        Quest { data: quest_data }
    }

    fn generate() -> String {
        // first, acquire a random action
        let action_file = File::open("data/actions").unwrap();
        let mut action = BufReader::new(action_file)
            .lines()
            .choose(&mut rand::thread_rng())
            .unwrap()
            .unwrap();
        // parse it to replace each instance of a place, thing, or person
        Quest::parse_action(&mut action);
        // return the action
        action
    }

    fn parse_action(action: &mut String) {
        // replace all instances of "thing"
        loop {
            let start = action.clone();
            let thing_file = File::open("data/things").unwrap();
            let thing = BufReader::new(thing_file)
                .lines()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .unwrap();
            *action = action.replacen("{thing}", &thing, 1);
            if start == *action {
                break;
            }
        }
        // replace all instances of "person"
        loop {
            let start = action.clone();
            let person_file = File::open("data/people").unwrap();
            let person = BufReader::new(person_file)
                .lines()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .unwrap();
            *action = action.replacen("{person}", &person, 1);
            if start == *action {
                break;
            }
        }
        // replace all instances of "place"
        loop {
            let start = action.clone();
            let place_file = File::open("data/places").unwrap();
            let place = BufReader::new(place_file)
                .lines()
                .choose(&mut rand::thread_rng())
                .unwrap()
                .unwrap();
            *action = action.replacen("{place}", &place, 1);
            if start == *action {
                break;
            }
        }
    }
}
