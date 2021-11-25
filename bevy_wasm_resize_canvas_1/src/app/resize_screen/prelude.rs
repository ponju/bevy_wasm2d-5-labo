use bevy::prelude::*;
use std::fmt::Error;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch="wasm32")]
use web_sys::*;

use super::{enviroment, ui::{self, Resolution}};
pub struct ResizeScreenPlugin;

impl Plugin for ResizeScreenPlugin{
    fn build(&self, app: &mut AppBuilder) {
        //setup
        app
        .add_startup_system(enviroment::camera_setup.system())
        .add_startup_system(ui::generate_ui.system());

        #[cfg(target_arch="wasm32")]
        app.add_startup_system(cache_screen_info.system());
        //spy
        #[cfg(target_arch="wasm32")]
        app.add_system(resize_canvas.system());

        let canvas=window().unwrap().document().unwrap().find_by_id::<HtmlElement>("bevy_app").unwrap();
        app.insert_non_send_resource(Canvas{htmlElement:canvas});
    }
}

pub static mut PRIMARY_WINDOW:Option<bevy::prelude::Window>=None;
pub struct Canvas{
    pub htmlElement:HtmlElement
}

#[cfg(target_arch="wasm32")]
fn cache_screen_info(mut commands:Commands){
}

#[cfg(target_arch="wasm32")]
fn resize_canvas(
    text_style:Res<TextStyle>,
    mut resolution_text_q:Query<&mut Text,(With<Resolution>)>,
    mut windows:ResMut<Windows>,
    canvas:NonSend<Canvas>
){
    let width=canvas.htmlElement.offset_width() as f32;
    let height =canvas.htmlElement.offset_height() as f32;
    let window=windows.get_primary_mut().unwrap();

    if window.width()==width && window.height() == height{
        return;
    }

    window.set_resolution(width,height);

    match resolution_text_q.single_mut(){
        Ok(mut text)=>{
            text.sections[0]=TextSection{
                style:text_style.clone(),
                value:format!("RESOLUTION:{},{}",width,height).to_string()
            }
        },
        Err(_)=>{eprintln!("No text for textbox in app-uis")}
    };
}

#[cfg(target_arch="wasm32")]
trait DocumentExpand{
    fn find_by_id<T>(self:&Self,id:&str) ->Option<T> where T:JsCast;
}

#[cfg(target_arch="wasm32")]
impl DocumentExpand for Document{
    fn find_by_id<T>(self:&Self, id:&str) -> Option<T>
    where T:JsCast {
        match self.get_element_by_id(id){
            Some(element)=>{
                let dyn_cast:Result<T,Element>=element.dyn_into::<T>();
                match(dyn_cast){
                    Ok(html_element)=>Some(html_element),
                    Err(element)=>Option::None
                }
            },
            None=>None
        }
    }
}
