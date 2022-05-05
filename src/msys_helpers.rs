use::std::os::raw::c_int;
use std::io::{Error, ErrorKind};

pub type c_char = u8;

pub const S_IRUSR: c_int = 256;
pub const S_IWUSR: c_int = 128;
pub const S_IXUSR: c_int = 64;
pub const S_IRWXU: c_int = S_IRUSR | S_IWUSR | S_IXUSR;
pub const S_IRGRP: c_int = 32;
pub const S_IWGRP: c_int = 16;
pub const S_IXGRP: c_int = 8;
pub const S_IRWXG: c_int = S_IRGRP | S_IWGRP | S_IXGRP;
pub const S_IROTH: c_int = 4;
pub const S_IWOTH: c_int = 2;
pub const S_IXOTH: c_int = 1;
pub const S_IRWXO: c_int = S_IROTH | S_IWOTH | S_IXOTH;

extern "C" {
    pub fn chmod(filename: *const c_char, mode: c_int) -> c_int;
}

pub fn msys_set_permissions(file_path: &str, mode: i32) -> Result<bool, Error> {
    let success = unsafe {chmod(file_path.as_ptr(), mode)};
    if success == 0 {
        return Ok(true);
    }
    else {
        let error_string = format!("Could not change permission of file: {}", file_path);
        return Err(Error::new(ErrorKind::Other, error_string));
    }
}

