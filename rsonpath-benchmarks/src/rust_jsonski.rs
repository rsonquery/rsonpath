use libc::c_void;
use std::ffi::CString;

mod jsonski_extern {
    use libc::{c_char, c_long, c_void};

    extern "C" {
        pub(crate) fn loadFile(file_name: *const c_char) -> *const c_void;
        pub(crate) fn runJsonSki(query: *const c_char, record: *const c_void) -> c_long;
        pub(crate) fn dropFile(record: *const c_void);
    }
}

#[derive(Clone)]
pub struct JsonSkiRecord {
    ptr: *const c_void,
}

impl Drop for JsonSkiRecord {
    fn drop(&mut self) {
        unsafe { jsonski_extern::dropFile(self.ptr) }
    }
}

pub struct JsonSkiQuery {
    c_string: CString,
}

pub fn load_jsonski_record(file_name: &str) -> JsonSkiRecord {
    let c_file_name = CString::new(file_name).unwrap();

    unsafe {
        let record_ptr = jsonski_extern::loadFile(c_file_name.as_ptr());
        JsonSkiRecord { ptr: record_ptr }
    }
}

pub fn create_jsonski_query(query: &str) -> JsonSkiQuery {
    JsonSkiQuery {
        c_string: CString::new(query).unwrap(),
    }
}

pub fn call_jsonski(query: &JsonSkiQuery, record: &JsonSkiRecord) -> i64 {
    unsafe { jsonski_extern::runJsonSki(query.c_string.as_ptr(), record.ptr) }
}
