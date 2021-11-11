use bevy::{prelude::{*}, text::Text2dSize};

pub fn generate_text_ui(mut commands:Commands,loader:Res<AssetServer>){
    commands.spawn_bundle(Text2dBundle{
        text:Text{
            sections:vec![
                TextSection{
                    value:"hello".to_string(),
                    style:TextStyle{
                        color:Color::ORANGE,
                        font:loader.load("fonts/PixelMplus10-Bold.ttf"),
                        font_size:48.,
                    }
                }
            ],
            ..Default::default()
        },
        text_2d_size:Text2dSize{
            size:Size{width:300.,height:200.},
        },
        ..Default::default()
    });
}