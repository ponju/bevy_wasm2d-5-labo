use std::fmt::Error;

use bevy::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
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
    mut text_q:Query<&mut Text,(With<ForTextbox>,Without<ForSlider>,Without<ForSelector>)>,
    mut slider_q:Query<&mut Text,(With<ForSlider>,Without<ForTextbox>,Without<ForSelector>)>,
    mut selector_q:Query<&mut Text,(With<ForSelector>,Without<ForTextbox>,Without<ForSlider>)>,
    text_style:Res<TextStyle>
){
    let mut doc=window().unwrap().document().unwrap();
    match text_q.single_mut(){
        Ok(mut text)=>{
            let element=doc.find_by_id::<HtmlInputElement>("text_box").expect("No dom elements exist indexed as text_box by id");
            text.sections[0]=TextSection{
                style:text_style.clone(),
                value:element.value()
            }
        },
        Err(_)=>{eprintln!("No text for textbox in app-uis")}
    };
    match slider_q.single_mut(){
        Ok(mut text)=>{
            let element=doc.find_by_id::<HtmlInputElement>("slider").expect("No dom elements exist indexed as slider by id");
            text.sections[0]=TextSection{
                style:text_style.clone(),
                value:element.value()
            }
        },
        Err(_)=>{eprintln!("No text for slider in app-uis")}
    };
    match selector_q.single_mut(){
        Ok(mut text)=>{
            let element=doc.find_by_id::<HtmlSelectElement>("selector").expect("No dom elements exist indexed as selector by id");
            text.sections[0]=TextSection{
                style:text_style.clone(),
                value:element.value().to_string()
            }
        },
        Err(_)=>{eprintln!("No text for selector in app-uis")}
    };
}

trait DocumentExpand{
    fn find_by_id<T>(self:&Self,id:&str) ->Option<T> where T:JsCast;
}

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

pub struct  ForTextbox;
pub struct  ForSlider;
pub struct ForSelector;
