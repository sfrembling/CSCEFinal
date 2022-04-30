mod quest;

use rand;
use std::io::{self, Write};

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
    let mut correct_guesses = 0;
    let mut total_quests = 0;
    loop {
        let cq_type: quest::QuestType;

        // If this is true, then generate a new quest. Otherwise, use a premade quest.
        let chosen_quest = if rand::random() {
            cq_type = quest::QuestType::Procedural;
            quest::Quest::new() // generate a new quest
        } else {
            cq_type = quest::QuestType::Real;
            quest::Quest::premade() // use a premade quest
        };

        println!("\nQuest ==> {}", chosen_quest);

        print!("\nEnter 1 if it is procedural, 2 if it is real, or 3 to quit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<i32>().unwrap();

        if input == 3 {
            break;
        }

        if cq_type.matches(input) {
            correct_guesses += 1;
        }

        total_quests += 1;
    }
    println!(
        "You guessed {} out of {} correctly.",
        correct_guesses, total_quests
    );
    println!("Thanks for using my program!");
}
