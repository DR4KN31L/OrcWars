use std::vec::Vec;
use bevy::math::{vec3};
use bevy::prelude::*;
use bevy::utils::HashSet;
use noise::{NoiseFn, Perlin};
use rand::{Rng, thread_rng};
use crate::assets_loader::{TextureGameAssets};
use crate::config::{ANIMATION_INTERVAL, MAP_COLUMNS, MAP_OFFSET, MAP_ROWS, MAP_SCALE_FACTOR};

pub struct TerrainPlugin;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
struct Bioma {
    pos:(i32,i32),
    map_texture: usize,
    z_index: i32,
    water_type: WaterType,
    is_water:bool,
}
#[derive(Component,Default,PartialEq,Debug,Clone)]
enum WaterType {
    Deep,
    Normal,
    #[default]
    None
}

impl Bioma {
    fn new(position:(i32,i32), map_texture:usize, z_index:i32,water_type: Option<WaterType>,is_water: Option<bool>) -> Self{
        Self{
            pos:position,
            map_texture,
            z_index,
            water_type: water_type.unwrap_or_default(),
            is_water: is_water.unwrap_or(false),
        }
    }
}
const TILE_WIDTH:u32 = 32; // texture pixel width
const TILE_HEIGHT:u32 = 32; // texture pixel height

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, terrain_setup)
            .add_systems(Update,animate_sprite);
    }
}

