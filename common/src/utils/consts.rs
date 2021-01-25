use crate::utils::math::*;

pub const BASE_VOLUME: f32 = 1.0 / 6.0;
pub const CHANNEL_COUNT: u32 = 3;
pub const CH_SHOT: u32 = 0;
pub const CH_BG_MUSIC: u32 = 1;
pub const CH_KILL: u32 = 2;

pub const WINDOW_WIDTH: i32 = 700;
pub const WINDOW_HEIGHT: i32 = 256 * 2;

pub const GAME_WIDTH: i32 = 512;
pub const GAME_HEIGHT: i32 = 256 * 2;

pub const CENTER_X: i32 = GAME_WIDTH * ONE / 2;
pub const PLAYER_Y: i32 = (GAME_HEIGHT - 96) * ONE;

pub const APP_NAME: &str = "Teki";
pub const PLAYER_SPEED: i32 = 7 * ONE;
pub const MYSHOT_SPEED: i32 = 12 * ONE;
pub const SHOT_DELAY: u32 = 4;
pub const FPS: u32 = 60;
pub const MIN_FPS: u32 = 15;

pub const X_COUNT: usize = 4;
pub const Y_COUNT: usize = 3;

pub const SCROLLING_BG_VEL: i32 = 3;

pub const BG_ALPHA: u8 = 190;
pub const RECT_STAGE_ALPHA: u8 = 120;
pub const TEXT_STAGE_ALPHA: u8 = 255;

pub const BASE_Y: i32 = 75;

pub const TILE_TEXTURE: &str = "blue";

pub const BG1_TEXTURE: &str = "bg_ground";
pub const BG2_TEXTURE: &str = "bg_water";

pub const PLAYER_SPRITE: &str = "reimu0";
pub const REIMU_SPRITE: &str = "a_reimu0";
pub const MARISA_SPRITE: &str = "a_marisa0";
pub const ENEMY_SPRITE: &str = "enemy1";
pub const BULLET_SPRITE: &str = "spell0";

pub const BG_MUSIC: &str = "bgm";
pub const TITLE_MUSIC: &str = "title";

pub const SE_KILL: &str = "assets/audio/kill";
pub const SE_SHOT: &str = "assets/audio/graze";
pub const SE_SELECT: &str = "assets/audio/select";
pub const SE_SELECT2: &str = "assets/audio/select2";

pub const RE_FONT: &str = "assets/fonts/regular.ttf";
pub const IM_FONT: &str = "assets/fonts/immortal.ttf";
