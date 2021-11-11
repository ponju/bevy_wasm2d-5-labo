use bevy::{math::{Rect, Size}, prelude::{AssetServer, Assets, BuildChildren, Color, ColorMaterial, Commands, Handle, ImageBundle, Res, ResMut, TextBundle}, text::{Font, Text, TextSection, TextStyle}, ui::{Style, Val}};

pub struct ScoreUI;
pub struct TimeUI;
pub struct ScoreBoard;

pub fn layout(mut commands:Commands,assets:Res<AssetServer>,mut materials:ResMut<Assets<ColorMaterial>>){
    let font_handle:Handle<Font>=assets.load("fonts/PixelMplus10-Regular.ttf");
    let text_style=TextStyle{color:Color::WHITE,font:font_handle.clone(),font_size:32.};

    //残り時間
    let time_text=
    TextBundle{
        text:Text { 
            sections: vec![
                TextSection{
                    style:text_style.clone(),
                    value:"TIME:XX".to_string()
                }                
            ],
            ..Default::default() 
        },
        style:Style{
            margin:Rect::<Val>{
                left:Val::Px(0.),                
                ..Default::default()
            }
            ,..Default::default()
        },
        ..Default::default()
    };
    //スコア
    let score_text=
    TextBundle{
        text:Text { 
            sections: vec![
                TextSection{
                    style:text_style.clone(),
                    value:"SCORE:XXX".to_string()
                }                
            ],
            ..Default::default() 
        },
        style:Style{
            margin:Rect::<Val>{
                right:Val::Px(0.),                
                ..Default::default()
            }
            ,..Default::default()
        },
        ..Default::default()
    };
    //ヘッダー背景のスタイル
    let container=ImageBundle{
        style: Style{
            display:bevy::ui::Display::Flex,
            align_items:bevy::ui::AlignItems::Center,
            justify_content:bevy::ui::JustifyContent::SpaceBetween,
            position_type:bevy::ui::PositionType::Absolute,
            position:Rect::<Val>{
                top:Val::Percent(0.),
                ..Default::default()
            },
            size:Size{
                width:Val::Percent(100.),
                height:Val::Px(64.)
            },
            ..Default::default()
        },
        material: materials.add(ColorMaterial{color:Color::rgba(0.,0.,0.,0.9),..Default::default()}),
        ..Default::default()
    };

    //with_childrenで親子関係に
    commands.spawn_bundle(container).insert(ScoreBoard)
    .with_children(
        |builder|{
            builder.spawn_bundle(score_text).insert(ScoreUI);
            builder.spawn_bundle(time_text).insert(TimeUI);
        }
    );
}