use json;
use std::env;
use std::fs;
use std::os::fd::AsFd;
use std::os::fd::AsRawFd;
use libelf::Elf;
use std::ffi::{c_char, CStr, CString};
use std::fs::File;
use libelf::raw::{Elf64_Shdr, Elf_Data};

use crate::error::VicError;
use crate::error::VicResult;


// for modifying the own exe file
#[link_section = ".victorinix_props"]
#[used]
static mut PROPS: i8 = 5;
//static PROPS: str = "YOUR STRING HERE";
//static mut PROPS: String = String::from("hi");
//from(r#"{"folder_initialized": false}"#);

#[derive(Debug)]
pub struct Props {
    inner: json::object::Object,
}

pub enum Prop {
    String(String),
    Int(i32),
    Bool(bool),
}

impl From<String> for Prop {
    fn from(value: String) -> Self {
        Prop::String(value)
    }
}
impl From<bool> for Prop {
    fn from(value: bool) -> Self {
        Prop::Bool(value)
    }
}
impl From<i32> for Prop {
    fn from(value: i32) -> Self {
        Prop::Int(value)
    }
}

impl Props {
    pub fn from_exe_file() -> VicResult<Self> {

        unsafe {println!("PROPS: {}", PROPS)}

        let exe = env::current_exe()?;

        let tmp = exe.with_extension("tmp");
        fs::copy(&exe, &tmp).expect("couldn't copy exe file");

        let result = unsafe {
            get_props_c(
                CString::new(exe.to_str()
                    .ok_or(VicError::msg("PathBuf of this exe can't be converted into a str"))?)?
                    .as_ptr()
            )
        };

        println!("result: {:?}", result);
        unsafe {
            let hi = *result;
            println!("err_ptr: {:?}", hi[1]);
            let c_str: &CStr = unsafe { CStr::from_ptr(hi[1]) };
            let str_slice: &str = c_str.to_str().unwrap();
            println!("error: {}", str_slice)
        }

        //let elf_bytes = fs::read(&exe)?;
        /*

        // initialise elf version
        let result = unsafe { libelf::raw::elf_version(libelf::raw::EV_CURRENT) };
        if result == libelf::raw::EV_NONE {

            // get error string for -1
            let c_buf: *const c_char = unsafe { libelf::raw::elf_errmsg(-1) };
            let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
            let str_slice: &str = c_str.to_str().unwrap();

            return VicResult::Err(VicError {msg: format!("ELF library initialization failed: {}", str_slice)});
        }


        let raw_elf = unsafe { libelf::raw::elf_begin(3, libelf::raw::Elf_Cmd::ELF_C_RDWR, std::ptr::null_mut())};
        if raw_elf == std::ptr::null_mut() {
            let c_buf: *const c_char = unsafe { libelf::raw::elf_errmsg(-1) };
            let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
            let str_slice: &str = c_str.to_str().unwrap();
            println!("elf_begin failed: {}", str_slice);
        }
        //let mut elf = unsafe {Elf::from_raw(raw_elf)};

        let kind = unsafe { libelf::raw::elf_kind(raw_elf) };

        println!("elf_kind rust: {}", kind);

        let elf_scn = unsafe {libelf::raw::elf_getscn(raw_elf, 5) };
        println!("got scn rust: {:?}", elf_scn);


        let props_section = get_props_section(raw_elf)?;
        //println!("props_section: {}", props_section);

        let elf_scn = unsafe {libelf::raw::elf_getscn(raw_elf, props_section) };


        let mut elf_data = Elf_Data {
            //d_buf: *mut c_void,
            d_buf: std::ptr::null_mut(),
            d_type: 4,
            d_version: 0,
            d_size: 0,
            d_off: 0,
            d_align: 0,
        };


        let mut elf_shdr = Elf64_Shdr {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        };
        let elf_scn_hdr = unsafe {libelf::raw::gelf_getshdr(elf_scn, &mut elf_shdr) };
        //unsafe { println!("shname: {}", elf_shdr.sh_name);}


        let new_elf_data = unsafe { libelf::raw::elf_getdata(elf_scn, &mut elf_data)};

        //let data = my_getdata(elf, props_section);

        //println!("hi data: {:?}", data);

        unsafe {

            //let new = *new_elf_data;

            println!("elf_data: {:?}", new_elf_data);
            println!("elf_data: {:?}", elf_data);
        }
        # */

        todo!()
    }

    pub fn write_to_file(self) -> VicResult<()> {
        todo!()
    }

    pub fn get<T: Into<String>>(self, path: T) -> VicResult<Prop> {
        todo!()
    }

    pub fn set<T: Into<String>, P: Into<Prop>>(self, path: T, prop: P) -> VicResult<()> {
        todo!()
    }
}


#[link(name = "victorinix")]
extern "C" { 
    fn get_props_c(path: *const c_char) -> *const [*const c_char; 2];
}



