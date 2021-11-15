use bevy::prelude::*;
pub struct DomSpyPlugin;
impl Plugin for DomSpyPlugin{
    fn build(&self, app: &mut AppBuilder) {
        // app
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
