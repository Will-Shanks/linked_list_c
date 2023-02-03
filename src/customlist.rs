use crate::llitem::LlItem;
use crate::innerlist::InnerList;


pub struct CustomList<'a, T: LlItem> {
    pub(crate) list: InnerList<'a, T>
}

impl<T: LlItem> CustomList<'_, T> {
    pub unsafe fn from(first: *mut T, drop: fn(*mut T)) -> CustomList<'static, T> {
        CustomList{list: InnerList::new(first, drop)}
    }
    pub unsafe fn new(drop: fn(*mut T)) -> CustomList<'static, T> {
        CustomList{list: InnerList::new(core::ptr::null_mut(), drop)}
    }
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
