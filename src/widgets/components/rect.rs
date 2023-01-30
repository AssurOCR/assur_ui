use bevy::{
    prelude::*,
};

#[derive(Component, Debug)]
pub struct Rect {
    pub label: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}