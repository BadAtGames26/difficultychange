use std::vec;
use engage::{
    menu::{content::shoptopmenu, BasicMenu, BasicMenuItem, BasicMenuMethods, MenuSequence}, 
    mess::Mess, 
    proc::{desc::ProcDesc, Bindable, ProcBoolMethod, ProcInst, ProcVoidMethod}, 
    resourcemanager, 
    titlebar::TitleBar
};
use unity::{prelude::*, system::List};

use crate::menu::items::{hard::HardMenuItem, lunatic::LunaticMenuItem, normal::NormalMenuItem};

pub struct DifficultyMenuSequence;

#[repr(i32)]
enum DifficultyMenuSequenceLabel {
    Start = 1,
    Exit,
}

impl MenuSequence for DifficultyMenuSequence {
    fn get_proc_desc(this: &'static ProcInst) -> Vec<&'static mut ProcDesc> {
        vec![
            ProcDesc::label(DifficultyMenuSequenceLabel::Start as _),
            ProcDesc::call(ProcVoidMethod::new(this, Self::load_resources)),
            ProcDesc::wait_while_true(ProcBoolMethod::new(this, Self::is_loading_resources)),
            ProcDesc::call(ProcVoidMethod::new(this, Self::create_menu_bind)),
            ProcDesc::label(DifficultyMenuSequenceLabel::Exit as _),
            ProcDesc::call(ProcVoidMethod::new(this, Self::exit)),
            ProcDesc::end(),
        ]
    }

    fn proc_name() -> &'static str {
        "DifficultyMenuSequence"
    }
}

impl DifficultyMenuSequence {
    pub extern "C" fn create_menu_bind(parent: &mut impl Bindable, _method_info: OptionalMethod) {
        println!("DifficultyMenuSequence::CreateMenuBind");

        // Shop Top Menu is used for these kind of menus, the Recollection Menu uses this as well.
        let menu_content: &mut engage::menu::BasicMenuContent = unsafe { shoptopmenu::shop_top_menu_content_create(None) }.unwrap();

        // Create a List<BasicMenuItem> for the BasicMenu
        // SystemList is used because otherwise infer errors happen, idk
        let menu_item_list_class = get_generic_class!(SystemList<BasicMenuItem>).unwrap();
        let menu_item_list = il2cpp::instantiate_class::<List<BasicMenuItem>>(menu_item_list_class).unwrap();

        // Create a item list with a capacity of 3
        menu_item_list.items = Il2CppArray::new(3).unwrap(); 

        // Adding the menu items, build_attributes is used to disable buttons when we don't want them selectable, can also hide them entirely
        let normal_menu_item = BasicMenuItem::new_impl::<NormalMenuItem>();
        menu_item_list.add(normal_menu_item);

        let hard_menu_item = BasicMenuItem::new_impl::<HardMenuItem>();
        menu_item_list.add(hard_menu_item);
        
        let luantic_menu_item = BasicMenuItem::new_impl::<LunaticMenuItem>();
        menu_item_list.add(luantic_menu_item);

        // Create a BasicMenu and fill it with our item list
        let basic_menu = BasicMenu::new(menu_item_list, menu_content);
        let descs = basic_menu.create_default_desc();
        
        basic_menu.create_bind(parent, descs, "DifficultyMenu");
        basic_menu.bind_parent_menu();
        
        TitleBar::open_header(
            Mess::get("MID_Hub_Bed_Difficulty_Change").to_string(), 
            Mess::get("MID_Hub_MyRoom_Bed_TitleHelp").to_string(), 
            ""
        );   
    }

    pub extern "C" fn load_resources(_parent: &mut impl Bindable, _method_info: OptionalMethod) {
        // Load the prefab for the Shop UI
        unsafe { shoptopmenu::shop_top_menu_content_load_prefab_async(None) };
    }

    pub extern "C" fn is_loading_resources(_parent: &mut impl Bindable, _method_info: OptionalMethod) -> bool {
        // This method will be pooled continuously until all the resources are loaded
        unsafe { resourcemanager::is_loading(None) }
    }

    pub extern "C" fn exit(_parent: &mut impl Bindable, _method_info: OptionalMethod) {
        unsafe {
            // Unload the prefab for the Shop UI
            shoptopmenu::shop_top_menu_content_unload_prefab(None);
        }  
        // Open a header with the previous menu content since it won't reappear
        TitleBar::open_header(
            Mess::get("MID_Hub_MyRoom_Bed").to_string(), 
            Mess::get("MID_Hub_MyRoom_Bed_TitleHelp").to_string(), 
            ""
        );  
    }
}

#[repr(C)]
#[unity::class("System.Collections.Generic", "List`1")]
pub struct SystemList {}