use bevy::prelude::*;
use super::{enviroment, ui};
pub struct DomSpyPlugin;
impl Plugin for DomSpyPlugin{
    fn build(&self, app: &mut AppBuilder) {
        //setup
        app
        .add_startup_system(enviroment::camera_setup.system())
        .add_startup_system(ui::generate_ui.system());
        //spy
        app.add_system(watch_dom.system());
    }
}

fn watch_dom(
    mut text_q:Query<&mut Text,With<ForTextbox>>,
    mut slider_q:Query<&mut Text,With<ForTextbox>>,
    mut selector_q:Query<&mut Text,With<ForTextbox>>
){

}
pub struct  ForTextbox;
pub struct  ForSlider;
pub struct ForSelector;
