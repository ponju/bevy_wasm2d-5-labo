use bevy::{prelude::*, render::render_graph::Stages};

mod setup; // app配下に uiモジュールを作っています。
mod scene_plugin;

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
    .add_plugin(scene_plugin::PrototypeScenePlugin)
    .add_startup_system(camera_setup.system());
    
    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}

fn camera_setup(mut commands:Commands){
    commands.insert_resource(ClearColor(Color::rgb(0.,0.,0.)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
