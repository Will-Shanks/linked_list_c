use std::ptr;
use libc::c_void;


//to be compatable with C linked list returned next __must__ be a *mut Element

pub struct Element {
    next: *mut Element,
    val: String
}

impl Element {
    pub fn new(v: String) -> Box<Element> {
        let n = ptr::null::<Element>() as *mut Element;
        Box::new(Element{val: v, next: n }) 
    }
    fn set_next(&mut self, next: *mut Element) {
        let old = self.next;
        if !old.is_null() && !next.is_null() { 
            unsafe{(*next).set_next(old)};
        }
        self.next = next;
    }
    fn get_next(&self) -> *mut Element {
        self.next
    }
}


pub struct List {
    head: *mut Element,
    next: *mut Element,
    drop_each: Option<fn(*mut Element)>,
    drop_first: Option<fn(*mut Element)>
}

impl List {
    pub fn new() -> List {
        List{head: ptr::null::<Element>() as *mut Element, next: ptr::null::<Element>() as *mut Element, drop_each: Some(|x: *mut Element| unsafe{std::ptr::drop_in_place(x)}), drop_first: None}
    }

    pub fn from_c(elem: *mut Element) -> List {
        List{head: elem, next: ptr::null::<Element>() as *mut Element, drop_each: Some(|x: *mut Element| unsafe{libc::free(x as *mut c_void)}), drop_first: None}
    }

    pub fn with_custom_drop(first: Option<*mut Element>, drop_each: Option<fn(*mut Element)>, drop_first: Option<fn(*mut Element)>) -> List {
        List{head: first.unwrap_or(ptr::null::<Element>() as *mut Element), next: ptr::null::<Element>() as *mut Element, drop_each: drop_each, drop_first: drop_first}
    }

    pub fn add(&mut self, mut elem: Box<Element>) {
        if !self.head.is_null() {
            elem.set_next(self.head);
        }
        //into_raw is crucial so elem isn't dropped
        self.head = Box::into_raw(elem);
    }
}

impl Drop for List{
    fn drop(&mut self) {
        if let Some(d) = self.drop_first {
            d(self.head);
        } else if let Some(d) = self.drop_each {
            while let Some(elem) = self.next() {
                d(elem);
            }
        }
    }
}

impl Iterator for List {
    type Item = *mut Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head.is_null() {
            None
        } else if self.next.is_null() {
            self.next = self.head;
            Some(self.next)
        } else if unsafe{& *self.next}.get_next().is_null() {
            self.next = ptr::null::<Element>() as *mut Element;
            None
        } else { self.next = unsafe{& *self.next}.get_next();
        Some(self.next)
        }
    }
}

#[test]
fn empty_list() {
    let mut list = List::new();
    assert_eq!(list.next(), None);
}

#[test]
fn non_empty_list() {
    let mut list = List::new();
    let first = Element::new(String::from("test"));

    list.add(first);
    list.add(Element::new(String::from("bannana")));

    assert_eq!(unsafe{& *(list.next().unwrap())}.val, String::from("bannana"));
    assert_eq!(unsafe{& *(list.next().unwrap())}.val, String::from("test"));
    assert_eq!(list.next(), None);
}

#[test]
fn multiple_iterations() {
    let mut list = List::new();

    list.add(Element::new(String::from("first")));
    list.add(Element::new(String::from("second")));

    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("second"));
    list.add(Element::new(String::from("third")));
    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("first"));
    list.add(Element::new(String::from("fourth")));
    assert_eq!(list.next() , None);
    list.add(Element::new(String::from("fifth")));

    let expected = vec![String::from("fifth"), String::from("fourth"), String::from("third"), String::from("second"), String::from("first")];
    assert_eq!(list.map(|e| unsafe{(*e).val.clone()}).collect::<Vec<String>>(), expected);
}

