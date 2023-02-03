use crate::innerlist::InnerList;
use crate::llitem::LlItem;

use log::trace;

pub struct List<'a, T: LlItem> {
    pub(crate) list: InnerList<'a, T>
}

impl<T: LlItem> List<'_, T> {
    pub fn from(first: *mut T) -> List<'static, T> {
        List{list: unsafe{InnerList::drop_each(first,
            |x: *mut T| {std::ptr::drop_in_place(x); std::alloc::dealloc(x as *mut u8, std::alloc::Layout::for_value(&*x));})}
    }}
    pub fn new() -> List<'static, T> {
        List{list: unsafe{InnerList::drop_each(core::ptr::null_mut(),
            |x: *mut T| {std::ptr::drop_in_place(x); std::alloc::dealloc(x as *mut u8, std::alloc::Layout::for_value(&*x));})}}
    }
    pub fn add(&mut self, elem: *mut T) {
        self.list.add(elem);
    }
    pub fn combine(&mut self, list: List<T>) {
        self.list.combine(list.list);
    }
    pub fn head(&self) -> *mut T {
        self.list.head()
    }
}

impl<'a, T: LlItem> Iterator for List<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.next()
    }
}

impl<T, U> From<Vec<U>> for List<'static, T>
where
    T: LlItem,
    U: Into<T>,
 {
    fn from(elems: Vec<U>) -> List<'static, T> {
        trace!("Converting Vec to InnerList");
        let mut l = List::new();
            //TODO figure out how to not need to copy into a Box
        for x in elems {
            //into_raw is crucial so elem isn't dropped
            l.add(Box::into_raw(Box::new(x.into())));
        }
        trace!("Vec converted to list with head {:?}", &l.head());
        l
    }
}


