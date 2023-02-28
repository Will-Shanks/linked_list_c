use crate::innerlist::InnerList;
use crate::llitem::LlItem;
use crate::List;
use crate::CustomList;

pub struct ConstList<'a, T: LlItem> {
    list: InnerList<'a, T>
}

impl<T: LlItem> ConstList<'_, T> {
    pub fn head(&self) -> *mut T {
        self.list.head()
    }
    pub fn from(first: *mut T) -> ConstList<'static, T> {
        ConstList{list: unsafe{InnerList::drop_each(first,
            |x: *mut T| {std::ptr::drop_in_place(x); std::alloc::dealloc(x as *mut u8, std::alloc::Layout::for_value(&*x));})}
        }
    }
}

impl<'a, T: LlItem> Iterator for ConstList<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.next()
    }
}

impl<'a, T: LlItem> From<List<'a, T>> for ConstList<'a, T> {
    fn from(list: List<'a, T>) -> ConstList<'a, T> {
       ConstList{list: list.list} 
    }
}

impl<'a, T: LlItem> From<CustomList<'a, T>> for ConstList<'a, T> {
    fn from(list: CustomList<'a, T>) -> ConstList<'a, T> {
       ConstList{list: list.list} 
    }
}
