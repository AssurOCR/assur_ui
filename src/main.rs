use chrono;

use bevy::app::App;
use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::schedule::ReportExecutionOrderAmbiguities,
    utils::Duration,
};
use bevy::ecs::system::Commands;

mod plugins;
mod widgets;
use plugins::wx_plugin::{start_wx_widgets, wx_loop};
use widgets::button::Button;
use widgets::components::rect::Rect;
use widgets::components::mouse::Mouse;
use widgets::core::core_widget;


fn get_current_time() -> String {
    chrono::Utc::now().format("%b %-d, %-I:%M:%S").to_string()
}

fn greet_hello_world() {
    println!("{}: Asalam Alaikum World!", get_current_time());
}

fn spawn_main_win(mut commands: Commands) {
    commands.spawn(Button::new("Hello, World!".to_string(), 0, 0, 100, 100));
}

fn main() {
    println!("Starting at: {}", get_current_time());

    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(100)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_startup_system(start_wx_widgets)
        .add_startup_system(spawn_main_win)
        .add_system(wx_loop)
        .add_system(greet_hello_world)
        .run();
}