use crate::utils::math::*;

pub const BASE_VOLUME: f32 = 1.0 / 4.0;
pub const CHANNEL_COUNT: u32 = 3;
pub const CH_SHOT: u32 = 0;
pub const CH_BG_MUSIC: u32 = 1;
pub const CH_KILL: u32 = 2;

pub const PADDING: i32 = 16;

pub const WINDOW_WIDTH: i32 = 320 * 3;
pub const WINDOW_HEIGHT: i32 = 256 * 3;

pub const GAME_WIDTH: i32 = 192 * 3;
pub const GAME_HEIGHT: i32 = WINDOW_HEIGHT - (PADDING * 2);

pub const CENTER_X: i32 = GAME_WIDTH * ONE / 2;
pub const PLAYER_Y: i32 = (GAME_HEIGHT - 16 - 8) * ONE;

pub const APP_NAME: &str = "Teki";
pub const PLAYER_SPEED: i32 = 8 * ONE;
pub const MYSHOT_SPEED: i32 = 12 * ONE;
pub const FPS: u32 = 60;

pub const X_COUNT: usize = 4;
pub const Y_COUNT: usize = 3;

pub const BASE_Y: i32 = 75;

pub const FONTS: &str = "font";

pub const BG_TEXTURE: &str = "blue";

pub const PLAYER_SPRITE: &str = "hero1";
pub const ENEMY_SPRITE: &str = "enemy1";
pub const BULLET_SPRITE: &str = "orb_blue_full";

pub const BUBBLE_SOUND: &str = "./assets/audio/bubble";
pub const BG_MUSIC: &str = "./assets/audio/loop";
pub const SE_KILL: &str = "./assets/audio/pop";
