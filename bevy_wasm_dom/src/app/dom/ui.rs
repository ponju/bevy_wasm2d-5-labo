use std::default;

use bevy::prelude::*;

use super::plugin::{ForSelector, ForSlider, ForTextbox};

pub fn generate_ui(mut commands:Commands,mut color_materials:ResMut<Assets<ColorMaterial>>,loader:Res<AssetServer>){
    let font=loader.load("fonts/PixelMplus10-Bold.ttf");
    let text_style=TextStyle{
        color:Color::WHITE,
        font:font.clone(),
        font_size:48.
    };
    let element_style=Style{
        margin:Rect::<Val>{
            left:Val::Px(8.),
            right:Val::Px(8.),
            top:Val::Px(8.),
            bottom:Val::Px(8.),
        }
        ,..Default::default()
    };
    commands.insert_resource(text_style.clone());
    commands.insert_resource(Time::default());
    commands.insert_resource(Timer::from_seconds(10.,true));
    commands.spawn_bundle(ImageBundle{
        material:color_materials.add(ColorMaterial::color(Color::SEA_GREEN)),
        style:
            Style{
                display:Display::Flex,
                justify_content:JustifyContent::Center,
                align_content:AlignContent::Center,
                align_items:AlignItems::Center,
                flex_wrap:FlexWrap::Wrap,
                position_type:PositionType::Absolute,
                position:Rect::<Val>{
                    top:Val::Percent(15.),
                    bottom:Val::Percent(15.),
                    left:Val::Px(0.),
                    right:Val::Px(0.),
                },
                ..Default::default()
            },
            ..Default::default()
    }).with_children(
        |builder|{
            builder
            .spawn_bundle(
                TextBundle{
                    text:Text{
                        alignment:TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
                        sections:vec![
                            TextSection{
                                style:Default::default(),
                                value:Default::default()
                            }
                        ]
                    },
                    style:element_style.clone(),
                    ..Default::default()
                }
            ).insert(ForTextbox);
            builder.spawn_bundle(
                TextBundle{
                    text:Text{
                        alignment:TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
                        sections:vec![
                            TextSection{
                                style:Default::default(),
                                value:Default::default()
                            }
                        ]
                    },
                    style:element_style.clone(),
                    ..Default::default()
                }
            ).insert(ForSlider);
            builder.spawn_bundle(
                TextBundle{
                    text:Text{
                        alignment:TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
                        sections:vec![
                            TextSection{
                                style:Default::default(),
                                value:Default::default()
                            }
                        ]
                    },
                    style:element_style.clone(),
                    ..Default::default()
                }
            ).insert(ForSelector);
        }
    );
}