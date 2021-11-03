use bevy::{prelude::{*}};

pub fn spawn_actor(mut commands:Commands,mut material_store:ResMut<Assets<ColorMaterial>>,mut asset_server:ResMut<AssetServer>){
    let texture_handle:Handle<Texture>=asset_server.load("images/bird.png"); //追加

    let material=ColorMaterial{
        color:Color::RED,
        texture:Some(texture_handle)
    };
    let mat_handle=material_store.add(material);

    commands.spawn_bundle(
        SpriteBundle{
            sprite:Sprite{
                resize_mode:SpriteResizeMode::Manual,
                size:Vec2::new(32.,32.),
                ..Default::default()
            },
            material:mat_handle,
            ..Default::default()
        }
    );
}