use chrono;

use bevy::app::App;
use bevy::prelude::*;
use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    log::LogPlugin,
    prelude::*,
    utils::Duration,
};

use std::ffi::CString;
use std::os::raw::c_char;


static mut can_start_wx: bool = true;

#[link(name = "wx_backend")]
extern "C" {
    fn start_wx_system(argc: i32, argv: *const *const c_char);
    fn set_func_pointer(func: extern "C" fn());
    fn update_events_loop();
}

extern "C" fn print_hello() {
    println!("Hello, World! from Rust");
}

fn get_argc_argv() -> (i32, *const *const i8) {
    let args: Vec<String> = std::env::args().collect();
    let argc = args.len() as i32;

    let mut argv: Vec<*const i8> = Vec::with_capacity(argc as usize);
    for arg in args {
        argv.push(CString::new(arg).unwrap().into_raw());
    }

    (argc, argv.as_ptr())
}

fn start_wx_widgets() {
    unsafe {
        if (can_start_wx) {
            println!("{}: Starting wx_system!", get_current_time());

            can_start_wx = false;

            let (argc, argv) = get_argc_argv();

            // Pass the command line arguments to start_wx_system

            set_func_pointer(print_hello);
            start_wx_system(argc, argv);
        } else {
            println!("{}: Cancelled starting wx_system!", get_current_time());
        }
    }
}

fn get_current_time() -> String {
    chrono::Utc::now().format("%b %-d, %-I:%M:%S").to_string()
}

fn greet_hello_world() {
    println!("{}: Asalam Alaikum World!", get_current_time());
}

fn wx_loop() {
    unsafe {
        update_events_loop();
    }
}

fn main() {
    println!("Starting at: {}", get_current_time());

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(start_wx_widgets)
        //.add_system(greet_hello_world)
        .add_system(wx_loop)
        .run();

    //start_wx_widgets();
}