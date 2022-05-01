mod dialogue;
mod objectives;
mod shared;

use std::io::{self, Write};
use std::fs::File;

fn main() {
    loop {
        match UserMode::get_user_mode() {
            UserMode::Dialogue => {
                um_dialogue();
            }
            UserMode::Objectives => {
                um_objectives();
            }
            UserMode::GenDialogue => {
                gm_dialogue();
            }
            UserMode::GenObjectives => {
                gm_objectives();
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

fn gm_dialogue() {
    print!("Enter the output file name: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();

    io::stdin().read_line(&mut filename).unwrap();

    print!("Enter the number of quests to generate: ");
    io::stdout().flush().unwrap();

    let mut num_quests = String::new();

    io::stdin().read_line(&mut num_quests).unwrap();

    let number = num_quests.trim().parse::<i32>().unwrap();

    let mut outfile = File::create(filename.trim()).unwrap();

    for _ in 0..number {
        let q = dialogue::Dialogue::new();
        write!(outfile, "{}\n\n", q).unwrap();
    }
}

fn gm_objectives() {
    print!("Enter the output file name: ");
    io::stdout().flush().unwrap();
    let mut filename = String::new();

    io::stdin().read_line(&mut filename).unwrap();

    print!("Enter the number of quests to generate: ");
    io::stdout().flush().unwrap();

    let mut num_quests = String::new();

    io::stdin().read_line(&mut num_quests).unwrap();

    let number = num_quests.trim().parse::<i32>().unwrap();

    let mut outfile = File::create(filename.trim()).unwrap();

    for _ in 0..number {
        let q = objectives::Objectives::new();
        write!(outfile, "{}\n\n", q).unwrap();
    }
}

fn print_bar() {
    println!("{}", "-".repeat(30));
}

/// Just decides the mode of operation for the program
enum UserMode {
    Dialogue,
    Objectives,
    GenDialogue,
    GenObjectives,
    Quit,
}

impl UserMode {
    /// Prompts user for their desired mode and returns their entry, with error-handling
    fn get_user_mode() -> UserMode {
        println!("Choose an option from below\n");
        println!("1. Create quest dialogue");
        println!("2. Create quest objectives");
        println!("3. Generate dialogue files");
        println!("4. Generate objective files");
        println!("5. Quit");
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
                3 => UserMode::GenDialogue,
                4 => UserMode::GenObjectives,
                5 => UserMode::Quit,
                _ => err_msg(),
            },
            Err(_) => err_msg(),
        }
    }
}
