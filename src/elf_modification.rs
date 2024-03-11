use json;
use std::env;
use std::fs;
use libelf::Elf;
use std::ffi::c_char;
use std::ffi::CStr;
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

        //let tmp = exe.with_extension("tmp");
        //fs::copy(&exe, &tmp).expect("couldn't copy exe file");

        let elf_bytes = fs::read(&exe)?;

        // initialise elf version
        let result = unsafe { libelf::raw::elf_version(libelf::raw::EV_CURRENT) };
        if result == libelf::raw::EV_NONE {

            // get error string for -1
            let c_buf: *const c_char = unsafe { libelf::raw::elf_errmsg(-1) };
            let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
            let str_slice: &str = c_str.to_str().unwrap();

            return VicResult::Err(VicError {msg: format!("ELF library initialization failed: {}", str_slice)});
        }

        //let mut elf = Elf::from_bytes(&elf_bytes)?;
        let mut elf = Elf::open("/home/me/work/tori-victorinix/target/debug/victorinix")?;
        println!("elf: {:?}", elf);

        let kind = unsafe { libelf::raw::elf_kind(elf.as_ptr()) };
        println!("elf_kind: {}", kind);

            //let header = libelf::raw::elf32_getehdr(elf.as_ptr());
            //println!("elf type: {}", (*header).e_type)

        let props_section = get_props_section(elf.as_ptr())?;

        let elf_scn = unsafe {libelf::raw::elf_getscn(elf.as_ptr(), props_section) };
        println!("elf_scn: {:?}", elf_scn);

        let mut elf_data = Elf_Data {
            //d_buf: *mut c_void,
            d_buf: std::ptr::null_mut(),
            d_type: 4,
            d_version: 0,
            d_size: 0,
            d_off: 0,
            d_align: 0,
        };

        let new_elf_data = unsafe { libelf::raw::elf_getdata(elf_scn, &mut elf_data)};

        unsafe {
            //let new = *new_elf_data;

            println!("elf_data: {:?}", new_elf_data);
            println!("elf_data: {:?}", elf_data);
        }

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

fn get_props_section(elfptr: *mut libelf::raw::Elf) -> VicResult<usize> {

    let mut counter = 1; // start with 1, because section number 0 is always an empty one.
    while true {

        let elf_scn = unsafe {libelf::raw::elf_getscn(elfptr, counter) };
        //println!("got scn: {:?}", elf_scn);

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
        //println!("old elf64 hdr: {:?}", elf_shdr);
        //println!("new elf64 hdr: {:?}", unsafe { *elf_scn_hdr});

        //unsafe {println!("sh_name index: {:?}", (*elf_scn_hdr).sh_name)};

        let mut shstrndx: usize = 0;
        let elf_shdrstndx_result = unsafe {libelf::raw::elf_getshdrstrndx(elfptr, &mut shstrndx)};
        // result should be 0, otherwise an error occured
        //println!("elf_shdrstrndx: {}", shstrndx);

        let c_buf: *const c_char = unsafe { libelf::raw::elf_strptr(elfptr, shstrndx.try_into().unwrap(), elf_shdr.sh_name.try_into().unwrap()) };

        //let c_buf: *mut c_char = unsafe { libelf::raw::elf_strptr(elfptr, 42, 72) };
        //println!("c_buf: {:?}", c_buf);
        //println!("here");
        let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
        let str_slice: &str = c_str.to_str().unwrap();
        println!("section name: {}", str_slice);

        if str_slice == ".victorinix_props" { return Ok(counter); }

        counter = counter +1;

        if counter > 100000 { return Err(VicError{msg: "Could not find the .victorinix_props section in the first 100000 sections".to_owned()})}
    }

    unreachable!()
}


