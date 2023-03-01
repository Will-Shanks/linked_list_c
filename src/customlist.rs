use crate::llitem::LlItem;
use crate::innerlist::InnerList;


///List type with custom drop implementation
///The various list types are likely to become an enum in a later version
pub struct CustomList<'a, T: LlItem> {
    pub(crate) list: InnerList<'a, T>
}

impl<T: LlItem> CustomList<'_, T> {
    /// Create a list from a provided raw pointer
    /// # Safety
    /// The provided drop method should be able to safely drop the entire list 
    /// when passed a pointer to the first element in the list
    pub unsafe fn from(first: *mut T, drop: fn(*mut T)) -> CustomList<'static, T> {
        CustomList{list: InnerList::new(first, drop)}
    }
    /// Create a new empty list
    /// # Safety
    /// The provided drop method should be able to safely drop the entire list 
    /// when passed a pointer to the first element in the list
    pub unsafe fn new(drop: fn(*mut T)) -> CustomList<'static, T> {
        CustomList{list: InnerList::new(core::ptr::null_mut(), drop)}
    }
    /// Create a list from a provided raw pointer
    /// # Safety
    /// The provided drop method should be able to safely drop each element in the list
    /// 1 element at a time, starting with the first
    pub unsafe fn drop_each(first: *mut T, drop: fn(*mut T)) -> CustomList<'static, T> {
        CustomList{list: InnerList::new(first, drop)}
    }
    pub fn add(&mut self, elem: *mut T) {
        self.list.add(elem);
    }
    pub fn combine(&mut self, list: CustomList<T>) {
        self.list.combine(list.list);
    }
    pub fn head(&self) -> *mut T {
        self.list.head()
    }
}

impl<'a, T: LlItem> Iterator for CustomList<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.next()
    }
}
