use bevy::prelude::*;
use std::{borrow::BorrowMut, cell::{RefCell, RefMut}, fmt::Error, rc::Rc};

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

        //JSに関数を登録
        unsafe{
            WINDOWS=Some(Windows::default());
            #[cfg(target_arch="wasm32")]
            app.world_mut().remove_resource::<Windows>();
            app.world_mut().insert_resource::<Windows>(Windows{..WINDOWS.()});
        }
        app.add_startup_stage("setup_enviroment",
        SystemStage::single_threaded().with_system_set(
            SystemSet::new().with_system(enviroment::camera_setup.system()).with_system(ui::generate_ui.system())
        ));
        app.add_startup_stage_after("setup_enviroment","register_hook", 
        SystemStage::single_threaded().with_system(register_resize_closure.system())
        );
    }
}

pub static mut WINDOWS:Option<Windows>=None;

#[cfg(target_arch="wasm32")]
fn register_resize_closure(mut commands:Commands){
    use wasm_bindgen::prelude::Closure;

    let canvas=window().expect("No browser").document().expect("No document").find_by_id::<HtmlElement>("bevy_app").expect("No element");
    unsafe{
        let closure=Closure::wrap(Box::new(move||{
            WINDOWS.as_mut().expect("No static windows").get_primary_mut().expect("No primary window").set_resolution(canvas.offset_width() as f32, canvas.offset_height() as f32)
        })as Box<dyn FnMut()>);
        closure.forget();
    }
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
