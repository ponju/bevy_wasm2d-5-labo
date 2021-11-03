use bevy::{prelude::*, render::render_graph::Stages};

mod actor; // app配下に actorモジュールを作っています。

pub fn run(){
    let mut app=App::build();

    app
    .insert_resource(WindowDescriptor{
        title:"My Bevy App".to_string(),
        width:600.,
        height:375.,
        #[cfg(target_arch="wasm32")]
        canvas:Some("canvas".to_string()),
        ..Default::default()
    })
    .add_startup_system(camera_setup.system())
    .add_plugins(DefaultPlugins);
    
    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);
    
    //2Dキャラクターのセットアップ
    app.add_startup_system(actor::actor2D::spawn_actor.system());
    app.run();
}

fn camera_setup(mut commands:Commands){
    commands.insert_resource(ClearColor(Color::rgb(0.5,0.5,1.)));
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
