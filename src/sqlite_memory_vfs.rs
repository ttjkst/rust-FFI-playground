use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::os::raw::c_int;
use std::rc::Rc;
use std::sync::Arc;
use crate::{sqlite3_vfs, SQLITE_ACCESS_READ, SQLITE_NOTFOUND};


pub struct memory_sqlite_db {
    files:HashMap<CString, Rc<dyn vir_file>>,
}

static mut MEMORY_SQLITE_DB: memory_sqlite_db = memory_sqlite_db{
    files:HashMap::new()
};

pub trait vir_file {
    fn  isFullReadAndWrite()->bool{
        Self::isReadAble()&&Self::isWriteAble()
    }
    fn isReadAble()->bool;
    fn isWriteAble()->bool;

}

extern "C" unsafe fn xAccess(arg1: *mut sqlite3_vfs,
                             zName: *const ::std::os::raw::c_char,
                             flags: c_int,
                             pResOut: *mut c_int,) ->c_int {
    let path = CStr::from_ptr(zName).into_c_string();
    let fileOrNull = MEMORY_SQLITE_DB.files.get(&path);
    if  fileOrNull.is_none() {
        return SQLITE_NOTFOUND as c_int;
    }else {
        let file = fileOrNull.expect("not happen");
        if   SQLITE_ACCESS_READ == flags as u32 {
            file.isReadAble();


        }
    }

}