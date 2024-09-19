#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate core;

pub mod rsqlite3;
pub mod sqlite_memory_vfs;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {}