use bevy::{prelude::*, render::render_graph::Stages};

mod layout; // app配下に layoutモジュールを作っています。

pub fn run(){
    let mut app=App::build();

    app
    .insert_resource(WindowDescriptor{
        title:"My Bevy App".to_string(),
        width:375.,
        height:600.,
        #[cfg(target_arch="wasm32")]
        canvas:Some("canvas".to_string()),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_startup_system(camera_setup.system())
    .add_plugin(layout::layout_plugin::AppLayoutPlugin);
    
    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    app.run();
}

fn camera_setup(mut commands:Commands){
    commands.insert_resource(ClearColor(Color::rgb(1.,1.,1.)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
