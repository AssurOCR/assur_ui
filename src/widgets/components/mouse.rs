use bevy::{
    prelude::*,
};

#[derive(Component, Debug)]
pub struct Mouse {
    pub left_click: bool,
    pub right_click: bool,
    pub hover: bool
}
