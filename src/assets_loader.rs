use bevy::prelude::*;
use crate::config::{ENEMY_SPRITE_PACK, WATER_PACK, PLAYER_SPRITE_PACK, TERRAIN_PACK, DETAILS_PACK};

#[derive(Resource)]
pub struct TextureGameAssets{
    pub player_handler: Option<Handle<Image>>,
    pub enemy_handler: Option<Handle<Image>>,
    pub map_handler: Option<Handle<Image>>,
    pub terrain_handler: Option<Handle<Image>>,
    pub details_handler: Option<Handle<Image>>,
}

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TextureGameAssets::default())
            .add_systems(Startup,load_assets);
    }
}

impl Default for TextureGameAssets{
    fn default() -> Self {
        Self{
            player_handler: None,
            enemy_handler: None,
            map_handler: None,
            terrain_handler: None,
            details_handler: None,
        }
    }
}

fn load_assets(mut scene_assets: ResMut<TextureGameAssets>,asset_server: Res<AssetServer>){

    *scene_assets = TextureGameAssets{
        player_handler: Some(asset_server.load(PLAYER_SPRITE_PACK)),
        enemy_handler: Some(asset_server.load(ENEMY_SPRITE_PACK)),
        map_handler: Some(asset_server.load(WATER_PACK)),
        terrain_handler: Some(asset_server.load(TERRAIN_PACK)),
        details_handler: Some(asset_server.load(DETAILS_PACK)),
    };
    println!("Assets Loaded!");
}