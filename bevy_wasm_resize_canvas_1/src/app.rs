use bevy::{prelude::*, render::render_graph::Stages};
mod resize_screen;

pub fn run(){
    let mut app=App::build();

    app
    .insert_resource(WindowDescriptor{
        title:"My Bevy App".to_string(),
        width:320.,
        height:600.,
        #[cfg(target_arch="wasm32")]
        canvas:Some("canvas".to_string()),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(resize_screen::prelude::ResizeScreenPlugin);
    
    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}
