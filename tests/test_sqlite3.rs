use enlu_db::rsqlite3::sqlite_connect;
#[cfg(test)]
pub  mod test {
    use std::ffi::CString;
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

}