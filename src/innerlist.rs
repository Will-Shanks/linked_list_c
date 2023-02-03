use core::ptr;
use core::marker::PhantomData;
use log::trace;


use crate::llitem::LlItem;

enum DropType<T: LlItem> {
    First(fn(*mut T)),
    Each(fn(*mut T)),
}

pub struct InnerList<'a, T: LlItem> {
    head: *mut T,
    current: *mut T, //current elem for iter tracking
    drop_method: DropType<T>,
    _phantom: PhantomData<&'a mut T>
}


unsafe impl<T: LlItem+Send> Send for InnerList<'_, T>{}
unsafe impl<T: LlItem+Sync> Sync for InnerList<'_, T>{}

impl<T: LlItem> InnerList<'_, T> {
    /// create a new linked list
    /// drop is called by the lists's Drop trait, with the first element in the list
    /// drop is responsible for cleaning up the list
    /// be carefull to not double free/ leak memory/ etc when using this method
    pub unsafe fn new(first: *mut T, drop_method: fn(*mut T)) -> InnerList<'static, T> {
        trace!("Creating new list with custom drop");
        InnerList::create(first, DropType::First(drop_method))
    }

    unsafe fn create(first: *mut T, drop: DropType<T>) -> InnerList<'static, T> {
        InnerList{head:first, current: ptr::null_mut(), drop_method: drop, _phantom: PhantomData}
    }

    /// create a new linked list
    /// similar to new, but drop is called on each element in the list in order, from first to last
    /// drop is responsible for cleaning up the list
    /// be carefull to not double free/ leak memory/ etc when using this method
    pub unsafe fn drop_each(first: *mut T, drop: fn(*mut T)) -> InnerList<'static, T> {
        trace!("Creating new list with custom drop");
        InnerList::create(first, DropType::Each(drop))
    }

    /// Add a list of elements to the front of the InnerList
    /// if trying to add two lists together see the InnerList<T>.combine(InnerList<T>) method
    /// which does memory managment correctly
    pub fn add(&mut self, elem: *mut T) {
        let oldhead = self.head;
        if !self.head.is_null() {
            let mut last = elem;
            while !unsafe{(*last).get_next()}.is_null() {
                last = unsafe{(*last).get_next()};
            }
            unsafe{(*last).set_next(self.head)};
        }
        self.head = elem;
        trace!("InnerList head {:?} Added elemenet(s), new head {:?}", &oldhead, &self.head);
    }

    /// Combine two InnerLists into one, handling memory correctly
    /// a naive approach, using InnerList<T>.add(InnerList<T>) causes the added list to be freed twice
    /// once when its original list is dropped, and a seccond when the list it was added to is dropped
    /// The caller is responsible for ensuring the drop method for the first list is acceptable for the elements of the second combining InnerLists with different drop_first/drop_each functions is undefined behaviour
    pub fn combine(&mut self, elems: InnerList<T>) {
        self.add(elems.head());
        std::mem::forget(elems);
    }

    /// Return a pointer to the head of the linked list
    /// This is the pointer a C FFI expects when it takes a linked list
    pub fn head(&self) -> *mut T {
        trace!("Returning list head {:?}", &self.head);
        self.head
    }
}

impl<'a, T: LlItem> Drop for InnerList<'a, T>{
    fn drop(&mut self) {
        trace!("InnerList Head {:?} Dropping list", &self.head);
        trace!("InnerList Head {:?} dropping head", &self.head);
        if let DropType::First(drop) = self.drop_method {
            if !self.head.is_null() {
                drop(self.head);
            }
        } else if let DropType::Each(drop) = self.drop_method {
            let mut next = self.head();
            while !next.is_null() {
                let tmp = next;
                next = unsafe{(*next).get_next()};
                trace!("Dropping {:?}, next is {:?}", &tmp, &next);
                drop(tmp);
            } 
        }
    }
}

impl<'a, T: LlItem> Iterator for InnerList<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
        // if list is empty nothing to return
            trace!("InnerList empty");
            None
        } else if self.current.is_null() {
            //at start of list, return head
            trace!("InnerList head {:?} At end of list, reseting and returning head", &self.head);
            //reset previous
            self.current = self.head;
            Some(unsafe{&*self.current})
        } else {
            //in the middle of the list, iterate
            self.current = unsafe{(*self.current).get_next()};
            trace!("InnerList head {:?} returning next element {:?}", &self.head, &self.current);
            if self.current.is_null() {
                None
            } else {
                Some(unsafe{&*self.current})
            }
        }
    }
}
