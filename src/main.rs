use libelf::Elf;
use std::env;
use std::fs;
use std::path::Path;

#[link_section = "count"]
#[used]
static mut RUN_COUNT: u32 = 0;

fn main() {
    println!("Hello, world!");
    let run_count = unsafe { RUN_COUNT };
    println!("Previous run count: {}", run_count);

    let exe = env::current_exe().expect("could not get current_exe");
    let tmp = exe.with_extension("tmp");
    fs::copy(&exe, &tmp).expect("couldn't copy exe file");

    println!("elf file to open: {}", &tmp.display());
    println!("original exe file: {}", &exe.display());
    let elf_bytes = fs::read(&tmp).expect("could not read file");
    //let elf_res = Elf::open(Path::new("/home/me/work/tori-victorinix/victorinix"));
    let elf_res = Elf::from_bytes(&elf_bytes);
    println!("elf_res: {:?}", elf_res);
    let elf = elf_res.expect("could not open elf file");

    unsafe {
        let header = libelf::raw::elf32_getehdr(elf.as_ptr());
        println!("elf type: {}", (*header).e_type)
    };

}
