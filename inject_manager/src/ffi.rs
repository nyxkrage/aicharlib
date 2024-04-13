use ffi::{cvec::CVec, FromFfi, IntoFfi};
use std::ffi::{c_char, c_int, CStr};

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InjectionEntry {
    pub keys: CVec<*mut c_char>,
    pub content: *mut c_char,
    pub extensions: extensions::ffi::Extensions,
    pub enabled: bool,
    pub insertion_order: i32,
    pub case_sensitive: bool,
    pub name: *mut c_char,
    pub priority: i32,
    pub id: i32,
    pub comment: *mut c_char,
    pub selective: bool,
    pub secondary_keys: CVec<*mut c_char>,
    pub constant: bool,
    pub position: *mut c_char,
}

impl FromFfi<crate::InjectionEntry> for InjectionEntry {
    fn from_ffi(self) -> crate::InjectionEntry {
        crate::InjectionEntry {
            keys: self.keys.from_ffi(),
            content: self.content.from_ffi(),
            extensions: self.extensions.from_ffi(),
            enabled: self.enabled,
            insertion_order: self.insertion_order,
            case_sensitive: self.case_sensitive,
            name: self.name.from_ffi(),
            priority: self.priority,
            id: self.id,
            comment: self.comment.from_ffi(),
            selective: self.selective,
            secondary_keys: self.secondary_keys.from_ffi(),
            constant: self.constant,
            position: self.position.from_ffi(),
        }
    }
}

impl IntoFfi<InjectionEntry> for crate::InjectionEntry {
    fn into_ffi(self) -> InjectionEntry {
        InjectionEntry {
            keys: self.keys.into_ffi(),
            content: self.content.into_ffi(),
            extensions: self.extensions.into_ffi(),
            enabled: self.enabled,
            insertion_order: self.insertion_order,
            case_sensitive: self.case_sensitive,
            name: self.name.into_ffi(),
            priority: self.priority,
            id: self.id,
            comment: self.comment.into_ffi(),
            selective: self.selective,
            secondary_keys: self.secondary_keys.into_ffi(),
            constant: self.constant,
            position: self.position.into_ffi(),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct InjectionManager {
    pub name: *mut c_char,
    pub description: *mut c_char,
    pub scan_depth: c_int,
    pub token_budget: c_int,
    pub recursive_scanning: bool,
    pub extensions: extensions::ffi::Extensions,
    pub entries: CVec<InjectionEntry>,
}

impl FromFfi<crate::InjectionManager> for InjectionManager {
    fn from_ffi(self) -> crate::InjectionManager {
        crate::InjectionManager {
            name: self.name.from_ffi(),
            description: self.description.from_ffi(),
            scan_depth: self.scan_depth,
            token_budget: self.token_budget,
            recursive_scanning: self.recursive_scanning,
            extensions: self.extensions.from_ffi(),
            entries: self.entries.from_ffi(),
        }
    }
}

impl IntoFfi<InjectionManager> for crate::InjectionManager {
    fn into_ffi(self) -> InjectionManager {
        InjectionManager {
            name: self.name.into_ffi(),
            description: self.description.into_ffi(),
            scan_depth: self.scan_depth,
            token_budget: self.token_budget,
            recursive_scanning: self.recursive_scanning,
            extensions: self.extensions.into_ffi(),
            entries: self.entries.into_ffi(),
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn inject_manager_parse_json(json_str: *const c_char) -> *mut InjectionManager {
    let injman_json = CStr::from_ptr(json_str).to_str().unwrap();
    let injman = serde_json::from_str::<super::InjectionManager>(injman_json).unwrap();
    let injman_ffi = Box::new(injman.into_ffi());
    Box::into_raw(injman_ffi)
}

#[no_mangle]
pub unsafe extern "C" fn inject_manager_free(injman: *mut InjectionManager) {
    if injman.is_null() {
        return;
    }
    Box::from_raw(injman).from_ffi();
}