use cstr::{Char, WideChar};

#[cfg_attr(target_os = "linux", link(name = "odbcinst", kind = "dylib"))]
#[cfg_attr(target_os = "macos", link(name = "iodbcinst", kind = "dylib"))]
#[cfg_attr(target_os = "windows", link(name = "odbccp32", kind = "raw-dylib"))]
extern "C" {
    pub fn SQLValidDSNW(dsn: *const WideChar) -> bool;
    pub fn SQLWriteDSNToIniW(dsn: *const WideChar, driver: *const WideChar) -> bool;
    pub fn SQLWritePrivateProfileStringW(
        section: *const WideChar,
        entry: *const WideChar,
        string: *const WideChar,
        filename: *const WideChar,
    ) -> bool;
    pub fn SQLRemoveDSNFromIniW(dsn: *const WideChar) -> bool;
    pub fn SQLGetPrivateProfileStringW(
        section: *const WideChar,
        entry: *const WideChar,
        default: *const WideChar,
        buffer: *mut WideChar,
        buffer_size: i32,
        filename: *const WideChar,
    ) -> i32;
    pub fn SQLGetPrivateProfileString(
        section: *const Char,
        entry: *const Char,
        default: *const Char,
        buffer: *mut Char,
        buffer_size: i32,
        filename: *const Char,
    ) -> i32;
    pub fn SQLGetConfigMode(buffer: *mut u32) -> i32;
}
