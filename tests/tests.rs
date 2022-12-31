use core::ptr;
use linked_list_c::{self, List};

linked_list_c::impl_LlItem!{[Element]}

pub struct Element {
    next: *mut Element,
    val: String
}

impl Element {
    pub fn new_raw(v: String) -> *mut Element {
        Box::into_raw(Box::new(Element::new(v)))
    }
    pub fn new(v: String) -> Element {
        Element{val:v, next: ptr::null_mut()}
    }
}

fn init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn empty_list() {
    init();
    let mut list: List<'_, Element> = List::new();
    assert_eq!(list.next().is_none(), true);
}

#[test]
fn non_empty_list() {
    init();
    let mut list = List::new();
    let first = Element::new_raw(String::from("test"));

    list.add(first);
    list.add(Element::new_raw(String::from("bannana")));

    assert_eq!(list.next().unwrap().val, String::from("bannana"));
    assert_eq!(list.next().unwrap().val, String::from("test"));
    assert_eq!(list.next().is_none(), true);
}

#[test]
fn multiple_iterations() {
    let mut list = List::new();

    list.add(Element::new_raw(String::from("first")));
    list.add(Element::new_raw(String::from("second")));

    assert_eq!(list.next().unwrap().val , String::from("second"));
    list.add(Element::new_raw(String::from("third")));
    assert_eq!(list.next().unwrap().val , String::from("first"));
    list.add(Element::new_raw(String::from("fourth")));
    assert_eq!(list.next().is_none(), true);

    list.add(Element::new_raw(String::from("fifth")));

    let expected = vec![String::from("fifth"), String::from("fourth"), String::from("third"), String::from("second"), String::from("first")];
    assert_eq!(list.map(|e| e.val.clone()).collect::<Vec<String>>(), expected);
}

#[test]
fn from_vec() {
    let mut list: List<Element> = vec![
        Element::new(String::from("sixth")),
        Element::new(String::from("fifth")),
        Element::new(String::from("fourth")),
    ].into();
    assert_eq!(list.next().unwrap().val , String::from("fourth"));
    assert_eq!(list.next().unwrap().val , String::from("fifth"));
    assert_eq!(list.next().unwrap().val , String::from("sixth"));
    assert_eq!(list.next().is_none(), true);
}

#[test]
fn combine() {
    let mut list: List<Element> = vec![
        Element::new(String::from("sixth")),
        Element::new(String::from("fifth")),
        Element::new(String::from("fourth")),
    ].into();

    let start: List<Element> = vec![
        Element::new(String::from("third")),
        Element::new(String::from("second")),
        Element::new(String::from("first")),
    ].into();

    list.combine(start);

    assert_eq!(list.next().unwrap().val , String::from("first"));
    assert_eq!(list.next().unwrap().val , String::from("second"));
    assert_eq!(list.next().unwrap().val , String::from("third"));
    assert_eq!(list.next().unwrap().val , String::from("fourth"));
    assert_eq!(list.next().unwrap().val , String::from("fifth"));
    assert_eq!(list.next().unwrap().val , String::from("sixth"));
    assert_eq!(list.next().is_none(), true);
}
