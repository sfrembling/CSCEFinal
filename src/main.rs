mod quest;

use rand;
use std::{
    io::{self, Write},
    process,
};

fn main() {
    println!("This program will present you with a 'quest' as if you were playing a video game.");
    println!(
        "There is a 50/50 chance that it was either written by hand or procedurally generated."
    );
    println!("Choose wisely as to which one you think it is.\n\n");
    main_loop();
}

/// Where the code is primarily being executed from
fn main_loop() {
    loop {
        print!("Press enter to get a new quest, or 'quit' to leave: ");
        // flush message to the stream
        io::stdout().flush().unwrap();
        // see if they entered "quit"
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // if they have, leave the program
        if input.trim().to_ascii_lowercase() == "quit" {
            println!("Goodbye!");
            process::exit(0);
        }

        // If this is true, then generate a new quest. Otherwise, use a premade quest.
        let chosen_quest = if rand::random() {
            quest::Quest::new() // generate a new quest
        } else {
            quest::Quest::premade() // use a premade quest
        };

        println!("Your quest is: {}", chosen_quest);
    }
}
