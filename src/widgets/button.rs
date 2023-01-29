use bevy::{
    prelude::*,
};


use crate::widgets::components::rect::Rect;
use crate::widgets::components::mouse::Mouse;
use crate::widgets::core::core_widget;

#[derive(Bundle)]
pub struct Button {
    pub rect: Rect,
    pub mouse: Mouse,
}

impl core_widget for Button {
    fn new(label: String, x: i32, y: i32, width: i32, height: i32) -> Self {
        Button {
            rect: Rect {
                label: label,
                x: x,
                y: y,
                width: width,
                height: height,
            },
            mouse: Mouse {
                left_click: false,
                right_click: false,
                hover: false,
            },
        }
    }
}