use enlu_db::rsqlite3::sqlite_connect;
#[cfg(test)]
pub  mod test {
    use std::ffi::CString;
    use crate::sqlite_connect;

    #[test]
    fn test_open() {
        let path = CString::new("path/to/db")
            .expect("errror str");
        let connect = sqlite_connect::open(path);
        match connect {
            Ok(sqlite_connect) => {
                let statement = sqlite_connect.prepare_statement("select * from enlu_version")
                    .expect("TODO: panic message");
                let result = statement.execute();
                let has_next = result.next().unwrap();
                let string = result.getString(1).unwrap();
                println!("{string}")
            }
            Err(_) => {
                println!("ok")
            }
        }
    }

}