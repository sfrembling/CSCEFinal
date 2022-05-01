use rand::Rng;

use super::shared::choose_random_from_filename;

pub struct Objectives {
    obj_list: Vec<String>,
    result: String
}

impl std::fmt::Display for Objectives {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.result)
    }
}

impl Objectives {
    pub fn new() -> Objectives {
        let action = choose_random_from_filename("data/objectives/action");
        
        let commands = action.split(";").map(|s| s.to_string()).collect::<Vec<String>>();

        let mut o = Objectives { obj_list: commands, result: String::new() };

        o.generate();

        o
    }

    fn generate(&mut self) {
        let mut obj_builder: Vec<String> = Vec::new();
        let mut index = 1;
        for command in &self.obj_list {
            let parsed = command.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
            if parsed[0].starts_with("GOTO") {
                let location = choose_random_from_filename("data/dialogue/location");

                obj_builder.push(format!("{}. Go to {}", index, location));

                index += 1;
                continue;
            }
            if parsed[0].starts_with("KILLN") {
                let entity = choose_random_from_filename("data/dialogue/entity");

                let index1 = parsed[1].find("{").unwrap() + 1;

                let index2 = parsed[1].find("}").unwrap();

                let mut n = parsed[1].as_str()[index1..index2].to_owned();

                if &n == "r" {
                    n = rand::thread_rng().gen_range(5..30).to_string();
                }

                obj_builder.push(format!("{}. Kill {} {}s", index, n, entity));

                index += 1;
                continue;
            }
            if parsed[0].starts_with("KILLT") {
                let entity = choose_random_from_filename("data/dialogue/entity");

                obj_builder.push(format!("{}. Kill the {}", index, entity));

                index += 1;
                continue;
            }
            if parsed[0].starts_with("LOOTN") {
                let item = choose_random_from_filename("data/dialogue/item");

                let index1 = parsed[1].find("{").unwrap() + 1;

                let index2 = parsed[1].find("}").unwrap();

                let mut n = parsed[1].as_str()[index1..index2].to_owned();

                if &n == "r" {
                    n = rand::thread_rng().gen_range(5..30).to_string();
                }

                obj_builder.push(format!("{}. Collect {} {}s", index, n, item));
                index += 1;
                continue;
            }
            if parsed[0].starts_with("LOOT") {
                let item = choose_random_from_filename("data/dialogue/item");

                obj_builder.push(format!("{}. Take the {}", index, item));
                index += 1;
                continue;
            }
            if parsed[0].starts_with("RET") {
                obj_builder.push(format!("{}. Return here", index));
                index += 1;
                continue;
            }
            if parsed[0].starts_with("FINDN") {
                let item = choose_random_from_filename("data/dialogue/item");

                let index1 = parsed[1].find("{").unwrap() + 1;

                let index2 = parsed[1].find("}").unwrap();

                let mut n = parsed[1].as_str()[index1..index2].to_owned();

                if &n == "r" {
                    n = rand::thread_rng().gen_range(5..30).to_string();
                }

                obj_builder.push(format!("{}. Find {} {}s", index, n, item));
                index += 1;
                continue;
            }
            if parsed[0].starts_with("FIND") {
                let item = choose_random_from_filename("data/dialogue/item");

                obj_builder.push(format!("{}. Find a {}", index, item));
                index += 1;
                continue;
            }
            if parsed[0].starts_with("GUARD") {
                obj_builder.push(format!("{}. Guard the area", index));
                index += 1;
                continue;
            }
        }
        self.result = obj_builder.join("\n").to_string();
    }
}