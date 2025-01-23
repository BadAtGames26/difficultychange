#![feature(ptr_sub_ptr)]

use skyline::patching::Patch;
use crate::sequences::myroom;

mod menu;
mod sequences;
mod util;


#[skyline::main(name = "difficultychange")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "Difficulty plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            42069,
            "Difficulty plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    // Patching the ChangeDifficulty function of MyRoomDifficultySequence to remove the dialog created at the end
    Patch::in_text(0x0238de84).nop().unwrap();
    skyline::install_hook!(myroom::myroom_create_desc);
}
