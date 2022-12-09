use std::ptr;

//to be compatable with C linked list returned next __must__ be a *mut Element

#[derive(Debug)]
pub struct Element {
    pub next: *mut Element,
    pub val: String
}

impl Element {
    pub fn new(v: String) -> Box<Element> {
        let n = ptr::null::<Element>() as *mut Element;
        Box::new(Element{val: v, next: n }) 
    }
    //next should be Pinned so it doesn't move
    fn set_next(&mut self, next: *mut Element) {
        let old = self.next;
        if !old.is_null() && !next.is_null() { 
            unsafe{(*next).set_next(old)};
        }
        self.next = next;
    }
}

//impl Copy for Element {}

//impl Clone for Element {
//    fn clone(&self) -> Self {
//        *self
//    }
//}


pub struct List {
    head: *mut Element,
    next: *mut Element,
}

impl<'a> List {
    pub fn new() -> List {
        List{head: ptr::null::<Element>() as *mut Element, next: ptr::null::<Element>() as *mut Element}
    }

    pub fn add(&mut self, mut elem: Box<Element>) {
        if !self.head.is_null() {
            elem.set_next(self.head);
        }
        self.head = Box::into_raw(elem);
        println!("{:?} in add", unsafe{& *self.head});
    }
}

impl Iterator for List {
    type Item = *mut Element;

    fn next(&mut self) -> Option<Self::Item> {
        println!("next");
        if self.head.is_null() {
            println!("nope");
            return None;
        }
        println!("{:?} head in next", unsafe{& *self.head});
        if self.next.is_null() {
            println!("foo");
            println!("{:?} head in next", unsafe{& *self.head});
            self.next = self.head;
            println!("{:?} next in next", unsafe{& *self.next});
            return Some(self.next);
        }
        if unsafe{& *self.next}.next.is_null() {
            println!("bar");
            self.next = ptr::null::<Element>() as *mut Element;
            return None;
        }
        println!("baz");
        self.next = unsafe{& *self.next}.next;
        return Some(self.next);
    }
}



#[test]
fn empty_list() {
    println!("start");
    let mut list = List::new();
    println!("middle");
    assert_eq!(list.next(), None);
    println!("end");
}

#[test]
fn non_empty_list() {
    let mut list = List::new();
    let first = Element::new(String::from("test"));
    println!("{:?} first", first);

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

    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("fifth"));
    list.add(Element::new(String::from("sixth")));
    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("fourth"));
    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("third"));
    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("second"));
    assert_eq!(unsafe{& *list.next().unwrap()}.val , String::from("first"));
    assert_eq!(list.next(), None);
}

