mod player;
mod config;
mod terrain;
mod enemy;
mod assets_loader;

use::bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::player::PlayerPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use crate::assets_loader::AssetsLoaderPlugin;
use crate::config::{HEIGHT, WIDTH};
use crate::enemy::EnemyPlugin;
use crate::terrain::TerrainPlugin;

#[derive(Component,Deref,DerefMut)]
struct AnimationTimer(Timer);
#[derive(Default,Clone,Copy,Debug,Eq,PartialEq,Hash,States)]
enum GameStatus{
    #[default]
    Started,
    Paused,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin{
                primary_window: Some(Window{
                    title: "Orc Wars!".to_string(),
                    resizable:true,
                    focused:true,
                    resolution:(WIDTH,HEIGHT).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(PanCamPlugin::default())
        .add_plugins(AssetsLoaderPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(PostStartup,spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default())
    ;
}