fn terrain_setup(
    mut commands: Commands,
    texture_handler: Res<TextureGameAssets>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
){

    let mut random_seed = thread_rng();

    let perlin = Perlin::new(random_seed.gen());

    let mut biomas = Vec::new();
    let mut vege = Vec::new();
    let mut occupied_pos = HashSet::new();

    for x in 0..MAP_ROWS {
        for y in 0..MAP_COLUMNS {

            let pos = (x as i32,y as i32);

            if occupied_pos.contains(&pos){
                continue;
            }


            let noise = {
                let pre_noise = perlin.get([x as f64 / MAP_OFFSET,y as f64 / MAP_OFFSET]);
                let pre_2noise = perlin.get([x as f64 / 45.5 ,y as f64 / 45.5 ]);
                (pre_noise + pre_2noise)/2.0
            };


            if noise >= 0.31  && noise < 0.36{
                let rnd = thread_rng().gen_range(0.0..1.0);
                if rnd > 0.8{
                    let rnd = thread_rng().gen_range(0..=2);
                    vege.push(Bioma::new((pos),rnd,0,None,None));
                }
                biomas.push(Bioma::new(pos, 0, -1, Some(WaterType::Deep),Some(true)));
                occupied_pos.insert(pos);
            }
            if noise >= 0.23 && noise < 0.31 {
                let rnd = thread_rng().gen_range(0.0..1.0);
                if rnd > 0.8{
                    let rnd = thread_rng().gen_range(0..=2);
                    vege.push(Bioma::new((pos),rnd,0,None,None));
                }
                biomas.push(Bioma::new(pos, 4, -1, Some(WaterType::Normal),Some(true)));
                occupied_pos.insert(pos);
            }
            if noise >= 0.36 && noise < 0.5 {
                let rnd = thread_rng().gen_range(0.0..1.0);
                if rnd > 0.8{
                    let rnd = thread_rng().gen_range(4..=13);
                    vege.push(Bioma::new((pos),rnd,0,None,None));
                }
                biomas.push(Bioma::new((x as i32, y as i32), 1, -1, None, None));
                occupied_pos.insert(pos);
            }
            if noise >= 0.5 && noise < 0.64 {
                let rnd = thread_rng().gen_range(0.0..1.0);
                if rnd > 0.8{
                    let rnd = thread_rng().gen_range(14..=15);
                    vege.push(Bioma::new(pos,rnd,0,None,None));
                }
                biomas.push(Bioma::new((x as i32, y as i32), 2, -1,None,None));
                occupied_pos.insert(pos);
            }
            if noise >= 0.64{
                let rnd = thread_rng().gen_range(0.0..1.0);
                if rnd > 0.8{
                    let rnd = thread_rng().gen_range(14..=15);
                    vege.push(Bioma::new(pos,rnd,0,None,None));
                }
                biomas.push(Bioma::new((x as i32, y as i32), 2, -1,None,None));
                occupied_pos.insert(pos);
            }
            if noise < 0.23{
                let rnd = thread_rng().gen_range(0.0..1.0);
                if rnd > 0.7{
                    let rnd = thread_rng().gen_range(4..=13);
                    vege.push(Bioma::new(pos,rnd,0,None,None));
                }
                biomas.push(Bioma::new((x as i32, y as i32), 0, -1, None, None));
                occupied_pos.insert(pos);
            }
        }
    }
    for bioma in biomas.iter(){

        let (x, y) = bioma.pos;
        let (x, y) = grid_to_world(x as f32, y as f32);

        if bioma.is_water == true {

            let layout = TextureAtlasLayout::from_grid(UVec2 { x: TILE_WIDTH, y: TILE_HEIGHT }, 3, 3, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);

            let texture = texture_handler.map_handler.clone();
            commands.spawn(
                (SpriteBundle {
                    transform: Transform::from_translation(vec3(x, y, bioma.z_index as f32)).with_scale(Vec3::splat(MAP_SCALE_FACTOR)),
                    texture: texture.unwrap(),
                    ..default()
                },
                 TextureAtlas {
                     layout: texture_atlas_layout.clone(),
                     index: bioma.map_texture,
                 }, AnimationTimer(Timer::from_seconds(ANIMATION_INTERVAL, TimerMode::Repeating)), bioma.water_type.clone())
            );
        }else {
            let layout = TextureAtlasLayout::from_grid(UVec2 { x: TILE_WIDTH, y: TILE_HEIGHT }, 3, 1, None, None);let texture_atlas_layout = texture_atlas_layout.add(layout);

            let texture = texture_handler.terrain_handler.clone();
            commands.spawn(
                (SpriteBundle {
                    transform: Transform::from_translation(vec3(x, y, bioma.z_index as f32)).with_scale(Vec3::splat(MAP_SCALE_FACTOR)),
                    texture: texture.unwrap(),
                    ..default()
                },
                 TextureAtlas {
                     layout: texture_atlas_layout.clone(),
                     index: bioma.map_texture,
                 }, AnimationTimer(Timer::from_seconds(ANIMATION_INTERVAL, TimerMode::Repeating)))
            );
        }
    }
    for detail in vege.iter() {
        let (x, y) = detail.pos;
        let (x, y) = grid_to_world(x as f32, y as f32);

        let layout = TextureAtlasLayout::from_grid(UVec2 { x: TILE_WIDTH, y: TILE_HEIGHT }, 4, 4, None, None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);

        let texture = texture_handler.details_handler.clone();
        commands.spawn(
            (SpriteBundle {
                transform: Transform::from_translation(vec3(x, y, detail.z_index as f32)).with_scale(Vec3::splat(MAP_SCALE_FACTOR)),
                texture: texture.unwrap(),
                ..default()
            },
             TextureAtlas {
                 layout: texture_atlas_layout.clone(),
                 index: detail.map_texture,
             }, AnimationTimer(Timer::from_seconds(ANIMATION_INTERVAL, TimerMode::Repeating)))
        );
    }
}

fn grid_to_world(x:f32,y:f32)->(f32,f32){
    (x * TILE_WIDTH as f32 * MAP_SCALE_FACTOR,
     y * TILE_HEIGHT as f32 * MAP_SCALE_FACTOR
    )
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &mut WaterType),With<WaterType>>,
) {

    for (mut timer, mut atlas, mut watertype) in &mut query {
        timer.tick(time.delta());
        if *watertype == WaterType::Normal{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%4;
            }
        }
        if *watertype == WaterType::Deep{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%4 +4;
            }
        }
    }
}