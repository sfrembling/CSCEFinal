use std::io::Write;

fn main() {
    println!("This program will present you with a 'quest' as if you were playing a video game.");
    println!("There is a 50/50 chance that it was either written by hand or procedurally generated.");
    println!("Choose wisely as to which one you think it is.\n\n");
    main_loop();
}

/// Where the code is primarily being executed from
fn main_loop() {
    loop {
        print!("Press enter to get a new quest, or 'quit' to leave: ");
        // flush message to the stream
        std::io::stdout().flush().unwrap();
        // see if they entered "quit"
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // if they have, leave the program
        if input.trim().to_ascii_lowercase() == "quit" {
            println!("Goodbye!");
            std::process::exit(0);
        }
    }
}