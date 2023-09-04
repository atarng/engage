use crate::proc::ProcInst;

#[repr(C)]
#[unity::class("App", "MainMenuSequence")]
pub struct MainMenuSequence {
    pub proc: ProcInst,
    is_resume: bool,
    is_loaded: bool,
    pub prev_sequence: i32,
    pub now_sequence: i32,
    pub next_sequence: i32,
}

impl AsRef<ProcInst> for MainMenuSequence {
    fn as_ref(&self) -> &ProcInst {
        &self.proc
    }
}

impl AsMut<ProcInst> for MainMenuSequence {
    fn as_mut(&mut self) -> &mut ProcInst {
        &mut self.proc
    }
}

#[unity::from_offset("App", "MainMenuSequence", "JumpToNextSequence")]
pub fn mainmenusequence_jumptonextsequence(this: *const u8);