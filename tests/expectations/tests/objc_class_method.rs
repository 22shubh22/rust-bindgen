/* automatically generated by rust-bindgen */

#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
pub trait Foo {
    unsafe fn method();
    unsafe fn methodWithInt_(foo: ::std::os::raw::c_int);
    unsafe fn methodWithFoo_(foo: id);
    unsafe fn methodReturningInt() -> ::std::os::raw::c_int;
    unsafe fn methodReturningFoo() -> *mut id;
    unsafe fn methodWithArg1_andArg2_andArg3_(
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    );
}
impl Foo for id {
    unsafe fn method() {
        msg_send!(
            objc::runtime::Class::get("Foo").expect("Couldn\'t find Foo"),
            method
        )
    }
    unsafe fn methodWithInt_(foo: ::std::os::raw::c_int) {
        msg_send!(
            objc::runtime::Class::get("Foo").expect("Couldn\'t find Foo"),
            methodWithInt: foo
        )
    }
    unsafe fn methodWithFoo_(foo: id) {
        msg_send!(
            objc::runtime::Class::get("Foo").expect("Couldn\'t find Foo"),
            methodWithFoo: foo
        )
    }
    unsafe fn methodReturningInt() -> ::std::os::raw::c_int {
        msg_send!(
            objc::runtime::Class::get("Foo").expect("Couldn\'t find Foo"),
            methodReturningInt
        )
    }
    unsafe fn methodReturningFoo() -> *mut id {
        msg_send!(
            objc::runtime::Class::get("Foo").expect("Couldn\'t find Foo"),
            methodReturningFoo
        )
    }
    unsafe fn methodWithArg1_andArg2_andArg3_(
        intvalue: ::std::os::raw::c_int,
        ptr: *mut ::std::os::raw::c_char,
        floatvalue: f32,
    ) {
        msg_send ! ( objc :: runtime :: Class :: get ( "Foo" ) . expect ( "Couldn\'t find Foo" ) , methodWithArg1 : intvalue andArg2 : ptr andArg3 : floatvalue )
    }
}
