
pub type VicResult<T> = Result<T, VicError>;

#[derive(Debug)]
pub struct VicError {
    pub msg: String,
} 

impl VicError {
    pub fn msg<T: Into<String>>(msg: T) -> VicError {
        VicError { msg: msg.into() }
    }
}

impl From<libelf::Error> for VicError {
    fn from(value: libelf::Error) -> Self {
        let string = format!("{}", value);
        return VicError { msg: string };

        //let c_buf: *const c_char = unsafe { libelf::raw::elf_errmsg(5) };
        //let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
        //let str_slice: &str = c_str.to_str().unwrap();
        //println!("ERROR: {}", str_slice);
    }
}

impl From<std::io::Error> for VicError {
    fn from(value: std::io::Error) -> Self {
        return VicError { msg: format!("{}", value) };
    }
}
