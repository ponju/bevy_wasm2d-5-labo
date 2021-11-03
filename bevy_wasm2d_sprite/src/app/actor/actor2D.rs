use bevy::prelude::{*};

pub fn spawn_actor(mut commands:Commands,mut material_store:ResMut<Assets<ColorMaterial>>){
    let material=ColorMaterial{
        color:Color::RED,
        ..Default::default()
    };
    let handle=material_store.add(material);

    commands.spawn_bundle(
        SpriteBundle{
            sprite:Sprite{
                resize_mode:SpriteResizeMode::Manual,
                size:Vec2::new(32.,32.),
                ..Default::default()
            },
            material:handle,
            ..Default::default()
        }
    );
}