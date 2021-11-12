use std::time::Duration;

use bevy::{prelude::{*}, text::Text2dSize, ui::widget::Image};

pub struct  SceneBoard;
pub struct  SceneHeader;
pub struct SceneDescription;

pub fn setup(mut commands:Commands,mut color_materials:ResMut<Assets<ColorMaterial>>,loader:Res<AssetServer>){
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
        material:color_materials.add(ColorMaterial::color(Color::RED)),
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
    }).insert(SceneBoard).with_children(
        |builder|{
            builder
            .spawn_bundle(
                TextBundle{
                    text:Text{
                        alignment:TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).insert(SceneDescription);
            builder.spawn_bundle(
                TextBundle{
                    text:Text{
                        alignment:TextAlignment { vertical: VerticalAlign::Center, horizontal: HorizontalAlign::Center },
                        ..Default::default()
                    },
                    ..Default::default()
                }
            ).insert(SceneHeader);
        }
    );
}