use engage::{menu::{BasicMenu, BasicMenuFields}, mess::Mess};
use unity::prelude::*;

use crate::sequences::myroomdiff::MyRoomDifficultySequence;

// Updates the difficulty with MyRoomDifficultySequence.ChangeDifficulty
pub fn update_difficulty(current: i32, next: i32) {
    let sequence = MyRoomDifficultySequence::instantiate().unwrap();
    // Changing the difficulties to what we want
    sequence.current_difficulty = current;
    sequence.next_difficulty = next;
    
    MyRoomDifficultySequence::change_difficulty(sequence);
}

// Converting the int to a localized string so we can make text for the dialogs
pub fn difficulty_to_string(difficulty: i32) -> String {
    match difficulty {
        0 => Mess::get("MID_SYS_Difficulty_Normal"),
        1 => Mess::get("MID_SYS_Difficulty_Hard"),
        2 => Mess::get("MID_SYS_Difficulty_Lunatic"),
        _ => Mess::get("MID_SYS_None")
    }.to_string()
}

