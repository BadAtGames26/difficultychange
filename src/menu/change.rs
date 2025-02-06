use engage::{fade::Fade, menu::MenuSequence, proc::{desc::ProcDesc, ProcInst}};

pub struct DifficultyChangeSequence;

impl MenuSequence for DifficultyChangeSequence {
    fn get_proc_desc(_this: &'static ProcInst) -> Vec<&'static mut ProcDesc> {
        vec![
            Fade::black_out(0.5, 4),
            ProcDesc::wait_time(1.0),
            Fade::black_in(0.5, 4),
            ProcDesc::end()
        ]
    }

    fn proc_name() -> &'static str {
        "DifficultyChangeSequence"
    }
}