use bevy::{input::*, prelude::*};
use bevy::math::vec3;
use bevy::prelude::KeyCode::{KeyA, KeyD, KeyS, KeyW, ShiftLeft};
use rand::Rng;
use crate::assets_loader::TextureGameAssets;
use crate::config::*;

pub struct PlayerPlugin;

#[derive(Component)]
struct PlayerCollider;

#[derive(Component)]
pub struct Player{
    pub health: i32,
    stamina: f32,
    check_run: bool,
}
#[derive(Resource, Default)]
struct CurrentPlayerState(PlayerState);
#[derive(Resource)]
struct PlayerSpriteIndex(usize);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Component,Default, PartialEq, Debug)]
enum PlayerState {
    #[default]
    Alive,
    IdleFront,
    IdleBack,
    IdleLeft,
    IdleRight,
    Walk,
    WalkFront,
    WalkBack,
    WalkLeft,
    WalkRight,
    Run,
    RunFront,
    RunBack,
    RunLeft,
    RunRight,
    DeadFront,
    DeadBack,
    DeadLeft,
    DeadRight,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerSpriteIndex(0))
            .insert_resource(CurrentPlayerState::default())
            .add_systems(PostStartup,
                player_spawn)
            .add_systems(Update,(
                animate_sprite,
                camera_follow_player.after(move_player),
                move_player,
                player_position_mapping,
                player_stamina_health,
            ))
        ;
    }
}
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize
}

