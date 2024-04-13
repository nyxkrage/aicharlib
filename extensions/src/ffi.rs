use ffi::{cvec::CVec, FromFfi, IntoFfi};
use std::ffi::{c_char, c_double, c_void, CString};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(C)]
pub enum ExtTag {
    String = 0,
    Number = 1,
    Array = 2,
    Object = 3,
    Boolean = 4,
    Null = 5,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union ExtUnion {
    string: *mut c_char,
    number: c_double,
    array: CVec<ExtValue>,
    object: CVec<(*mut c_char, ExtValue)>,
    boolean: bool,
    null: *const c_void,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExtValue {
    tag: ExtTag,
    value: ExtUnion,
}

impl IntoFfi<ExtValue> for serde_json::Value {
    fn into_ffi(self) -> ExtValue {
        match self {
            serde_json::Value::Null => ExtValue {
                tag: ExtTag::Null,
                value: ExtUnion {
                    null: std::ptr::null(),
                },
            },
            serde_json::Value::Bool(b) => ExtValue {
                tag: ExtTag::Boolean,
                value: ExtUnion { boolean: b },
            },
            serde_json::Value::Number(n) => ExtValue {
                tag: ExtTag::Number,
                value: ExtUnion {
                    number: n.as_f64().unwrap(),
                },
            },
            serde_json::Value::String(s) => ExtValue {
                tag: ExtTag::String,
                value: ExtUnion {
                    string: CString::new(s).unwrap().into_raw(),
                },
            },
            serde_json::Value::Array(a) => ExtValue {
                tag: ExtTag::Array,
                value: ExtUnion {
                    array: a
                        .into_iter()
                        .map(|v| v.into_ffi())
                        .collect::<CVec<_>>(),
                },
            },
            serde_json::Value::Object(o) => ExtValue {
                tag: ExtTag::Object,
                value: ExtUnion {
                    object: o
                        .into_iter()
                        .map(|(k, v)| (CString::new(k).unwrap().into_raw(), v.into_ffi()))
                        .collect::<CVec<_>>(),
                },
            },
        }
    }
}

impl FromFfi<serde_json::Value> for ExtValue {
    fn from_ffi(self) -> serde_json::Value {
        unsafe {
            match self.tag {
                ExtTag::String => serde_json::Value::String(
                    CString::from_raw(self.value.string)
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                ExtTag::Number => serde_json::Value::Number(
                    serde_json::Number::from_f64(self.value.number).unwrap(),
                ),
                ExtTag::Array => serde_json::Value::Array(
                    self.value.array.into_iter().map(|e| e.from_ffi()).collect(),
                ),
                ExtTag::Object => serde_json::Value::Object(
                    self.value
                        .object
                        .into_iter()
                        .map(|(k, v)| {
                            (CString::from_raw(k).to_str().unwrap().to_string(), v.from_ffi())
                        })
                        .collect(),
                ),
                ExtTag::Boolean => serde_json::Value::Bool(self.value.boolean),
                ExtTag::Null => serde_json::Value::Null,
            }
        }
    }
}

pub type Extensions = CVec<(*mut c_char, ExtValue)>;