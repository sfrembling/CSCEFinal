mod dialogue;
mod objectives;
mod shared;


fn main() {
    loop {
        match UserMode::get_user_mode() {
            UserMode::Dialogue => {
                um_dialogue();
            }
            UserMode::Objectives => {
                um_objectives();
            }
            UserMode::Quit => break,
        }
    }
}

fn um_dialogue() {
    let d = dialogue::Dialogue::new();

    print_bar();
    println!("\n{}\n", d);
    print_bar();
}

fn um_objectives() {
    let o = objectives::Objectives::new();

    print_bar();
    println!("\n{}\n", o);
    print_bar();
}

fn print_bar() {
    println!("{}", "-".repeat(30));
}

/// Just decides the mode of operation for the program
enum UserMode {
    Dialogue,
    Objectives,
    Quit,
}

impl UserMode {
    /// Prompts user for their desired mode and returns their entry, with error-handling
    fn get_user_mode() -> UserMode {
        use std::io::{self, Write};
        println!("Choose an option from below\n");
        println!("1. Create quest dialogue");
        println!("2. Create quest objectives");
        println!("3. Quit");
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
                3 => UserMode::Quit,
                _ => err_msg(),
            },
            Err(_) => err_msg(),
        }
    }
}
