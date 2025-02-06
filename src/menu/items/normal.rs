use engage::{
    dialog::yesno::{BasicDialogItemNo, BasicDialogItemYes, TwoChoiceDialogMethods, YesNoDialog}, 
    gameuserdata::GameUserData, menu::{BasicMenuItem, BasicMenuItemAttribute, BasicMenuItemMethods, BasicMenuResult, MenuSequence}, 
    mess::Mess
};
use unity::prelude::*;

use crate::{menu::change::DifficultyChangeSequence, util::{difficulty_to_string, update_difficulty}};


pub struct NormalMenuItem;

impl BasicMenuItemMethods for NormalMenuItem {
    extern "C" fn get_name(_this: &mut BasicMenuItem, _method_info: unity::prelude::OptionalMethod) -> &'static Il2CppString {
        Mess::get("MID_SYS_Difficulty_Normal")
    }

    extern "C" fn a_call(this: &'static mut BasicMenuItem, _method_info: OptionalMethod) -> BasicMenuResult {
        if !this.is_attribute_disable() {
            let current = difficulty_to_string(GameUserData::get_difficulty(false));
            let text = format!(
                "{}: {}\n\n{}: {}", 
                Mess::get("MID_Hub_Bed_Difficulty_Change_Before"), 
                current, 
                Mess::get("MID_Hub_Bed_Difficulty_Change_After"), 
                Mess::get("MID_SYS_Difficulty_Normal")
            );
            YesNoDialog::bind::<NormalConfirmDialog>(
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
        if GameUserData::get_difficulty(false) == 0 {
            BasicMenuItemAttribute::Disable
        } else {
            BasicMenuItemAttribute::Enable
        }
    }
}

pub struct NormalConfirmDialog;

impl TwoChoiceDialogMethods for NormalConfirmDialog {
    extern "C" fn on_first_choice(_this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        DifficultyChangeSequence::bind(*_this.fields.parent.parent.menu.proc.parent.as_ref().unwrap());

        update_difficulty(GameUserData::get_difficulty(false), 0);
        
        BasicMenuResult::se_decide().with_close_all(true)
    }

    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::se_decide().with_close_this(true)
    }
}
