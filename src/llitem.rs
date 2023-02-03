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

// Implement LlItem for any struct which contains a member next: *mut Self which points to the next element in the list
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
