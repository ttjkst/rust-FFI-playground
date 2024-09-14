use core::error;
use std::ffi::CString;
use std::fmt::{write, Debug, Display, Formatter};
use std::ptr::{null, null_mut};
use libc::{c_char, c_int};
use crate::{sqlite3, sqlite3_bind_text, sqlite3_close, sqlite3_open, sqlite3_prepare_v2, sqlite3_step, sqlite3_stmt, SQLITE_DONE, SQLITE_OK, SQLITE_ROW};



#[derive(Debug)]
pub  struct sqlite_error{
    error_code:u8,
    message:String,
}

impl sqlite_error_misuse for sqlite_error {
    
}

impl Display for sqlite_error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"message:{},sqlite3_code:{}",self.message,self.error_code)
    }
}

impl error::Error for sqlite_error {}

pub struct sqlite_connect {
    db:  *mut sqlite3

}

pub struct sql_statement {
    stmt:*mut sqlite3_stmt
}

pub struct sql_stmt_result {
    stmt:*mut sqlite3_stmt
}

impl sql_stmt_result {
    pub fn  next(self) ->Result<bool,sqlite_error>{
        unsafe {
            let status = sqlite3_step(self.stmt);
            if status == SQLITE_DONE as i32 { 
                return Result::Ok(true)
            }else if status == SQLITE_ROW as i32 {
                return Ok(false)
            }else { 
                return Err(sqlite_error{message:})
            }
        }
    }
}

impl sql_statement {
    pub fn bind_param(self,index:i32,param:String)->(){
        unsafe {
            sqlite3_bind_text(self.stmt,index as c_int,
                              param.as_ptr() as * const c_char,
                              param.as_bytes().len() as c_int,
                              None);
        }
    }

    pub fn  execute()
}

impl sqlite_connect{
    fn new(sqlite3: *mut sqlite3)->sqlite_connect{
        sqlite_connect{
            db:sqlite3
        }
    }
    fn open(dbpath: CString)-> Result<sqlite_connect,sqlite_error>{
        unsafe {
            let   mut db:*mut sqlite3 = null_mut();
            let status = sqlite3_open(dbpath.as_ptr(),&mut db);
            if status == SQLITE_OK as i32 {
                return  Ok(sqlite_connect::new(db));
            }
           Err(sqlite_error{error_code: status as u8,message:String::from("open db fail")})
        }
    }
    fn prepare_statement(self,sql:String)-> Result<sql_statement,sqlite_error>{
        unsafe {
            let mut stmt:*mut  sqlite3_stmt =  null_mut();
            let mut tail: *const std::os::raw::c_char = null();
            let status = sqlite3_prepare_v2(self.db,
                                       sql.as_ptr() as * const c_char,
                                       sql.as_bytes().len() as c_int,
                                       &mut stmt, &mut tail);
            if status == SQLITE_OK as i32 {
                Ok(sql_statement{stmt})
            }else {
                Err(sqlite_error{error_code: status as u8,message:String::from("prepare_statement error!")})
            }
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
            Ok(sqlite_connect) => {
                println!("error")
            }
            Err(_) => {
                println!("ok")
            }
        }
    }
}