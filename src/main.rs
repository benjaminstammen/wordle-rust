use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;

use crate::game::GameState;

mod game;

fn main() {
    // TODO: support more games - currently, only classic Wordle is supported.

    // let selections = &[
    //     "Classic",
    //     "Quordle",
    // ];

    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Welcome! Select a game.")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();

    //println!("Selection: {}", selection);

    let word: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the word you'd like someone else to guess")
        .interact_text()
        .unwrap();
    let mut grid = game::Grid::new(word.clone());

    let mut game_status = GameState::InProgress;
    while matches!(game_status, GameState::InProgress) {
        grid.print();
        println!("---");
        println!("known: {:?}", grid.known_chars);
        println!("close: {:?}", grid.close_chars);
        println!("unused: {:?}", grid.unused_chars);
        println!("---");

        let guess: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your guess")
            .validate_with({
                |input: &String| -> Result<(), &str> {
                    let expected = word.clone().chars().count();
                    let actual = input.chars().count();
                    if expected == actual {
                        Ok(())
                    } else {
                        Err("Incorrect argument length")
                    }
                }
            })
            .interact_text()
            .unwrap();

        game_status = grid.guess(&guess);
        print!("{}[2J", 27 as char);
    }

    // Final summary
    grid.print();
    match game_status {
        GameState::Lost => {
            println!("You lost! Word was {}.", grid.word)
        },
        GameState::Won => {
            println!("You won! Good job!")
        },
        _ => panic!("Exited game loop in non-final state")
    }
}
