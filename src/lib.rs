use core::ptr;
#[cfg(feature="libc")]
use libc::c_void;
use core::marker::PhantomData;
use log::trace;

pub trait LlItem {
    fn get_next(&self) -> *mut Self;
    fn set_next(&mut self, next: *mut Self);
}

#[macro_export]
macro_rules! impl_LlItem {
    ([$($t:ty),+]) => {
        $(impl linked_list_c::LlItem for $t {
            fn get_next(&self) -> *mut Self {
                self.next
            }
            fn set_next(&mut self, next: *mut $t) {
                let old = self.next;
                if !old.is_null() && !next.is_null() { 
                    unsafe{(*next).set_next(old)};
                }
                self.next = next;
            }
        })*
    }
}

pub struct List<'a, T: LlItem> {
    head: *mut T,
    n: *mut T,
    drop_first: Option<fn(*mut T)>,
    drop_each: Option<fn(*mut T)>,
    _phantom: PhantomData<&'a mut T>
}

impl<T: LlItem> List<'_, T> {
    pub fn new() -> List<'static, T> {
        trace!("Creating new List");
        List{head: ptr::null_mut(), n: ptr::null_mut(), drop_first: None, drop_each: Some(|x: *mut T| unsafe{std::ptr::drop_in_place(x)}), _phantom: PhantomData}
    }

    #[cfg(feature="libc")]
    pub fn from_c(elem: *mut T) -> List<'static, T> {
        trace!("Creating new List from c list");
        List{head: elem, n: ptr::null_mut(), drop_first: None, drop_each: Some(|x: *mut T| unsafe{libc::free(x as *mut c_void)}), _phantom: PhantomData}
    }

    pub fn with_custom_drop(first: *mut T, drop_each: Option<fn(*mut T)>, drop_first: Option<fn(*mut T)>) -> List<'static, T> {
        trace!("Creating new list with custom drop");
        List{head: first, n: ptr::null_mut(), drop_first, drop_each, _phantom: PhantomData}
    }

    pub fn add(&mut self, mut elem: Box<T>) {
        if !self.head.is_null() {
            elem.set_next(self.head);
        }
        let oldhead = self.head;
        //into_raw is crucial so elem isn't dropped
        self.head = Box::into_raw(elem);
        trace!("List head {:?} Added elemenet, new head {:?}", &oldhead, &self.head);
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
            l.add(Box::new(x.into()));
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
                next = unsafe{(*next).get_next()};
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
            trace!("List empty");
            None
        } else if self.n.is_null() {
            trace!("List head {:?} At end of list, reseting and returning head", &self.head);
            self.n = self.head;
            Some(unsafe{&*self.n})
        } else if unsafe{& *self.n}.get_next().is_null() {
            trace!("List head {:?} At end of list", &self.head);
            self.n = ptr::null_mut();
            None
        } else {
            self.n = unsafe{& *self.n}.get_next();
            trace!("List head {:?} returning next element {:?}", &self.head, &self.n);
            Some(unsafe{&*self.n})
        }
    }
}
