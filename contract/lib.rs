//! rock paper scissors game on the NEAR blockchain. 

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct RockPaperScissorsGame {
    primary_commit: String,
    secondary_commit: String,
    game_result: String
}

enum GamePhase {
    Idle
    FirstChoiceMade,
    SecondChoiceMade,
    Resolved
}

#[near_bindgen]
impl RockPaperScissorsGame{
    // 0 = rock, 1 = paper, 2 = scissors :)
    let stringChoices = ['rock', 'paper', 'scissors']

    pub fn new_game(&mut self){
        // new game is started by a player comitting to a value, and placing a bet

    }
    pub fn respond_game(&mut self){

    }
    fn resolve_game(&mut self){
        i8 firstPlayerChoice = 2
        i8 secondPlayerChoice = 2
        // TODO: get player commitments as a number

        String firstPlayerChoiceString = stringChoices[firstPlayerChoice]
        String secondPlayerChoiceString = stringChoices[secondPlayerChoice]

        env::log_str(format!("first player chose {}", firstPlayerChoiceString))
        env::log_str(format!("second player chose {}", secondPlayerChoiceString))

        i8 dif = firstPlayerChoice - secondPlayerChoice
        
        // if both numbers are the same - draw
        if dif == 0 {
            // TODO resolve_draw
        }
        // if both numbers are consecutive - biggest wins
        else if dif.abs() == 1 {
            if firstPlayerChoice > secondPlayerChoice {
                // TODO resolve_win p1
            }
            else if firstPlayerChoice < secondPlayerChoice {
                // TODO resolve_win p2
            }
        }
        // if both numbers are not consecutive - smallest one wins
        else if dif.abs() > 1 {
            if firstPlayerChoice < secondPlayerChoice {
                // TODO resolve_win p1
            }
            else if firstPlayerChoice > secondPlayerChoice {
                // TODO resolve_win p2
            }
        }
    }
}