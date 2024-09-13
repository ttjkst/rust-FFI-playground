
use std::ffi::CString;
use std::ptr::{null, null_mut};
use crate::{sqlite3, sqlite3_close, sqlite3_open};

pub struct sqlite_connect {
    db:  *mut sqlite3

}

impl sqlite_connect{
    fn new(sqlite3: *mut sqlite3)->sqlite_connect{
        sqlite_connect{
            db:sqlite3
        }
    }
    fn open(dbpath: CString)-> Option<sqlite_connect>{
        unsafe {
            let   mut db:*mut sqlite3 = null_mut();
            let status = sqlite3_open(dbpath.as_ptr(),&mut db);
            if status ==0 {
                return  Some(sqlite_connect::new(db));
            }
            //println!("hello{db}");
           return None;
        }
    }
}

impl Drop for sqlite_connect{
    fn drop(&mut self) {
        unsafe {
            let mut ptr = self.db;
            let i = sqlite3_close(ptr);
            println!("drop db")
        }
    }
}

#[cfg(test)]
pub  mod test{
    use std::ffi::CString;
    use super::sqlite_connect;

    #[test]
    fn test_open(){
        let  path = CString::new("/Users/ttjkst/Codes/dir/rust/enlu-db/tests/res/enlu.db")
            .expect("errror str");
        let option = sqlite_connect::open(path);
        match option {
            None => {
                println!("error")
            }
            Some(_) => {
                println!("ok")
            }
        }
    }
}