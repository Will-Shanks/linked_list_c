use core::ptr;
#[cfg(feature="libc")]
use libc::c_void;
use core::marker::PhantomData;
use log::trace;

pub trait LlItem {
    fn get_next(&self) -> *mut Self;
    fn set_next(&mut self, next: *mut Self) -> Option<*mut Self>;
}

//helper to make throwing *mut T around easier
//probably not the best idea, but oh well
trait LlItemPtr {
    unsafe fn get_next(self) -> Self;
    unsafe fn set_next(self, next: Self) -> Option<Self> where Self: Sized;
}

impl<T: LlItem> LlItemPtr for *mut T {
    unsafe fn get_next(self) -> Self {
       (*self).get_next()
    }
    unsafe fn set_next(self, next: Self) -> Option<Self> {
        (*self).set_next(next)
    }
}

#[macro_export]
macro_rules! impl_LlItem {
    ([$($t:ty),+]) => {
        $(impl linked_list_c::LlItem for $t {
            fn get_next(&self) -> *mut Self {
                self.next
            }
            fn set_next(&mut self, next: *mut Self) -> Option<*mut Self> {
                let old = self.next;
                self.next = next;
                if !old.is_null() { 
                    Some(old)
                } else {
                    None
                }
            }
        })*
    }
}

pub struct List<'a, T: LlItem> {
    head: *mut T,
    current: *mut T, //current elem for iter tracking
    drop_first: Option<fn(*mut T)>,
    drop_each: Option<fn(*mut T)>,
    _phantom: PhantomData<&'a mut T>
}

impl<T: LlItem> List<'_, T> {
    //create a new linked list, drops rust "correctly" should not be used for elements whose Drop doesn't properly clean up (ex: things retrieved via an ffi)
    pub fn new() -> List<'static, T> {
        trace!("Creating new List");
        List{head: ptr::null_mut(), current: ptr::null_mut(), drop_first: None, drop_each: Some(|x: *mut T| unsafe{std::ptr::drop_in_place(x); std::alloc::dealloc(x as *mut u8, std::alloc::Layout::for_value(&*x));}), _phantom: PhantomData}
    }

    //create a new linked list, that uses libc::free() to drop elements. Good for lists of simple free-able elements obtained via FFIs
    #[cfg(feature="libc")]
    pub fn from_c(elem: *mut T) -> List<'static, T> {
        trace!("Creating new List from c list");
        List{head: elem, n: ptr::null_mut(), drop_first: None, drop_each: Some(|x: *mut T| unsafe{libc::free(x as *mut c_void)}), _phantom: PhantomData}
    }

    //create a new linked list, if drop_first is set, a pointer to the first element is passed to it (List.head()).
    //Otherwise if drop_each is set a pointer to each element (starting with the first) is passed to it
    // if both drop_first and drop_each are None then the elements are not cleaned up when the List is dropped
    // be carefull to not leak memory when using this method
    pub fn with_custom_drop(first: *mut T, drop_each: Option<fn(*mut T)>, drop_first: Option<fn(*mut T)>) -> List<'static, T> {
        trace!("Creating new list with custom drop");
        List{head: first, current: ptr::null_mut(), drop_first, drop_each, _phantom: PhantomData}
    }

    // Add a list of elements to the front of the List
    // if trying to add two lists together see the List<T>.combine(List<T>) method
    // which does memory managment correctly
    pub fn add(&mut self, elem: *mut T) {
        let oldhead = self.head;
        if !self.head.is_null() {
            let mut last = elem;
            while !unsafe{last.get_next()}.is_null() {
                last = unsafe{last.get_next()};
            }
            unsafe{last.set_next(self.head)};
        }
        self.head = elem;
        trace!("List head {:?} Added elemenet(s), new head {:?}", &oldhead, &self.head);
    }

    // Combine two Lists into one, handling memory correctly
    // a naive approach, using List<T>.add(List<T>) causes the added list to be freed twice
    // once when its original list is dropped, and a seccond when the list it was added to is dropped
    // The caller is responsible for ensuring the drop method for the first list is acceptable for the elements of the second combining Lists with different drop_first/drop_each functions is undefined behaviour
    pub fn combine(&mut self, elems: List<T>) {
        self.add(elems.head());
        std::mem::forget(elems);
    }

    pub fn head(&self) -> *mut T {
        trace!("Returning list head {:?}", &self.head);
        self.head
    }
}

impl<T, U> From<Vec<U>> for List<'static, T>
where
    T: LlItem,
    U: Into<T>,
 {
    fn from(elems: Vec<U>) -> List<'static, T> {
        trace!("Converting Vec to List");
        let mut l = List::new();
        //TODO figure out how to not need to box here
        for x in elems {
            //into_raw is crucial so elem isn't dropped
            l.add(Box::into_raw(Box::new(x.into())));
        }
        trace!("Vec converted to list with head {:?}", &l.head);
        l
    }
}

impl<'a, T: LlItem> Drop for List<'a, T>{
    fn drop(&mut self) {
        trace!("List Head {:?} Dropping list", &self.head);
        if let Some(d) = self.drop_first {
            trace!("List Head {:?} dropping head", &self.head);
            if !self.head.is_null() {
                d(self.head);
            }
        } else if let Some(d) = self.drop_each {
            let mut next = self.head;
            while !next.is_null() {
                let tmp = next;
                next = unsafe{next.get_next()};
                trace!("Dropping {:?}, next is {:?}", &tmp, &next);
                d(tmp);
            } 
        }
    }
}

impl<'a, T: LlItem> Iterator for List<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
        // if list is empty nothing to return
            trace!("List empty");
            None
        } else if self.current.is_null() {
            //at start of list, return head
            trace!("List head {:?} At end of list, reseting and returning head", &self.head);
            //reset previous
            self.current = self.head;
            Some(unsafe{&*self.current})
        } else {
            //in the middle of the list, iterate
            self.current = unsafe{self.current.get_next()};
            trace!("List head {:?} returning next element {:?}", &self.head, &self.current);
            if self.current.is_null() {
                None
            } else {
                Some(unsafe{&*self.current})
            }
        }
    }
}
