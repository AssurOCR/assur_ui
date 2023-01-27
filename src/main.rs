use chrono;

use bevy::app::App;
use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    utils::Duration,
};

mod plugins;
use plugins::wx_plugin::{start_wx_widgets, wx_loop};


fn get_current_time() -> String {
    chrono::Utc::now().format("%b %-d, %-I:%M:%S").to_string()
}

fn greet_hello_world() {
    println!("{}: Asalam Alaikum World!", get_current_time());
}

fn main() {
    println!("Starting at: {}", get_current_time());

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(start_wx_widgets)
        .add_system(wx_loop)
        .add_system(greet_hello_world)
        .run();
}