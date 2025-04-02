use std::cmp::PartialEq;
use std::f32;
use bevy::math::vec3;
use std::f32::consts::PI;
use std::fmt::Debug;
use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use crate::assets_loader::TextureGameAssets;
use crate::config::{ANIMATION_INTERVAL, ENEMY_SPEED, MAX_ENEMIES, SPAWN_INTERVAL, SPAWN_TIME, SPRITE_SCALE_FACTOR};
use crate::player::Player;
pub struct EnemyPlugin;


#[derive(Resource)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
}
#[derive(Component)]
pub struct Enemy;
#[derive(Component,PartialEq,Debug,Default)]
enum EnemyType{
    #[default]
    Slave,
    MiniBoss,
}

#[derive(Resource,Default)]
struct EnemyCurrentState(EnemyState);

#[derive(Component,Default, PartialEq, Debug)]
enum EnemyState {
    #[default]
    Alive,
    IdleFront,
    IdleBack,
    IdleLeft,
    IdleRight,
    RunFront,
    RunBack,
    RunLeft,
    RunRight,
    Dead,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app_si: &mut App) {
        app_si.insert_resource(GlobalTextureAtlas::default())
            .insert_resource(EnemyCurrentState::default())
            .add_systems(Startup,load_assets)
            .add_systems(Update, (
                spawn_enemy.run_if(on_timer(Duration::from_secs_f32(SPAWN_INTERVAL))),
                animate_enemies,
                follow_player
            ))
        ;
    }
}

fn spawn_enemy(
    mut commands: Commands,
    player_query : Query<&Transform,With<Player>>,
    enemy_query: Query<&Transform,(With<Enemy>,Without<Player>)>,
    handler : Res<TextureGameAssets>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {

    let total_enemies_spawned = enemy_query.iter().len();
    let enemy_spawn_count = (MAX_ENEMIES-total_enemies_spawned).min(SPAWN_TIME);

    if total_enemies_spawned >= MAX_ENEMIES || player_query.is_empty(){
        return;
    }

    for _ in 0..enemy_spawn_count {

        let player_pos = player_query.single().translation.truncate();
        let (x,y) = get_random_pos(player_pos);
        let enemy_type = EnemyType::generate_random_enemy();
        let enemy_state = EnemyState::default();
        let texture = handler.enemy_handler.clone();
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(SPRITE_SCALE_FACTOR),8,32,None,None);
        let texture_atlas_layout = texture_atlas_layout.add(layout);

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(vec3(x,y,0.0)).with_scale(Vec3::splat(2.5)),
                texture: texture.unwrap(),
                ..default()
            },
            TextureAtlas {
                layout: texture_atlas_layout,
                index: enemy_type.get_enemy_index(),
            },
            Enemy,
            enemy_type,
            enemy_state,
            crate::AnimationTimer(Timer::from_seconds(ANIMATION_INTERVAL, TimerMode::Repeating)),
        ));
    }
    println!("Total enemy spawned: {}",total_enemies_spawned);
}

