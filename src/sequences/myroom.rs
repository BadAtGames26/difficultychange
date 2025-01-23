use engage::{menu::MenuSequence, proc::{desc::ProcDesc, Bindable, ProcInstFields, ProcVoidMethod}};
use unity::prelude::*;

use crate::menu::menu::DifficultyMenuSequence;

#[unity::class("App","MyRoomSequence")]
pub struct MyRoomSequence {
    pub proc: ProcInstFields,
}

impl Bindable for MyRoomSequence {}

#[unity::hook("App","MyRoomSequence","CreateDesc")]
pub fn myroom_create_desc(this: &'static MyRoomSequence, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc> {
    let desc = call_original!(this, method_info);
    desc[43] = ProcDesc::call(ProcVoidMethod::new(this, create_difficulty_menu));
    desc
}

pub extern "C" fn create_difficulty_menu(sequence: &'static mut MyRoomSequence, _method_info: OptionalMethod) {    
    DifficultyMenuSequence::bind(sequence);
}