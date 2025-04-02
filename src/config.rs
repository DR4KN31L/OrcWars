//-------------------------------------------------------------------------
// Player Speed/Animation Configuration
pub const NORMAL_SPEED: f32 = 450.0;
pub const SPECIAL_SPEED:f32 = 800.0;
//--------------------------------------------------------------------------
// Enemy Speed
pub const ENEMY_SPEED:f32 = 6.35;
//--------------------------------------------------------------------------
// SPRITE PACKS & SCALE FACTOR
pub const PLAYER_SPRITE_PACK: &str = "player_sprite_pack.png";
pub const ENEMY_SPRITE_PACK: &str = "ememy_sprite_pack.png";
pub const WATER_PACK: &str = "water_full.png";
pub const TERRAIN_PACK: &str = "terrain.png";
pub const DETAILS_PACK : &str = "nature.png";
pub const SPRITE_SCALE_FACTOR:u32 = 64;
pub const ANIMATION_INTERVAL: f32 = 0.1;
//-------------------------------------------------------------------------
// Window
pub const HEIGHT:f32 = 820f32;
pub const WIDTH:f32 = 1024f32;
//-------------------------------------------------------------------------
// World Generation
pub const MAP_ROWS: usize = 250;
pub const MAP_COLUMNS: usize = 250;
pub const MAP_SCALE_FACTOR: f32 = 2.5;
pub const MAP_OFFSET: f64 = 10.5;
//-------------------------------------------------------------------------
// Spawner
pub const SPAWN_INTERVAL: f32 = 2f32;
pub const SPAWN_TIME:usize = 10;
pub const MAX_ENEMIES:usize = 300;