fn animate_enemies(
    time : Res<Time>,
    mut query : Query<(&mut crate::AnimationTimer, &mut TextureAtlas, &EnemyType, &EnemyState),(With<Enemy>,Without<Player>)>,
){

    for (mut timer, mut atlas, enemy_type, enemy_state) in &mut query {
        timer.tick(time.delta());
        if (enemy_state == &EnemyState::IdleFront  || enemy_state == &EnemyState::Alive) && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8
            }
        }
        if enemy_state == &EnemyState::IdleBack && enemy_type.is_slave(){
            if timer.just_finished() {
                atlas.index = (atlas.index+1)%8 + 8
            }
        }
        if enemy_state == &EnemyState::IdleLeft && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 16
            }
        }
        if enemy_state == &EnemyState::IdleRight && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 24
            }
        }
        if enemy_state == &EnemyState::RunFront && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 32
            }
        }
        if enemy_state == &EnemyState::RunBack && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 40
            }
        }
        if enemy_state == &EnemyState::RunLeft && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 48
            }
        }
        if enemy_state == &EnemyState::RunRight && enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 56
            }
        }
        if (enemy_state == &EnemyState::IdleFront  || enemy_state == &EnemyState::Alive) && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 128
            }
        }
        if enemy_state == &EnemyState::IdleBack && !enemy_type.is_slave(){
            if timer.just_finished() {
                atlas.index = (atlas.index+1)%8 + 136
            }
        }
        if enemy_state == &EnemyState::IdleLeft && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 144
            }
        }
        if enemy_state == &EnemyState::IdleRight && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 152
            }
        }
        if enemy_state == &EnemyState::RunFront && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 160
            }
        }
        if enemy_state == &EnemyState::RunBack && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 168
            }
        }
        if enemy_state == &EnemyState::RunLeft && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 176
            }
        }
        if enemy_state == &EnemyState::RunRight && !enemy_type.is_slave(){
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 184
            }
        }
    }
}
impl EnemyCurrentState{
    fn set_state(&mut self,new_state : EnemyState){
        self.0 = new_state;
    }
}

impl EnemyType {
    fn generate_random_enemy() -> Self{
        let mut rand = rand::thread_rng();
        let random_enemy = rand.gen_range(0..4);
        return match random_enemy {
            0 => EnemyType::MiniBoss,
            _ => EnemyType::Slave
        };
    }
    fn get_enemy_index(&self) -> usize{
        match self {
            EnemyType::Slave => 0,
            EnemyType::MiniBoss => 127
        }
    }
    fn is_slave(&self) -> bool {
        match self {
            EnemyType::Slave => true,
            _ => false
        }
    }
}

impl Default for GlobalTextureAtlas {
    fn default() -> Self {
        Self {
            layout: None,
        }
    }
}

fn get_random_pos(pos: Vec2) -> (f32,f32){
    let mut rng = rand::thread_rng();
    let angle = rng.gen_range(0.0..PI * 2.0);
    let dist = rng.gen_range(1000.0..5000.0);

    let offset_x = angle.cos() * dist;
    let offset_y = angle.sin() * dist;

    let random_x = pos.x + offset_x;
    let random_y = pos.y + offset_y;

    (random_x, random_y)
}

fn load_assets(
    mut handle: ResMut<GlobalTextureAtlas>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(SPRITE_SCALE_FACTOR), 8, 32, None, None);
    handle.layout = Some(texture_atlas_layouts.add(layout));
}

fn follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut EnemyState), (With<Enemy>,Without<Player>)>,
) {
    if player_query.is_empty() {
        return;
    }
    if enemy_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation;

    for (mut enemy_transform, mut enemy_state) in &mut enemy_query {
        let direction = (player_position - enemy_transform.translation).normalize();
        let distance = distance(player_position, enemy_transform.translation);

        let enemy_player_vector = player_position - enemy_transform.translation;
        let angle = f32::to_degrees(f32::atan2(enemy_player_vector.y, enemy_player_vector.x));

        let angle = if angle < 0.0 { angle + 360.0 } else { angle };


        if distance < 375.0 {
            if 0.0 <= angle && 25f32 >= angle || 335f32 <= angle && 360f32 >= angle {
                *enemy_state = EnemyState::RunRight;
            }
            if 26f32 <= angle && 155f32 >= angle{
                *enemy_state = EnemyState::RunBack;
            }
            if 156f32 <= angle && 205f32 >= angle {
                *enemy_state = EnemyState::RunLeft;
            }
            if 206f32 <= angle && 334f32 >= angle{
                *enemy_state = EnemyState::RunFront;
            }
            enemy_transform.translation += direction * ENEMY_SPEED;
        } else {
            *enemy_state = EnemyState::IdleFront;
        }
    }
}

fn distance(player: Vec3,enemy: Vec3) -> f32 {
    let distance_x = enemy.x - player.x;
    let distance_y = enemy.y - player.y;

    (distance_x.powi(2)+distance_y.powi(2)).sqrt()
}
