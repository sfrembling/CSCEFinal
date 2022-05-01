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

    pub fn from(line: &str) -> Dialogue {
        let mut d = Dialogue {
            text: line.to_string(),
        };

        d.generate();

        d
    }

    /// Generate the dialogue given a fresh action string
    ///
    /// Subsequent calls to this method shouldn't change anything
    pub fn generate(&mut self) {
        loop {
            let s = self.text.clone();

            match self.text.find("{") {
                Some(_) => {}
                None => break,
            }

            let index1 = self.text.find("{").unwrap() + 1;

            match self.text.find("}") {
                Some(_) => {}
                None => break,
            }

            let index2 = self.text.find("}").unwrap();

            let tag = self.text.as_str()[index1..index2].to_owned();

            let result = choose_random_from_filename(&format!("data/dialogue/{}", tag));

            self.text = self.text.replacen(&format!("{{{}}}", tag), &result, 1);

            if s == *self.text {
                break;
            }
        }
    }
}
