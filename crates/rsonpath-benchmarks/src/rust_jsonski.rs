use crate::framework::implementation::Implementation;
use libc::c_void;
use std::{
    ffi::{CString, NulError},
    num::TryFromIntError,
};
use thiserror::Error;

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

pub struct JsonSki {}

impl Implementation for JsonSki {
    type Query = JsonSkiQuery;

    type File = JsonSkiRecord;

    type Error = JsonSkiError;

    fn id() -> &'static str {
        "jsonski"
    }

    fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }

    fn load_file(&self, file_path: &str) -> Result<Self::File, Self::Error> {
        let c_file_name =
            CString::new(file_path).map_err(|err| JsonSkiError::InvalidFilePath { source: err })?;

        unsafe {
            let record_ptr = jsonski_extern::loadFile(c_file_name.as_ptr());
            Ok(JsonSkiRecord { ptr: record_ptr })
        }
    }

    fn compile_query(&self, query: &str) -> Result<Self::Query, Self::Error> {
        Ok(JsonSkiQuery {
            c_string: CString::new(query)
                .map_err(|err| JsonSkiError::InvalidQueryString { source: err })?,
        })
    }

    fn run(&self, query: &Self::Query, file: &Self::File) -> Result<u64, Self::Error> {
        Ok(unsafe {
            let res = jsonski_extern::runJsonSki(query.c_string.as_ptr(), file.ptr);

            res.try_into()
                .map_err(|err| JsonSkiError::ResultOutOfRange {
                    value: res,
                    source: err,
                })?
        })
    }
}

#[derive(Error, Debug)]
pub enum JsonSkiError {
    #[error("error turning file path to a CString")]
    InvalidFilePath {
        #[source]
        source: NulError,
    },
    #[error("error turning query string to a CString")]
    InvalidQueryString {
        #[source]
        source: NulError,
    },
    #[error("received result outside of u64 range: {value}")]
    ResultOutOfRange {
        value: i64,
        #[source]
        source: TryFromIntError,
    },
}
