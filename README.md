# linked_list_c

This is a Rust crate with the goal of making C style linked lists easier to work with.

Some FFIs accept and/or return linked lists, requiring pointer wrangling and unsafe code to handle.
This crate tries to take care of those ugly bits for you, and provide a safe and ergonomic interface instead  

## Features
- Easily create Lists that can be passed to C FFIs
- Safely iterate over a linked list received from a C FFI
- handles cleaning up List on drop
  - can set a custom drop function if Drop, or libc::free() doesn't work for you
- extremely simple trait is all thats neccessary to use with your favorite struct
  - just add `#[derive(LlItem)]` if `yourStruct.next` points to the next `yourStruct`
  - or use the `impl_LlItem!([yourStruct, ...])` macro
- Zero copy and minimal extra memory footprint (besides `from<Vec>` impl)
- currently not quite no_std, but this could easily be made possible
