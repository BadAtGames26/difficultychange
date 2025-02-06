use engage::proc::ProcInstFields;
use unity::prelude::*;

#[unity::class("App","MyRoomDifficultySequence")]
pub struct MyRoomDifficultySequence {
    pub proc: ProcInstFields,
    pub current_difficulty: i32,
    pub next_difficulty: i32,
}

impl MyRoomDifficultySequence {
    pub fn change_difficulty(sequence: &Self ) {
        unsafe {
            myds_change_difficulty(sequence, None);
        }
    }
}

#[unity::from_offset("App", "MyRoomDifficultySequence", "ChangeDifficulty")]
pub fn myds_change_difficulty(this: &MyRoomDifficultySequence, method_info: OptionalMethod);