fn player_spawn(
    mut commands: Commands,
    handler : Res<TextureGameAssets>,
    mut texture_atlas: ResMut<Assets<TextureAtlasLayout>>
){
    println!("Player Spawned");

    let texture = handler.player_handler.clone();
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(SPRITE_SCALE_FACTOR), 8, 16, None, None);
    let texture_atlas = texture_atlas.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    commands.spawn((
        SpriteBundle {//rand::thread_rng().gen_range(0.0..100.0),rand::thread_rng().gen_range(0.0..100.0) -> random x y pos
            transform: Transform::from_translation(vec3(10000.0,10000.0,1.0)).with_scale(Vec3::splat(2.5)),
            texture : texture.unwrap(),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas,
            index: animation_indices.first,
        },
        Player{
            health:100,
            stamina:100.0,
            check_run: true,
        },
        AnimationTimer(Timer::from_seconds(ANIMATION_INTERVAL, TimerMode::Repeating)),
    ));

    commands.spawn(TextBundle::from_sections([
        TextSection::new("{ x : ",TextStyle{font_size:14.5,..default()}),
        TextSection::new("",TextStyle{font_size:14.5,..default()}),
        TextSection::new(",",TextStyle{font_size:14.5,..default()}),
        TextSection::new("y : ",TextStyle{font_size:14.5,..default()}),
        TextSection::new("",TextStyle{font_size:14.5,..default()}),
        TextSection::new("| {Health: ",TextStyle{font_size:14.5,..default()}),
        TextSection::new("",TextStyle{font_size:14.5,..default()}),
        TextSection::new("| {Stamina: ",TextStyle{font_size:14.5,..default()}),
        TextSection::new("",TextStyle{font_size:14.5,..default()}),

    ]).with_style(
        Style{
            position_type: PositionType::Absolute,
            ..default()
        }
    ));

}
fn player_stamina_health
(
    mut query : Query<&mut Text>,
    player_query : Query<&Player>,
){
    let mut position_text = query.single_mut();
    position_text.sections[6].value = player_query.single().health.to_string()+" } ";
    position_text.sections[8].value = (player_query.single().stamina as i32).to_string()+" } ";
}
fn player_position_mapping(
    mut query : Query<&mut Text>,
    player_query : Query<&Transform,With<Player>>
){
    let transform = player_query.single();
    let mut position_text = query.single_mut();
    position_text.sections[1].value = (transform.translation.x as i32).to_string();
    position_text.sections[4].value = (transform.translation.y as i32).to_string()+" } ";
}
fn camera_follow_player(
    query_player : Query<&Transform,With<Player>>,
    mut query_camera : Query<&mut Transform,(Without<Player>,With<Camera>)>
){
    let player_transform = query_player.single();
    let mut camera_transform = query_camera.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
fn move_player(
    mut player_query: Query<(&mut Player,&mut Transform),With<Player>>  ,
    mut player_state: ResMut<CurrentPlayerState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
){
    for (mut player_ent,mut transform) in &mut player_query.iter_mut(){
        if !player_state.is_dead() {

            let mut is_moving = false;

            if player_ent.check_run && keyboard.pressed(ShiftLeft) {
                if keyboard.pressed(KeyW){
                    transform.translation.y += SPECIAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::RunFront);
                    is_moving = true;
                }
                if keyboard.pressed(KeyS) {
                    transform.translation.y -= SPECIAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::RunBack);
                    is_moving = true;
                }
                if keyboard.pressed(KeyD) {
                    transform.translation.x += SPECIAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::RunRight);
                    is_moving = true;
                }
                if keyboard.pressed(KeyA) {
                    transform.translation.x -= SPECIAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::RunLeft);
                    is_moving = true;
                }
                player_ent.stamina -= 10.0*time.delta_seconds();
                println!("stamina reduced: {}",player_ent.stamina);
                if player_ent.stamina <= 0.0 {
                    player_ent.check_run = false;
                }
            } else  {
                if keyboard.pressed(KeyW) {
                    transform.translation.y += NORMAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::WalkFront);
                    is_moving = true;
                }
                if keyboard.pressed(KeyS) {
                    transform.translation.y -= NORMAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::WalkBack);
                    is_moving = true;
                }
                if keyboard.pressed(KeyD) {
                    transform.translation.x += NORMAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::WalkRight);
                    is_moving = true;
                }
                if keyboard.pressed(KeyA) {
                    transform.translation.x -= NORMAL_SPEED * time.delta_seconds();
                    player_state.set_state(PlayerState::WalkLeft);
                    is_moving = true;
                }

                if player_ent.stamina < 100.0 {
                    player_ent.stamina += 5.0*time.delta_seconds();
                    println!("stamina: {}",player_ent.stamina)
                }
                if player_ent.stamina >= 25.0{
                    player_ent.check_run = true;
                }
            }
            //println!("Player Status: {:?}",player_state.0);
            if !is_moving {
                match player_state.0 {
                    PlayerState::WalkBack | PlayerState::RunBack => {
                        player_state.set_state(PlayerState::IdleBack);
                    }
                    PlayerState::WalkFront | PlayerState::RunFront=> {
                        player_state.set_state(PlayerState::IdleFront);
                    }
                    PlayerState::WalkRight | PlayerState::RunRight => {
                        player_state.set_state(PlayerState::IdleRight);
                    }
                    PlayerState::WalkLeft | PlayerState::RunLeft => {
                        player_state.set_state(PlayerState::IdleLeft);
                    }
                    _ => {}
                }
            }
        }
    }
}
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas),With<Player>>,
    player_state: Res<CurrentPlayerState>
) {
    for ( mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());

        if player_state.0 == PlayerState::IdleBack  || player_state.0 == PlayerState::Alive{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8
            }
        }
        if player_state.0 == PlayerState::IdleFront{
            if timer.just_finished() {
                atlas.index = (atlas.index+1)%8 + 8
            }
        }
        if player_state.0 == PlayerState::IdleLeft {
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 16
            }
        }
        if player_state.0 == PlayerState::IdleRight {
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 24
            }
        }
        if player_state.0 == PlayerState::WalkBack || player_state.0 == PlayerState::RunBack{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 32
            }
        }
        if player_state.0 == PlayerState::WalkFront || player_state.0 == PlayerState::RunFront{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 40
            }
        }
        if player_state.0 == PlayerState::WalkLeft || player_state.0 == PlayerState::RunLeft{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 48
            }
        }
        if player_state.0 == PlayerState::WalkRight || player_state.0 == PlayerState::RunRight{
            if timer.just_finished(){
                atlas.index = (atlas.index+1)%8 + 56
            }
        }
    }
}

impl CurrentPlayerState {
    fn is_alive(&self) -> bool {
        self.0 == PlayerState::Alive
    }
    fn is_walking(&self) -> bool {
        match self.0 {
            PlayerState::Walk => true,
            PlayerState::WalkBack => true,
            PlayerState::WalkFront => true,
            PlayerState::WalkLeft => true,
            PlayerState::WalkRight => true,
            _ => false
        }
    }
    fn is_dead(&self) -> bool{
        match self.0{
            PlayerState::DeadFront => true,
            PlayerState::DeadBack => true,
            PlayerState::DeadLeft => true,
            PlayerState::DeadRight => true,
            _ => false
        }
    }
    fn is_running(&self) -> bool {
        match self.0 {
            PlayerState::Run => true,
            PlayerState::RunBack => true,
            PlayerState::RunFront => true,
            PlayerState::RunLeft => true,
            PlayerState::RunRight => true,
            _ => false
        }
    }
    fn set_state(&mut self,new_state: PlayerState){
        self.0 = new_state;
    }
}