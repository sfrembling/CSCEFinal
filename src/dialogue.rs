use super::shared::choose_random_from_filename;

pub struct Dialogue {
    text: String,
}

impl std::fmt::Display for Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Dialogue {
    /// Get a new Dialogue object with a random action
    pub fn new() -> Dialogue {
        let action = choose_random_from_filename("data/dialogue/action");

        let mut d = Dialogue { text: action };

        d.generate();

        d
    }

    /// Generate the dialogue given a fresh action string
    /// 
    /// Subsequent calls to this method shouldn't change anything
    pub fn generate(&mut self) {
        // Replace each instance of a location
        loop {
            let s = self.text.clone();

            let location = choose_random_from_filename("data/dialogue/location");

            self.text = self.text.replacen("{location}", &location, 1);

            if s == *self.text {
                break;
            }
        }
        // Replace each instance of an entity
        loop {
            let s = self.text.clone();

            let entity = choose_random_from_filename("data/dialogue/entity");

            self.text = self.text.replacen("{entity}", &entity, 1);

            if s == *self.text {
                break;
            }
        }
        // Replace each instance of an item
        loop {
            let s = self.text.clone();

            let item = choose_random_from_filename("data/dialogue/item");

            self.text = self.text.replacen("{item}", &item, 1);

            if s == *self.text {
                break;
            }
        }
    }
}



