use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "wx_backend")]
extern "C" {
    fn start_wx_system(argc: i32, argv: *const *const c_char);
    fn set_func_pointer(func: extern "C" fn());
}

extern "C" fn print_hello() {
    println!("Hello, World! from Rust");
}

fn main() {
    // Get the command line arguments for argc and argv
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len() as i32;

    let mut argv: Vec<*const c_char> = Vec::with_capacity(argc as usize);
    for arg in args {
        argv.push(CString::new(arg).unwrap().into_raw());
    }

    // Pass the command line arguments to start_wx_system
    unsafe {
        set_func_pointer(print_hello);
        start_wx_system(argc, argv.as_ptr());
    }
}