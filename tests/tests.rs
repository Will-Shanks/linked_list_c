use core::ptr;
use linked_list_c::{self, List};

linked_list_c::impl_LlItem!{[Element]}

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
}

#[test]
fn empty_list() {
    let mut list: List<'_, Element> = List::new();
    assert_eq!(list.next().is_none(), true);
}

#[test]
fn non_empty_list() {
    let mut list = List::new();
    let first = Element::new(String::from("test"));

    list.add(first);
    list.add(Element::new(String::from("bannana")));

    assert_eq!(list.next().unwrap().val, String::from("bannana"));
    assert_eq!(list.next().unwrap().val, String::from("test"));
    assert_eq!(list.next().is_none(), true);
}

#[test]
fn multiple_iterations() {
    let mut list = List::new();

    list.add(Element::new(String::from("first")));
    list.add(Element::new(String::from("second")));

    assert_eq!(list.next().unwrap().val , String::from("second"));
    list.add(Element::new(String::from("third")));
    assert_eq!(list.next().unwrap().val , String::from("first"));
    list.add(Element::new(String::from("fourth")));
    assert_eq!(list.next().is_none(), true);

    list.add(Element::new(String::from("fifth")));

    let expected = vec![String::from("fifth"), String::from("fourth"), String::from("third"), String::from("second"), String::from("first")];
    assert_eq!(list.map(|e| e.val.clone()).collect::<Vec<String>>(), expected);
}
