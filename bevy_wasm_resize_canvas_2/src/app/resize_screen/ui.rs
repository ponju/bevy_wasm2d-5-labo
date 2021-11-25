use std::default;

use bevy::prelude::*;

pub struct BaseBoard;
pub struct Square;
pub struct Resolution;

pub fn generate_ui(mut commands:Commands,mut color_materials:ResMut<Assets<ColorMaterial>>,loader:Res<AssetServer>){
    let font=loader.load("fonts/PixelMplus10-Bold.ttf");
    let text_style=TextStyle{
        color:Color::WHITE,
        font:font.clone(),
        font_size:48.
    };
    commands.insert_resource(text_style.clone());
    commands.insert_resource(Time::default());
    commands.insert_resource(Timer::from_seconds(10.,true));
    commands.spawn_bundle(ImageBundle{
        material:color_materials.add(ColorMaterial::color(Color::BLACK)),
        style:
            Style{
                display:Display::Flex,
                justify_content:JustifyContent::Center,
                align_content:AlignContent::Center,
                align_items:AlignItems::Center,
                flex_wrap:FlexWrap::Wrap,
                position_type:PositionType::Absolute,
                position:Rect::<Val>{
                    top:Val::Px(0.),
                    bottom:Val::Px(0.),
                    left:Val::Px(0.),
                    right:Val::Px(0.),
                },
                ..Default::default()
            },
            ..Default::default()
    }).insert(BaseBoard)
    .with_children(
        |builder|{
            builder
            .spawn_bundle(
                ImageBundle{
                    material:color_materials.add(ColorMaterial::color(Color::RED)),
                    style:
                        Style{
                            size:Size::<Val>{
                                width:Val::Px(32.),
                                height:Val::Px(32.)
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    }
            ).insert(Square);
            builder.spawn_bundle(
                TextBundle{
                    text:Text{
                        alignment:TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
                        sections:vec![
                            TextSection{
                                style:Default::default(),
                                value:"RESOLUTION\n".to_string()
                            }
                        ]
                    },
                    ..Default::default()
                }
            ).insert(Resolution);
        }
    );
}