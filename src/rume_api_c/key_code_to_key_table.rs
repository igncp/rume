use std::collections::HashSet;

use crate::rume::key_table::{RumeKeyModifier, RumeKeyTable};

pub fn get_key_table_from_key_code(key_code: u16) -> Option<RumeKeyTable> {
    match key_code {
        0x00 => Some(RumeKeyTable::LetterA),
        0x0B => Some(RumeKeyTable::LetterB),
        0x08 => Some(RumeKeyTable::LetterC),
        0x02 => Some(RumeKeyTable::LetterD),
        0x0E => Some(RumeKeyTable::LetterE),
        0x03 => Some(RumeKeyTable::LetterF),
        0x05 => Some(RumeKeyTable::LetterG),
        0x04 => Some(RumeKeyTable::LetterH),
        0x22 => Some(RumeKeyTable::LetterI),
        0x26 => Some(RumeKeyTable::LetterJ),
        0x28 => Some(RumeKeyTable::LetterK),
        0x25 => Some(RumeKeyTable::LetterL),
        0x2E => Some(RumeKeyTable::LetterM),
        0x2D => Some(RumeKeyTable::LetterN),
        0x1F => Some(RumeKeyTable::LetterO),
        0x23 => Some(RumeKeyTable::LetterP),
        0x0C => Some(RumeKeyTable::LetterQ),
        0x0F => Some(RumeKeyTable::LetterR),
        0x01 => Some(RumeKeyTable::LetterS),
        0x11 => Some(RumeKeyTable::LetterT),
        0x20 => Some(RumeKeyTable::LetterU),
        0x09 => Some(RumeKeyTable::LetterV),
        0x0D => Some(RumeKeyTable::LetterW),
        0x07 => Some(RumeKeyTable::LetterX),
        0x10 => Some(RumeKeyTable::LetterY),
        0x06 => Some(RumeKeyTable::LetterZ),
        0x2C => Some(RumeKeyTable::QuestionMarkDown),
        0x12 => Some(RumeKeyTable::ExclamationMarkDown),
        24 => Some(RumeKeyTable::Plus),
        27 => Some(RumeKeyTable::Minus),
        0x31 => Some(RumeKeyTable::Space),
        36 => Some(RumeKeyTable::Enter),
        51 => Some(RumeKeyTable::Backspace),
        53 => Some(RumeKeyTable::Escape),
        123 => Some(RumeKeyTable::ArrowLeft),
        124 => Some(RumeKeyTable::ArrowRight),
        125 => Some(RumeKeyTable::ArrowDown),
        126 => Some(RumeKeyTable::ArrowUp),
        41 => Some(RumeKeyTable::Colon),

        _ => None,
    }
}

const BITMASK_PAIRS: &[(u32, RumeKeyModifier)] = &[
    (1 << 0, RumeKeyModifier::Shift),
    (1 << 1, RumeKeyModifier::Lock),
    (1 << 2, RumeKeyModifier::Control),
    (1 << 3, RumeKeyModifier::Mod1),
    (1 << 4, RumeKeyModifier::Mod2),
    (1 << 5, RumeKeyModifier::Mod3),
    (1 << 6, RumeKeyModifier::Mod4),
    (1 << 7, RumeKeyModifier::Mod5),
    (1 << 8, RumeKeyModifier::Button1),
    (1 << 9, RumeKeyModifier::Button2),
    (1 << 10, RumeKeyModifier::Button3),
    (1 << 11, RumeKeyModifier::Button4),
    (1 << 12, RumeKeyModifier::Button5),
];
pub(super) fn extract_modifiers_from_flag(
    flag: u32,
) -> HashSet<crate::rume::key_table::RumeKeyModifier> {
    let mut modifiers: HashSet<RumeKeyModifier> = std::collections::HashSet::new();

    for (bitmask, modifier) in BITMASK_PAIRS.iter() {
        if flag & bitmask != 0 {
            modifiers.insert(modifier.clone());
        }
    }

    modifiers
}
