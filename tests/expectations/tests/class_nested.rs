/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct A {
    pub member_a: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct A_B {
    pub member_b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_B() {
    assert_eq!(::std::mem::size_of::<A_B>() , 4usize , concat ! (
               "Size of: " , stringify ! ( A_B ) ));
    assert_eq! (::std::mem::align_of::<A_B>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( A_B ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const A_B ) ) . member_b as * const _ as usize
                } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( A_B ) , "::" ,
                stringify ! ( member_b ) ));
}
impl Clone for A_B {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct A_C {
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_C() {
    assert_eq!(::std::mem::size_of::<A_C>() , 4usize , concat ! (
               "Size of: " , stringify ! ( A_C ) ));
    assert_eq! (::std::mem::align_of::<A_C>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( A_C ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const A_C ) ) . baz as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( A_C ) , "::" ,
                stringify ! ( baz ) ));
}
impl Clone for A_C {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct A_D<T> {
    pub foo: T,
}
impl <T> Default for A_D<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(::std::mem::size_of::<A>() , 4usize , concat ! (
               "Size of: " , stringify ! ( A ) ));
    assert_eq! (::std::mem::align_of::<A>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( A ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const A ) ) . member_a as * const _ as usize }
                , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( A ) , "::" , stringify
                ! ( member_a ) ));
}
impl Clone for A {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "var"]
    pub static mut var: A_B;
}
#[test]
fn __bindgen_test_layout_A_D_instantiation_16() {
    assert_eq!(::std::mem::size_of::<A_D<::std::os::raw::c_int>>() , 4usize ,
               concat ! (
               "Size of template specialization: " , stringify ! (
               A_D<::std::os::raw::c_int> ) ));
    assert_eq!(::std::mem::align_of::<A_D<::std::os::raw::c_int>>() , 4usize ,
               concat ! (
               "Alignment of template specialization: " , stringify ! (
               A_D<::std::os::raw::c_int> ) ));
}
extern "C" {
    #[link_name = "baz"]
    pub static mut baz: A_D<::std::os::raw::c_int>;
}
#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct D {
    pub member: A_B,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(::std::mem::size_of::<D>() , 4usize , concat ! (
               "Size of: " , stringify ! ( D ) ));
    assert_eq! (::std::mem::align_of::<D>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( D ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const D ) ) . member as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( D ) , "::" , stringify
                ! ( member ) ));
}
impl Clone for D {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Templated<T> {
    pub member: T,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Templated_Templated_inner<T> {
    pub member_ptr: *mut T,
}
impl <T> Default for Templated_Templated_inner<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl <T> Default for Templated<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
