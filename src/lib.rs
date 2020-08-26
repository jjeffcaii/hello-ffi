#![crate_type = "dylib"]

use futures::executor::block_on;
use std::future::Future;
use std::mem::transmute;
use std::pin::Pin;

#[no_mangle]
pub extern "C" fn double_input(input: i32) -> i32 {
    println!("hello --from rust shared library");
    input * 2
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> *mut i32 {
    unsafe { transmute(Box::into_raw(Box::new(a + b))) }
}

#[no_mangle]
pub extern "C" fn get_add(f: *mut i32) -> i32 {
    let v = unsafe { &mut *f };
    *v
}

#[no_mangle]
pub extern "C" fn add_future(a: i32, b: i32) -> *mut Pin<Box<dyn Future<Output = i32>>> {
    let f = Box::pin(async move { a + b });
    Box::into_raw(Box::new(f))
}

#[no_mangle]
pub extern "C" fn block_get_add(f: *mut Pin<Box<dyn Future<Output = i32>>>) -> i32 {
    let f = unsafe { *Box::from_raw(f) };
    block_on(f)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
