use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr};
use std::io::Error;
use std::os::raw::c_int;
use std::path::PathBuf;
use std::fs::File;
use std::sync::{Arc, Mutex, RwLock};
use memmap2::{Mmap};
use crate::{sqlite3_file, sqlite3_io_methods, sqlite3_vfs, SQLITE_ACCESS_READWRITE, SQLITE_NOTFOUND, SQLITE_OK, SQLITE_READONLY};


pub struct memory_sqlite_db {
    pub files:Arc<HashMap<String, RwLock<Box<dyn vir_file>>>>
}

impl memory_sqlite_db {
    pub fn get_manger()->Option<&'static memory_sqlite_db> {
        unsafe {
            MEMORY_SQLITE_DB.as_ref()
        }
    }

    pub fn  register_manger(db:memory_sqlite_db) {
        unsafe {
            MEMORY_SQLITE_DB = Some(db);
        }
    }
}

static mut MEMORY_SQLITE_DB:Option<memory_sqlite_db>= None;


pub trait vir_file {
    fn  isFullReadAndWrite(&self)->bool{
        Self::isReadAble(&self)&&Self::isWriteAble(&self)
    }
    fn isReadAble(&self) ->bool;
    fn isWriteAble(&self)->bool;

    fn openFile(&self)->bool;

}

const  READABLE_FLAGS:i32 = 1>>1;
const  WRITEABLE_FLAGS:i32 = 1>>2;
pub struct mmap_file{
    mmap:Mmap,
    accessFlags:i32

}
impl mmap_file {
    pub fn new(path: PathBuf)->Result<mmap_file, Error>{
        let  file = File::open(path)?;
        Ok(mmap_file{
            mmap:unsafe {Mmap::map(&file)?},
            accessFlags:READABLE_FLAGS&WRITEABLE_FLAGS
        })
    }
}

impl vir_file for mmap_file {

    fn isReadAble(&self) -> bool {
        self.accessFlags|READABLE_FLAGS == READABLE_FLAGS
    }

    fn isWriteAble(&self) -> bool {
        self.accessFlags|WRITEABLE_FLAGS == WRITEABLE_FLAGS
    }

    fn openFile(&self) -> bool {
        true
    }
}


pub extern "C" fn xAccess(_arg1: *mut sqlite3_vfs,
                             zName: *const ::std::os::raw::c_char,
                             flags: c_int,
                             pResOut: *mut c_int,) ->c_int {
    unsafe {
        let path = CStr::from_ptr(zName).to_owned().into_string().unwrap();
        let x = memory_sqlite_db::get_manger();
        if x.is_none() {
            return SQLITE_NOTFOUND as c_int;
        }
        let db = x.unwrap();
        let map =  &db.files;
        let fileOrNull = map.get(&path);
        if fileOrNull.is_none() {
            *pResOut = c_int::from(false);
        } else {
            let file_read = fileOrNull.expect("not happen");
            let file = file_read.read().unwrap();
            if SQLITE_READONLY == flags as u32 {
                if file.isReadAble()&&!file.isWriteAble() {
                    *pResOut = 1;
                }else {
                    *pResOut = 0;
                }
            } else if SQLITE_ACCESS_READWRITE == flags as u32 {
                if file.isFullReadAndWrite() {
                    *pResOut = 1;
                }else {
                    *pResOut = 0;
                }
            }
        }
        return SQLITE_OK as c_int;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3_file_extend_memory_file {
    pub pMethods: *const sqlite3_io_methods,
}

pub      extern  "C" fn xOpen(
arg1: *mut sqlite3_vfs,
zName: *const ::std::os::raw::c_char,
arg2: *mut sqlite3_file,
flags: ::std::os::raw::c_int,
pOutFlags: *mut ::std::os::raw::c_int,
) -> ::std::os::raw::c_int{


}

