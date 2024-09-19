use core::error;
use std::ffi::{CStr, CString};
use std::fmt::{ Debug, Display, Formatter};
use std::ptr::{null, null_mut};
use std::rc::{Rc};
use libc::{c_char, c_int};
use crate::{sqlite3,
            sqlite3_bind_text,
            sqlite3_close,
            sqlite3_column_int,
            sqlite3_column_text,
            sqlite3_finalize,
            sqlite3_open,
            sqlite3_prepare_v2,
            sqlite3_step,
            sqlite3_stmt,
            SQLITE_DONE,
            SQLITE_OK,
            SQLITE_ROW};



#[derive(Debug)]
pub  struct sqlite_error{
    error_code:u8,
    message:String,
    is_error_misuse:bool,
}


impl  Default for sqlite_error {
    fn default() -> Self {
        sqlite_error{
            error_code:0,
            message:String::from("unknown"),
            is_error_misuse:false,
        }
    }
}

impl Display for sqlite_error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"message:{},sqlite3_code:{}",self.message,self.error_code)
    }
}

impl error::Error for sqlite_error {}

pub struct sqlite_connect {
    db:  *mut sqlite3,
}


pub struct sql_statement {
    ptr:*mut sqlite3_stmt,
    connect:Rc<*const sqlite_connect>
}

impl PartialEq for sql_statement {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl Drop for sql_statement {
    fn drop(&mut self) {
        println!("sqlite3_finalize statement");
        unsafe {
            sqlite3_finalize(self.ptr);
        }
    }
}

pub struct sql_stmt_result {
    stmt:sql_statement
}

impl sql_stmt_result {
    pub fn  next(&self) ->Result<bool,sqlite_error>{
        unsafe {
            let x = self.stmt.ptr;
            let status = sqlite3_step(x);
            if status == SQLITE_DONE as i32 { 
                 Ok(true)
            }else if status == SQLITE_ROW as i32 {
                 Ok(false)
            }else { 
                 Err(
                    sqlite_error{
                    is_error_misuse:true,
                    ..Default::default()
                    }
                )
            }
        }
    }

    pub fn getInti32(self,index:i32){
        unsafe {
             sqlite3_column_int(self.stmt.ptr, index);
        }
    }

    pub fn getString(self,index:i32)->Option<String>{
        unsafe {
            let  text = sqlite3_column_text(self.stmt.ptr, index)  as * mut c_char;
            if text!=null_mut() {
                Some(CStr::from_ptr(text).to_str().unwrap().to_string())
            }else {
                None
            }
        }
    }
}

impl sql_statement {
    pub fn bind_param(self,index:i32,param:String)->(){
        unsafe {
            sqlite3_bind_text(self.ptr,index as c_int,
                              param.as_ptr() as * const c_char,
                              param.as_bytes().len() as c_int,
                              None);
        }
    }

    pub fn  execute(self)->sql_stmt_result{
        sql_stmt_result {
            stmt:self
        }
    }
}

impl sqlite_connect{
    pub fn new(sqlite3: *mut sqlite3)->sqlite_connect{
        sqlite_connect{
            db:sqlite3,
        }
    }
    pub fn open(dbpath: CString)-> Result<sqlite_connect,sqlite_error>{
        unsafe {
            let   mut db:*mut sqlite3 = null_mut();
            let status = sqlite3_open(dbpath.as_ptr(),&mut db);
            if status == SQLITE_OK as i32 {
                return  Ok(sqlite_connect::new(db));
            }
           Err(sqlite_error{
               error_code: status as u8,
               message:String::from("open db fail"),
               ..Default::default()
           })
        }
    }
    pub fn prepare_statement(&self,sql:&str)-> Result<sql_statement,sqlite_error>{
        unsafe {
            let mut stmt:*mut  sqlite3_stmt =  null_mut();
            let mut tail: *const std::os::raw::c_char = null();
            let status = sqlite3_prepare_v2(self.db,
                                       sql.as_ptr() as * const c_char,
                                       sql.as_bytes().len() as c_int,
                                       &mut stmt, &mut tail);
            if status == SQLITE_OK as i32 {
                let statement =  sql_statement {
                    ptr: stmt,
                    connect: Rc::new(self)
                };
                Ok(statement)
            }else {
                Err(sqlite_error{
                    error_code: status as u8,
                    message:String::from("prepare_statement error!"),
                    ..Default::default()
                    }
                )
            }
        }
    }
}

impl Drop for sqlite_connect{
    fn drop(&mut self) {
        unsafe {
            sqlite3_close(self.db);
            println!("drop db")
        }
    }
}
