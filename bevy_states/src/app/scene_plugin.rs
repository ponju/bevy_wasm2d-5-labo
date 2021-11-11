use bevy::{core::{Time, Timer}, prelude::{Entity, IntoSystem, Plugin, Query, Res, ResMut, State, SystemSet, With}, text::{ Text, TextSection, TextStyle}};

use super::setup::{self, SceneDescription, SceneHeader};

pub struct PrototypeScenePlugin;
impl Plugin for PrototypeScenePlugin{
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
            .add_startup_system(setup::setup.system())
            .add_state(AppStates::Setup)
            //Setup sequence
            .add_system_set(SystemSet::on_enter(AppStates::Setup).with_system(on_setup_entry.system()))
            .add_system_set(SystemSet::on_update(AppStates::Setup).with_system(on_setup_process.system()))
            .add_system_set(SystemSet::on_exit(AppStates::Setup).with_system(on_setup_exit.system()))
            //State A
            .add_system_set(SystemSet::on_enter(AppStates::A).with_system(on_stateA_entered.system()))
            .add_system_set(SystemSet::on_update(AppStates::A).with_system(on_stateA_process.system()))
            .add_system_set(SystemSet::on_exit(AppStates::A).with_system(on_stateA_exit.system()))
            //State B
            .add_system_set(SystemSet::on_enter(AppStates::B).with_system(on_stateB_entered.system()))
            .add_system_set(SystemSet::on_update(AppStates::B).with_system(on_stateB_process.system()))
            .add_system_set(SystemSet::on_exit(AppStates::B).with_system(on_stateB_exit.system()));
    }
}

#[derive(Debug,Clone,Eq,PartialEq,Hash)]
pub enum AppStates{
    Setup,A,B
}

pub fn on_setup_entry(
    mut header_q:Query<(Entity,&mut Text),With<SceneHeader>>,
    mut timer:ResMut<Timer>,
    ts :Res<TextStyle>
){
    eprintln!("Setup sequnce start!!");
    timer.reset();
        
    match header_q.single_mut(){
        Err(_)=>eprintln!("No header text element in the board."),
        Ok((entity,mut text))=>{text.sections=vec![
            TextSection{
                value:"Setup Sequence".to_string(),
                style:ts.clone()
            }
        ]}
    }
}

pub fn on_setup_process(
    mut description_q:Query<(Entity,&mut Text),With<SceneDescription>>,
    mut states:ResMut<State<AppStates>>,
    mut time:ResMut<Time>,
    mut timer:ResMut<Timer>,
    ts :Res<TextStyle>
){       
    match description_q.single_mut(){
        Err(_)=>eprintln!("No header text element in the board."),
        Ok((entity,mut text))=>{
            let time_text=format!("Time:{:.2}",timer.elapsed_secs());
            text.sections=vec![
            TextSection{
                value:time_text,
                style:ts.clone()
            }
        ]}
    }
    timer.tick(time.delta());
    if timer.elapsed().as_secs()>=5{
        states.set(AppStates::A);
    }
}
pub fn on_setup_exit(){
    eprintln!("Setup finished.")
}

pub fn on_stateA_entered(
    mut header_q:Query<(Entity,&mut Text),With<SceneHeader>>,
    mut timer:ResMut<Timer>,
    ts :Res<TextStyle>
){
    eprintln!("A sequnce start!!");
    timer.reset();
        
    match header_q.single_mut(){
        Err(_)=>eprintln!("No header text element in the board."),
        Ok((entity,mut text))=>{text.sections=vec![
            TextSection{
                value:"State A".to_string(),
                style:ts.clone()
            }
        ]}
    }
}

pub fn on_stateA_process(
    mut description_q:Query<(Entity,&mut Text),With<SceneDescription>>,
    mut states:ResMut<State<AppStates>>,
    mut time:ResMut<Time>,
    mut timer:ResMut<Timer>,
    ts :Res<TextStyle>
){       
    match description_q.single_mut(){
        Err(_)=>eprintln!("No SceneDescription text element in the board."),
        Ok((entity,mut text))=>{
            let time_text=format!("Time:{:.2}",timer.elapsed_secs());
            text.sections=vec![
            TextSection{
                value:time_text,
                style:ts.clone()
            }
        ]}
    }
    timer.tick(time.delta());
    if timer.elapsed().as_secs()>=5{
        states.set(AppStates::B);
    }
}
pub fn on_stateA_exit(){
    eprintln!("A finished.")
}

pub fn on_stateB_entered(
    mut header_q:Query<(Entity,&mut Text),With<SceneHeader>>,
    mut timer:ResMut<Timer>,
    ts :Res<TextStyle>
){
    eprintln!("B sequnce start!!");
    timer.reset();
        
    match header_q.single_mut(){
        Err(_)=>eprintln!("No header text element in the board."),
        Ok((entity,mut text))=>{text.sections=vec![
            TextSection{
                value:"State B".to_string(),
                style:ts.clone()
            }
        ]}
    }
}

pub fn on_stateB_process(
    mut description_q:Query<(Entity,&mut Text),With<SceneDescription>>,
    mut states:ResMut<State<AppStates>>,
    mut time:ResMut<Time>,
    mut timer:ResMut<Timer>,
    ts :Res<TextStyle>
){       
    match description_q.single_mut(){
        Err(_)=>eprintln!("No SceneDescription text element in the board."),
        Ok((entity,mut text))=>{
            let time_text=format!("Time:{:.2}",timer.elapsed_secs());
            text.sections=vec![
            TextSection{
                value:time_text,
                style:ts.clone()
            }
        ]}
    }
    timer.tick(time.delta());
    if timer.elapsed().as_secs()>=5{
        states.set(AppStates::A);
    }
}
pub fn on_stateB_exit(){
    eprintln!("B finished.")
}
