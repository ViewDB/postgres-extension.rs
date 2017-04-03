#![feature(libc, custom_attribute, plugin)]
#![plugin(postgres_extension_plugin)]

extern crate postgres_extension;
#[macro_use]
extern crate postgres_extension_macros;
pg_module!(version: 90500);


extern crate libc;
use libc::{ c_int };


#[pg_export]
pub fn is_zero(a: c_int) -> c_int {
    if a == 0 {
        return 1
    } else {
        return 0
    }
}

#[pg_export]
pub fn return_num(a: c_int) -> c_int {
    return a
}
