pub mod cvec;
use std::{collections::HashMap, ffi::c_char};

use cvec::CVec;

pub trait IntoFfi<T> {
    fn into_ffi(self) -> T;
}

pub trait FromFfi<T> {
    fn from_ffi(self) -> T;
}

impl<T, U> IntoFfi<CVec<T>> for Vec<U> where U: IntoFfi<T> {
    fn into_ffi(self) -> CVec<T> {
        self.into_iter().map(|v| U::into_ffi(v)).collect()
    }
}

impl<T, U> FromFfi<Vec<T>> for CVec<U>
where
    U: FromFfi<T>
{
    fn from_ffi(self) -> Vec<T> {
        self.into_iter().map(|v| v.from_ffi()).collect()
    }
}

impl FromFfi<String> for *mut c_char {
    fn from_ffi(self) -> String {
        unsafe {
            std::ffi::CString::from_raw(self)
                .into_string()
                .unwrap()
        }
    }
}

impl IntoFfi<*mut c_char> for String {
    fn into_ffi(self) -> *mut c_char {
        std::ffi::CString::new(self)
            .unwrap()
            .into_raw() as *mut c_char
    }
}

impl<K,V,T,U> IntoFfi<CVec<(T,U)>> for HashMap<K,V>
where
    K: IntoFfi<T>,
    V: IntoFfi<U>,
{
    fn into_ffi(self) -> CVec<(T,U)> {
        self.into_iter().map(|(k,v)| (k.into_ffi(), v.into_ffi())).collect()
    }
}

impl<K,V,T,U> FromFfi<HashMap<K,V>> for CVec<(T,U)>
where
    T: FromFfi<K>,
    U: FromFfi<V>,
    K: Eq + std::hash::Hash,
{
    fn from_ffi(self) -> HashMap<K,V> {
        self.into_iter().map(|(k,v)| (k.from_ffi(), v.from_ffi())).collect()
    }
}