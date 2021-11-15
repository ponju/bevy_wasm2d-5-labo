use std::time::Duration;

use bevy::{prelude::{*}, text::Text2dSize, ui::widget::Image};

pub fn camera_setup(mut commands:Commands){
    commands.insert_resource(ClearColor(Color::rgb(0.,0.,0.)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
