use engage::{
    dialog::yesno::{BasicDialogItemNo, BasicDialogItemYes, TwoChoiceDialogMethods, YesNoDialog}, 
    gameuserdata::GameUserData, menu::{BasicMenuItem, BasicMenuItemAttribute, BasicMenuItemMethods, BasicMenuResult, MenuSequence}, 
    mess::Mess
};
use unity::prelude::*;

use crate::{menu::change::DifficultyChangeSequence, util::{difficulty_to_string, update_difficulty}};


pub struct HardMenuItem;

impl BasicMenuItemMethods for HardMenuItem {
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: unity::prelude::OptionalMethod) -> &'static Il2CppString {
        Mess::get("MID_SYS_Difficulty_Hard")
    }

    extern "C" fn a_call(this: &'static mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() {
            let current = difficulty_to_string(GameUserData::get_difficulty(false));
            let text = format!(
                "{}: {}\n\n{}: {}", 
                Mess::get("MID_Hub_Bed_Difficulty_Change_Before"), 
                current, Mess::get("MID_Hub_Bed_Difficulty_Change_After"), 
                Mess::get("MID_SYS_Difficulty_Hard")
            );
            YesNoDialog::bind::<HardConfirmDialog>(
                this.menu, 
                text, 
                Mess::get("MID_Hub_Bed_Difficulty_Change_Yes").to_string(), 
                Mess::get("MID_KEYHELP_MENU_RETURN").to_string()
            );
            
            BasicMenuResult::se_decide()
        } else {
            BasicMenuResult::se_miss()
        }
    }

    extern "C" fn build_attributes(_this: &mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuItemAttribute {
        if GameUserData::get_difficulty(false) == 1 {
            BasicMenuItemAttribute::Disable
        } else {
            BasicMenuItemAttribute::Enable
        }
    }
}

pub struct HardConfirmDialog;

impl TwoChoiceDialogMethods for HardConfirmDialog {
    extern "C" fn on_first_choice(_this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        DifficultyChangeSequence::bind(*_this.fields.parent.parent.menu.proc.parent.as_ref().unwrap());

        update_difficulty(GameUserData::get_difficulty(false), 1);
        
        BasicMenuResult::se_decide().with_close_all(true)
    }

    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::se_decide().with_close_this(true)
    }
}
