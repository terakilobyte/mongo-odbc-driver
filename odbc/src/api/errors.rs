use mongodb::error::{BulkWriteFailure, Error, ErrorKind, WriteFailure};
const VENDOR_IDENTIFIER: &str = "MongoDB";

// SQL states
pub const HYC00: &str = "HYC00";
pub const HY024: &str = "HY024";
pub const _01S02: &str = "01S02";

/// modified from code() in:
/// https://github.com/mongodb/mongo-rust-driver/blob/main/src/error.rs
fn get_code(me: &Error) -> Option<i32> {
    match me.kind.as_ref() {
        ErrorKind::Command(command_error) => Some(command_error.code),
        // errors other than command errors probably will not concern us, but
        // the following is included for completeness.
        ErrorKind::BulkWrite(BulkWriteFailure {
            write_concern_error: Some(wc_error),
            ..
        }) => Some(wc_error.code),
        ErrorKind::Write(WriteFailure::WriteConcernError(wc_error)) => Some(wc_error.code),
        _ => None,
    }
}

#[derive(Debug)]
pub enum ODBCError {
    Unimplemented(&'static str),
    InvalidHandleType(&'static str),
    InvalidAttrValue(&'static str),
    MongoError(Error),
    OptionValueChanged(&'static str, &'static str),
    UriFormatError(&'static str),
}

impl ODBCError {
    pub fn get_sql_state(&self) -> &str {
        match self {
            ODBCError::Unimplemented(_) => HYC00,
            ODBCError::MongoError(_) => HYC00,
            ODBCError::InvalidAttrValue(_) => HY024,
            ODBCError::InvalidHandleType(_) => HYC00,
            ODBCError::OptionValueChanged(_, _) => _01S02,
            ODBCError::UriFormatError(_) => HYC00,
        }
    }
    pub fn get_error_message(&self) -> String {
        match self {
            ODBCError::Unimplemented(fn_name) => format!(
                "[{}][API] The feature {} is not implemented",
                VENDOR_IDENTIFIER, fn_name
            ),
            ODBCError::MongoError(error) => format!(
                "[{}][API] Mongo specific error: {:?}",
                VENDOR_IDENTIFIER, error
            ),
            ODBCError::InvalidHandleType(ty) => format!(
                "[{}][API] Invalid handle type, expected {}",
                VENDOR_IDENTIFIER, ty
            ),
            ODBCError::InvalidAttrValue(attr) => format!(
                "[{}][API] Invalid value for attribute {}",
                VENDOR_IDENTIFIER, attr
            ),
            ODBCError::OptionValueChanged(attr, value) => format!(
                "[{}][API] Invalid value for attribute {}, changed to {}",
                VENDOR_IDENTIFIER, attr, value
            ),
            ODBCError::UriFormatError(s) => {
                format!("[{}][API] Uri Format Error: {}", VENDOR_IDENTIFIER, s)
            }
        }
    }
    pub fn get_native_err_code(&self) -> i32 {
        match self {
            // Functions that return these errors don't interact with MongoDB,
            // and so the driver returns 0 since it doesn't have a native error
            // code to propagate.
            ODBCError::Unimplemented(_)
            | ODBCError::InvalidAttrValue(_)
            | ODBCError::InvalidHandleType(_)
            | ODBCError::OptionValueChanged(_, _)
            | ODBCError::UriFormatError(_) => 0,
            ODBCError::MongoError(me) => get_code(me).unwrap_or(-1),
        }
    }
}
