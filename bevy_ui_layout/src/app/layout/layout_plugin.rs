use bevy::{prelude::{*}, text::Text2dSize};

use super::score_board;


pub struct AppLayoutPlugin;

impl Plugin for AppLayoutPlugin{
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(score_board::layout.system());
    }
}