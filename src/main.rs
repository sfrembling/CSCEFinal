mod quest;

use rand;
use std::io::{self, Write};

fn main() {
    println!("This program will present you with a 'quest' as if you were playing a video game.");
    println!(
        "There is a 50/50 chance that it was either written by hand or procedurally generated."
    );
    println!("Enter 1 if its procedural, 2 if its real, or 3 to quit");
    println!("Choose wisely as to which one you think it is.\n\n");
    main_loop();
}

/// Where the code is primarily being executed from
fn main_loop() {
    let mut total_quests = 0;
    let mut procedural_correct = 0;
    let mut real_correct = 0;
    let mut procedural_total = 0;
    let mut real_total = 0;
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

        print!("\nEnter (1, 2, or 3): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<i32>().unwrap();

        if input == 3 {
            break;
        }

        match cq_type {
            quest::QuestType::Procedural => {
                procedural_total += 1;
                match cq_type.matches(input) {
                    true => {
                        procedural_correct += 1;
                    }
                    false => {}
                }
            }
            quest::QuestType::Real => {
                real_total += 1;
                match cq_type.matches(input) {
                    true => {
                        real_correct += 1;
                    }
                    false => {}
                }
            }
        }

        total_quests += 1;
    }
    println!("\n\nTotal quests embarked: {}", total_quests);
    if procedural_total != 0 {
        println!(
            "For procedural quests, you got {:.2}% correct",
            (procedural_correct as f64 / procedural_total as f64) * 100.0
        );
    } else {
        println!("You haven't seen any procedural quests");
    }
    if real_total != 0 {
        println!(
            "For real quests, you got {:.2}% correct",
            (real_correct as f64 / real_total as f64) * 100.0
        );
    } else {
        println!("You haven't seen any real quests");
    }
    println!("\nThanks for using my program!");
}
