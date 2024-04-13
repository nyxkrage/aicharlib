#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct CVec<T> {
    pub len: usize,
    pub cap: usize,
    pub data: *mut T,
}

impl<T> From<Vec<T>> for CVec<T> {
    fn from(mut value: Vec<T>) -> Self {
        let len = value.len();
        let cap = value.capacity();
        let data = value.as_mut_ptr();
        std::mem::forget(value);
        CVec {
            len,
            cap,
            data,
        }
    }
}

impl<T> Into<Vec<T>> for CVec<T> {
    fn into(self) -> Vec<T> {
        unsafe {
            let v = Vec::from_raw_parts(self.data, self.len, self.cap);
            std::mem::forget(self);
            v
        }
    }
}

impl<T> IntoIterator for CVec<T> {
    type Item = T;
    
    type IntoIter = std::vec::IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }

}
    
impl<T> FromIterator<T> for CVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let vec = Vec::from_iter(iter);
        CVec::from(vec)
    }
}

impl<T> CVec<T> {
    pub fn to_vec(self) -> Vec<T> {
        self.into()
    }
}
