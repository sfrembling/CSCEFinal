use std::io::{BufRead, BufReader};

use std::fs::File;

use rand::Rng;

use super::dialogue::Dialogue;
use super::shared::choose_random_from_filename;

pub struct Objectives {
    obj_list: Vec<String>,
    result: String,
}

impl std::fmt::Display for Objectives {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.result)
    }
}

impl Objectives {
    pub fn new() -> Objectives {
        let action = choose_random_from_filename("data/objectives/action");

        let commands = action
            .split(";")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let mut o = Objectives {
            obj_list: commands,
            result: String::new(),
        };

        o.generate();

        o
    }

    fn generate(&mut self) {
        let mut obj_builder: Vec<String> = Vec::new();
        let mut index = 1;

        // first, try to open the corresponding file

        
        for command in &self.obj_list {
            let command_file = File::open("data/objectives/command_dict").unwrap();
            if command == "" {
                break;
            }
            for line in BufReader::new(&command_file).lines() {
                match line {
                    Ok(s) => {
                        if s.starts_with(command) {
                            let mut s = s
                                .split(" # ")
                                .collect::<Vec<&str>>()
                                .iter()
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>();
                            if s[0].contains("{}") {
                                // it allows for a number
                                let n = rand::thread_rng().gen_range(2..=50).to_string();
                                s[1] = s[1].replacen("{}", &n, 1);
                            }
                            obj_builder.push(format!("{}. {}", index, Dialogue::from(&s[1])));
                            index += 1;
                            break;
                        }
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }

        self.result = obj_builder.join("\n").to_string();
    }
}
