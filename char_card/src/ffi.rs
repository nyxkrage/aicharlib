use ffi::cvec::CVec;
use inject_manager;
use extensions;
use std::ffi::{c_char, c_long, CStr};
use ffi::FromFfi;
use ffi::IntoFfi;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Character {
    pub name: *mut c_char,
    pub description: *mut c_char,
    pub personality: *mut c_char,
    pub scenario: *mut c_char,
    pub first_mes: *mut c_char,
    pub mes_example: *mut c_char,
    pub creator_notes: *mut c_char,
    pub system_prompt: *mut c_char,
    pub post_history_instructions: *mut c_char,
    pub alternate_greetings: CVec<*mut c_char>,
    pub character_book: inject_manager::ffi::InjectionManager,
    pub tags: CVec<*mut c_char>,
    pub creator: *mut c_char,
    pub character_version: *mut c_char,
    pub extensions: extensions::ffi::Extensions,
}

impl Character {
    pub fn to_ffi(char: super::Character) -> Self {
        Character {
            name: char.name.into_ffi(),
            description: char.description.into_ffi(),
            personality: char.personality.into_ffi(),
            scenario: char.scenario.into_ffi(),
            first_mes: char.first_mes.into_ffi(),
            mes_example: char.mes_example.into_ffi(),
            creator_notes: char.creator_notes.into_ffi(),
            system_prompt: char.system_prompt.into_ffi(),
            post_history_instructions: char.post_history_instructions.into_ffi(),
            alternate_greetings: char.alternate_greetings.into_ffi(),
            character_book: char.character_book.into_ffi(),
            character_version: char.character_version.into_ffi(), 
            tags: char
                .tags.into_ffi(),
            creator: char.creator.into_ffi(),
            extensions: char.extensions.into_ffi()
        }
    }

    pub unsafe fn from_ffi(self) -> super::Character {
        super::Character {
            name: self.name.from_ffi(),
            description: self.description.from_ffi(),
            personality: self.personality.from_ffi(),
            scenario: self.scenario.from_ffi(),
            first_mes: self.first_mes.from_ffi(),
            mes_example: self.mes_example.from_ffi(),
            creator_notes: self.creator_notes.from_ffi(),
            system_prompt: self.system_prompt.from_ffi(),
            post_history_instructions: self.post_history_instructions.from_ffi(),
            alternate_greetings: self.alternate_greetings.from_ffi(),
            character_book: self.character_book.from_ffi(),
            tags: self.tags.from_ffi(),
            creator: self.creator.from_ffi(),
            character_version: self.character_version.from_ffi(), 
            extensions: self.extensions.from_ffi()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn char_card_parse_png(png: *const u8, png_len: c_long) -> *mut Character {
    let data = unsafe { std::slice::from_raw_parts(png, png_len as usize) };
    let char = super::Character::from_png(data).unwrap();
    let char_ffi = Box::new(Character::to_ffi(char));
    Box::into_raw(char_ffi)
}

#[no_mangle]
pub unsafe extern "C" fn char_card_parse_json(json_str: *const c_char) -> *mut Character {
    let char_json = CStr::from_ptr(json_str).to_str().unwrap();
    let char = super::Character::from_json(char_json).unwrap();
    let char_ffi = Box::new(Character::to_ffi(char));
    Box::into_raw(char_ffi)
}

#[no_mangle]
pub unsafe extern "C" fn char_card_free(char: *mut Character) {
    if char.is_null() {
        return;
    }
    Box::from_raw(char).from_ffi();
}
