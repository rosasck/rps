/** 
 * @file main.rs
 * @brief Rock, Paper, Scissors Game
 * @author Katie Stoltz
 * 
 */

use std::io;
use rand::Rng;

mod core;
use core::types::{RPSMove, RPSResult, RPSMoveError};
use core::helperfunctions::Compare;

fn main() {
    println!("Lets play Rock, Paper, Scissors! Best of 5 wins");

    loop {
        let mut player_wins = 0;
        let mut computer_wins = 0;
        let mut end_game = false;

        // Game Loop
        'rps_game: loop {
            let comp_move: RPSMove = rand::thread_rng().gen();

            println!("Please select (R)ock, (P)aper, or (S)cissors: \n(Q) to end the game");

            // Each Play Loop
            loop {
                let mut player_move = String::new();

                io::stdin()
                    .read_line(&mut player_move)
                    .expect("Couldn't read move");

                let player_move: Result<RPSMove, RPSMoveError> = player_move.trim().parse();

                let player_move = match player_move {
                    Ok(player_move_val) => {
                        println!("\nYour play: {}", player_move_val);
                        println!("Computer played: {}", comp_move);
                        player_move_val
                    }
                    Err(RPSMoveError::Unknown(s)) => {
                        match s.to_lowercase().as_str() {
                            "q" | "quit" => {
                                end_game = true;
                                break 'rps_game;
                            }
                            _ => {
                                println!("Oops! \"{}\" isn't an option, try again!\n", s);
                                continue;
                            }
                        }
                    }
                };

                // Play outcome
		println!("\nResults:");
                let result: RPSResult = player_move.compare(&comp_move);
                match result {
		    RPSResult::Tie(_) => println!("Tie...Nobody wins"),
                    RPSResult::Win(_) => {
                        player_wins += 1;
                        println!("{}: You won this round.", result);
                    }
                    RPSResult::Loss(_) => {
                        computer_wins += 1;
                        println!("{}: You lost this round.", result);
                    }
                }

                break;
            }

            // Game Outcome Check
            if player_wins == 3 {
                println!("Congratulations, You won the game!\n");
                break;
            } else if computer_wins == 3 {
                println!("Computer Wins!\n");
                break;
            } else {
                println!(
                    "\nCurrent Score: \nPlayer Wins: {} \nComputer Wins: {} \n",
                    player_wins, computer_wins
                );
		println!("-------------------------------------------------------");
            }
        }
	// Player Quit Game
	if end_game == true {
        	println!("Ending Game!\n");
		break;
    	}

        // Replay 
	println!("-------------------------------------------------------");
        println!("Do you want to play again? \n(Q) to end the game");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Couldn't read input");

        match input.trim().to_lowercase().as_str() {
            "q" | "quit" => {
                println!("Ending Game!\n");
                break;
            }
            _ => continue,
        }

	
    }
    
}
