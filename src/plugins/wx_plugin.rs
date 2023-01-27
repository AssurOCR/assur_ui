use chrono;
use std::ffi::CString;

#[link(name = "wx_backend")]
extern "C" {
    fn start_wx_system(argc: i32, argv: *const *const u8);
    fn set_func_pointer(func: extern "C" fn());
    fn update_events_loop();
}

extern "C" fn print_hello() {
    println!("Hello, World! from Rust");
}

fn get_current_time() -> String {
    chrono::Utc::now().format("%b %-d, %-I:%M:%S").to_string()
}


fn get_argc_argv() -> (i32, *const *const u8) {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len() as i32;

    let mut argv: Vec<*const u8> = Vec::with_capacity(argc as usize);
    for arg in args {
        argv.push(CString::new(arg).unwrap().into_raw() as *const u8);
    }

    (argc, argv.as_ptr())
}

pub fn start_wx_widgets() {
    unsafe {
        println!("{}: Starting wx_system!", get_current_time());

        let (argc, argv) = get_argc_argv();

        // Pass the command line arguments to start_wx_system

        set_func_pointer(print_hello);
        start_wx_system(argc, argv);
    }
}

pub fn wx_loop() {
    unsafe {
        update_events_loop();
    }
}