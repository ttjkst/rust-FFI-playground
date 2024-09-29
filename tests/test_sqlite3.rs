use enlu_db::rsqlite3::sqlite_connect;
#[cfg(test)]
pub  mod test {
    use std::collections::HashMap;
    use std::ffi::{c_int, CString};
    use std::path::PathBuf;
    use std::ptr::null_mut;
    use std::sync::{Arc, RwLock};
    use enlu_db::{sqlite3_vfs, SQLITE_ACCESS_READWRITE, SQLITE_READONLY};
    use enlu_db::sqlite_memory_vfs::{memory_sqlite_db, mmap_file, vir_file, xAccess};
    use crate::sqlite_connect;

    ///Users/ttjkst/Codes/dir/rust/enlu-db/tests/res/enlu.db
    #[test]
    fn test_open() {
        let path = CString::new("/Users/ttjkst/Codes/dir/rust/enlu-db/tests/res/enlu.db")
            .expect("error str");
        let connect = sqlite_connect::open(path);
        match connect {
            Ok(sqlite_connect) => {
                let statement = sqlite_connect.prepare_statement("select * from enlu_version")
                    .expect("TODO: panic message");
                let result = statement.execute();
                let _has_next = result.next().unwrap();
                let string = result.getString(1).unwrap();
                println!("{string}");

                let statement2 = sqlite_connect.prepare_statement("select * from enlu_version")
                    .expect("error");
                let result2 = statement2.execute();
                let _has_next = result2.next().unwrap();
                let string2 = result2.getString(1).unwrap();
                println!("{string2}");

            }
            Err(_) => {
                println!("ok")
            }
        }
    }

    #[test]
    fn testXopen(){
        let file = mmap_file::new(PathBuf::from("/Users/ttjkst/Codes/dir/rust/enlu-db/tests/res/enlu.db"));

        let mut hash_map = HashMap::new();
        let x = (file.unwrap() as Box<dyn vir_file>);
        hash_map.insert(String::from("1212"),RwLock::new(x));
        let map = Arc::new(hash_map);
        let db = memory_sqlite_db { files: map, };
        memory_sqlite_db::register_manger(db);
        unsafe {
            let  sqlite_vfs:*mut sqlite3_vfs = null_mut();
            let mut value:c_int = 42;
            let  p_res_out  =  &mut value as *mut c_int;
            let i = xAccess(sqlite_vfs, CString::new(String::from("1212")).unwrap().as_ptr(), SQLITE_READONLY as c_int, p_res_out);
            println!("{},{}",*p_res_out,i)
        }

    }

}