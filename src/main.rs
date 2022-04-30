fn main() {
    loop {
        match UserMode::get_user_mode() {
            UserMode::Dialogue => {
                todo!(); // generate quest dialogue and display it
            }
            UserMode::Objectives => {
                todo!(); // generate quest objectives and display them
            }
        }
    }
}

/// Just decides the mode of operation for the program
enum UserMode {
    Dialogue,
    Objectives,
}

impl UserMode {
    /// Prompts user for their desired mode and returns their entry, with error-handling
    fn get_user_mode() -> UserMode {
        use std::io::{self, Write};
        println!("Choose an option from below\n");
        println!("1. Create quest dialogue");
        println!("2. Create quest objectives");
        print!("Enter: ");

        let mut input = String::new();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        input = input.trim().to_string();

        let err_msg = || {
            println!("\nError: Mode incorrect!\n");
            UserMode::get_user_mode()
        };

        match input.parse() {
            Ok(value) => match value {
                1 => UserMode::Dialogue,
                2 => UserMode::Objectives,
                _ => err_msg(),
            },
            Err(_) => err_msg(),
        }
    }
}
